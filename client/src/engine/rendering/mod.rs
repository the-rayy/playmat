use std::sync::Arc;

use egui_wgpu::RendererOptions;
use wgpu::{Color, ExperimentalFeatures};
use winit::dpi::PhysicalSize;

use crate::engine::gui::Renderable;

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

  pub fn render(&mut self, gui: Renderable) {
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

    self.render_3d(&mut encoder, &texture_view);
    self.render_egui(&mut encoder, &texture_view, gui);

    self.queue.submit([encoder.finish()]);
    self.window.pre_present_notify();
    output.present();
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
      multiview_mask: None,
    });
  }

  fn render_egui(
    &mut self,
    encoder: &mut wgpu::CommandEncoder,
    texture_view: &wgpu::TextureView,
    gui: Renderable,
  ) {
    let screen_descriptor = egui_wgpu::ScreenDescriptor {
      size_in_pixels: [
        self.window.inner_size().width,
        self.window.inner_size().height,
      ],
      pixels_per_point: self.window.scale_factor() as f32,
    };
    for (id, image_delta) in &gui.textures.set {
      self
        .egui_renderer
        .update_texture(&self.device, &self.queue, *id, image_delta);
    }
    self.egui_renderer.update_buffers(
      &self.device,
      &self.queue,
      encoder,
      &gui.primitives,
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
      multiview_mask: None,
    });
    let mut rpass = rpass.forget_lifetime();
    self
      .egui_renderer
      .render(&mut rpass, &gui.primitives, &screen_descriptor);
    drop(rpass);
    for x in &gui.textures.free {
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
