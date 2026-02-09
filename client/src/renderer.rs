use std::sync::Arc;

use egui::{ClippedPrimitive, TexturesDelta};
use egui_wgpu::RendererOptions;
use wgpu::{Color, ExperimentalFeatures};
use winit::dpi::PhysicalSize;

pub struct Renderer {
  window: Arc<winit::window::Window>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  size: winit::dpi::PhysicalSize<u32>,
  surface: wgpu::Surface<'static>,
  surface_format: wgpu::TextureFormat,
  egui_renderer: egui_wgpu::Renderer,
}

impl Renderer {
  pub async fn new(window: Arc<winit::window::Window>) -> Self {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      #[cfg(not(target_arch = "wasm32"))]
      backends: wgpu::Backends::PRIMARY,
      #[cfg(target_arch = "wasm32")]
      backends: wgpu::Backends::GL,
      ..Default::default()
    });
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
        required_limits: if cfg!(target_arch = "wasm32") {
          wgpu::Limits::downlevel_webgl2_defaults()
        } else {
          wgpu::Limits::default()
        },
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
        experimental_features: ExperimentalFeatures::default(),
      })
      .await
      .expect("Unable to request device");

    let size = window.inner_size();
    let surface_format = surface.get_capabilities(&adapter).formats[0];

    let egui_renderer =
      egui_wgpu::Renderer::new(&device, surface_format, RendererOptions::default());

    let state = Self {
      window,
      device,
      queue,
      size,
      surface,
      surface_format,
      egui_renderer,
    };

    state.configure_surface();

    state
  }

  pub fn render(&mut self, gui_primitives: Vec<ClippedPrimitive>, gui_textures: TexturesDelta) {
    let surface_texture = self
      .surface
      .get_current_texture()
      .expect("Unable to acquire next surface texture");
    let texture_view = surface_texture
      .texture
      .create_view(&wgpu::TextureViewDescriptor {
        format: Some(self.surface_format.add_srgb_suffix()),
        ..Default::default()
      });

    let mut encoder = self.device.create_command_encoder(&Default::default());

    self.render_3d(&mut encoder, &texture_view);
    self.render_egui(&mut encoder, &texture_view, gui_primitives, gui_textures);

    self.queue.submit([encoder.finish()]);
    self.window.pre_present_notify();
    surface_texture.present();
  }

  pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
    self.size = new_size;
    self.configure_surface();
  }

  fn render_3d(&self, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView) {
    let _renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None,
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
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
    });
  }

  fn render_egui(
    &mut self,
    encoder: &mut wgpu::CommandEncoder,
    texture_view: &wgpu::TextureView,
    primitives: Vec<ClippedPrimitive>,
    textures: TexturesDelta,
  ) {
    let screen_descriptor = egui_wgpu::ScreenDescriptor {
      size_in_pixels: [
        self.window.inner_size().width,
        self.window.inner_size().height,
      ],
      pixels_per_point: self.window.scale_factor() as f32,
    };
    for (id, image_delta) in &textures.set {
      self
        .egui_renderer
        .update_texture(&self.device, &self.queue, *id, image_delta);
    }
    self.egui_renderer.update_buffers(
      &self.device,
      &self.queue,
      encoder,
      &primitives,
      &screen_descriptor,
    );
    let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Load,
          store: wgpu::StoreOp::Store,
        },
        depth_slice: None,
      })],
      depth_stencil_attachment: None,
      label: Some("egui main render pass"),
      timestamp_writes: None,
      occlusion_query_set: None,
    });
    let mut rpass = rpass.forget_lifetime();
    self
      .egui_renderer
      .render(&mut rpass, &primitives, &screen_descriptor);
    drop(rpass);
    for x in &textures.free {
      self.egui_renderer.free_texture(x)
    }
  }

  fn configure_surface(&self) {
    if self.window.inner_size().width + self.window.inner_size().height == 0 {
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
