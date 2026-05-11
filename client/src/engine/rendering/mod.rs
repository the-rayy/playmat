use std::sync::Arc;

use wgpu::{Color, ExperimentalFeatures};
use winit::dpi::PhysicalSize;

use crate::engine::rendering::{mesh::Mesh, scene_renderer::SceneRenderer};

mod vertex;
pub mod mesh;
mod scene_renderer;
pub mod math;

pub struct Renderer {
  window: Arc<winit::window::Window>,
  pub device: wgpu::Device,
  queue: wgpu::Queue,
  size: winit::dpi::PhysicalSize<u32>,
  surface: wgpu::Surface<'static>,
  surface_format: wgpu::TextureFormat,
  scene_renderer: SceneRenderer,
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

    let scene_renderer = SceneRenderer::new(&device, surface_format, size);

    let state = Self {
      window,
      device,
      queue,
      size,
      surface,
      surface_format,

      scene_renderer,
    };

    state.configure_surface();

    state
  }

  #[expect(clippy::panic, reason = "device lost")]
  pub fn render(&self, mvp: math::Mat4, mesh: &Mesh) {
    self.scene_renderer.update_mvp(&self.queue, &mvp);
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
    self.scene_renderer.draw(&mut encoder, &texture_view, mesh);

    self.queue.submit([encoder.finish()]);
    self.window.pre_present_notify();
    output.present();
  }

  pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
    self.size = new_size;
    self.configure_surface();
    self.scene_renderer.resize(&self.device, new_size.width, new_size.height);
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
