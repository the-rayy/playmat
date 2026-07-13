use std::sync::Arc;

use wgpu::ExperimentalFeatures;

use crate::engine::rendering::texture::Texture;

pub mod canvas;
mod scene;
pub mod texture;

pub struct Renderer {
  window: Arc<winit::window::Window>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface: wgpu::Surface<'static>,
  surface_format: wgpu::TextureFormat,

  renderer_scene: scene::Renderer,
  renderer_canvas: canvas::Renderer,
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
        apply_limit_buckets: true,
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

    let surface_format = *surface
      .get_capabilities(&adapter)
      .formats
      .first()
      .expect("no available surface formats");

    let renderer_scene = scene::Renderer::new(&device, surface_format);
    let renderer_canvas = canvas::Renderer::new(&device, surface_format);

    let state = Self {
      window,
      device,
      queue,
      surface,
      surface_format,
      renderer_scene,
      renderer_canvas,
    };

    state.configure_surface();

    let debug_texture = Texture::new();
    state.load_texture(&debug_texture);

    state
  }

  pub fn load_texture(&self, texture: &Texture) {
    let desc = texture.descriptor();
    let size = texture.size();
    let tex = self.device.create_texture(&desc);

    self.queue.write_texture(
      wgpu::TexelCopyTextureInfo {
        texture: &tex,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      texture.data(),
      wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4*size.width),
        rows_per_image: Some(1*size.height),
      },
      size,
    );
  }

  #[expect(clippy::panic, reason = "device lost")]
  pub fn render(&self, draw_list: &canvas::draw_list::DrawList, frame_no: u64) {
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

    self
      .renderer_scene
      .render(&self.device, &mut encoder, &texture_view, frame_no);
    self
      .renderer_canvas
      .render(&self.queue, &mut encoder, &texture_view, draw_list);

    self.queue.submit([encoder.finish()]);
    self.window.pre_present_notify();
    self.queue.present(output);
  }

  pub fn resize(&self) {
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
      color_space: wgpu::SurfaceColorSpace::Auto,
    };
    self.surface.configure(&self.device, &surface_config);
  }

  pub fn get_screen_size(&self) -> (u32, u32) {
    (
      self.window.inner_size().width,
      self.window.inner_size().height,
    )
  }
}
