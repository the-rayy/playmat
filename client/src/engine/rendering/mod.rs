use std::sync::Arc;

use wgpu::{Color, ExperimentalFeatures};
use winit::dpi::PhysicalSize;

mod mesh;
mod vertex;
mod instance;

pub struct Renderer {
  window: Arc<winit::window::Window>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  size: winit::dpi::PhysicalSize<u32>,
  surface: wgpu::Surface<'static>,
  surface_format: wgpu::TextureFormat,

  pipeline: wgpu::RenderPipeline,
}

impl Renderer {
  pub async fn new(window: Arc<winit::window::Window>) -> Self {
    let instance = wgpu::Instance::new(super::platform::wgpu::instance_descriptor());
    let surface = instance
      .create_surface(window.clone())
      .expect("Could not create surface");
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .expect("Unable to request adapter");
    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: super::platform::wgpu::device_limits(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
        experimental_features: ExperimentalFeatures::default(),
      })
      .await
      .expect("Unable to request device");

    let size = window.inner_size();
    let surface_format = *surface
      .get_capabilities(&adapter)
      .formats
      .first()
      .expect("no available surface formats");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("shaders/example.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      immediate_size: 0,
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[vertex::Vertex::buffer_layout(), instance::Instance::buffer_layout()],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: surface_format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList, 
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
      },
      depth_stencil: None, 
      multisample: wgpu::MultisampleState {
        count: 1,          
        mask: !0,         
        alpha_to_coverage_enabled: false, 
      },
      multiview_mask: None,
      cache: None,        
    });

    let state = Self {
      window,
      device,
      queue,
      size,
      surface,
      surface_format,

      pipeline: render_pipeline,
    };

    state.configure_surface();

    state
  }

  #[expect(clippy::panic, reason = "device lost")]
  pub fn render(&self, frame_no: u64) {
    let output = match self.surface.get_current_texture() {
      wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
      wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
        self.configure_surface();
        surface_texture
      }
      wgpu::CurrentSurfaceTexture::Timeout
      | wgpu::CurrentSurfaceTexture::Occluded
      | wgpu::CurrentSurfaceTexture::Validation => return,
      wgpu::CurrentSurfaceTexture::Outdated => {
        self.configure_surface();
        return;
      }
      wgpu::CurrentSurfaceTexture::Lost => {
        // You could recreate the devices and all resources
        // created with it here, but we'll just bail

        panic!("Lost device");
      }
    };
    let texture_view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&Default::default());

    let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None,
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(Color::BLUE),
          store: wgpu::StoreOp::Store,
        },
        depth_slice: None,
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
      multiview_mask: None,
    });
    let mesh = mesh::Mesh::debug_cube(&self.device);
    renderpass.set_pipeline(&self.pipeline);
    renderpass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

    let instance = instance::Instance::debug(&self.device, frame_no);
    renderpass.set_vertex_buffer(1, instance.buffer.slice(..));

    renderpass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16); 
    renderpass.draw_indexed(0..mesh.index_count, 0, 0..1); 

    drop(renderpass);

    self.queue.submit([encoder.finish()]);
    self.window.pre_present_notify();
    output.present();
  }

  pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
    self.size = new_size;
    self.configure_surface();
  }

  fn configure_surface(&self) {
    if self.window.inner_size().width == 0 || self.window.inner_size().height == 0 {
      return;
    }
    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: self.surface_format,
      view_formats: vec![self.surface_format.add_srgb_suffix()],
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      width: self.window.inner_size().width,
      height: self.window.inner_size().height,
      desired_maximum_frame_latency: 2,
      present_mode: wgpu::PresentMode::AutoVsync,
    };
    self.surface.configure(&self.device, &surface_config);
  }
}
