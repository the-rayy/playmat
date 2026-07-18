use std::{collections::HashMap, sync::Arc};

use wgpu::ExperimentalFeatures;

use crate::engine::rendering::texture::{Texture, TextureKey};

pub mod canvas;
mod scene;
pub mod texture;

pub struct Renderer {
  window: Arc<winit::window::Window>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface: wgpu::Surface<'static>,
  surface_format: wgpu::TextureFormat,
  textures_bind_group_layout: wgpu::BindGroupLayout,

  textures: HashMap<TextureKey, wgpu::BindGroup>,

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

    let textures_bind_group_layout =
      device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
              multisampled: false,
              view_dimension: wgpu::TextureViewDimension::D2,
              sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
          },
        ],
        label: Some("texture_bind_group_layout"),
      });

    let default_texture = load_texture(
      &device,
      &queue,
      &Texture::new(vec![255, 255, 255, 255], 1, 1),
      &textures_bind_group_layout,
    );

    let renderer_scene = scene::Renderer::new(&device, surface_format);
    let renderer_canvas = canvas::Renderer::new(
      &device,
      surface_format,
      textures_bind_group_layout.clone(),
      default_texture,
    );

    let state = Self {
      window,
      device,
      queue,
      surface,
      surface_format,
      textures_bind_group_layout,
      textures: HashMap::default(),
      renderer_scene,
      renderer_canvas,
    };

    state.configure_surface();

    state
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
    self.renderer_canvas.render(
      &self.queue,
      &mut encoder,
      &texture_view,
      draw_list,
      &self.textures,
    );

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

  pub fn load_texture(&mut self, key: TextureKey, texture: &Texture) {
    let bind_group = load_texture(
      &self.device,
      &self.queue,
      texture,
      &self.textures_bind_group_layout,
    );
    self.textures.insert(key, bind_group);
  }
}

pub fn load_texture(
  device: &wgpu::Device,
  queue: &wgpu::Queue,
  texture: &Texture,
  bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
  let desc = texture.descriptor();
  let size = texture.size();
  let sampler_desc = texture.sampler_descriptor();
  let tex = device.create_texture(&desc);

  queue.write_texture(
    wgpu::TexelCopyTextureInfo {
      texture: &tex,
      mip_level: 0,
      origin: wgpu::Origin3d::ZERO,
      aspect: wgpu::TextureAspect::All,
    },
    texture.pixels(),
    wgpu::TexelCopyBufferLayout {
      offset: 0,
      bytes_per_row: Some(4 * size.width),
      rows_per_image: Some(size.height),
    },
    size,
  );

  let sampler = device.create_sampler(&sampler_desc);
  let texture_view = tex.create_view(&wgpu::TextureViewDescriptor::default());
  device.create_bind_group(&wgpu::BindGroupDescriptor {
    layout: bind_group_layout,
    entries: &[
      wgpu::BindGroupEntry {
        binding: 0,
        resource: wgpu::BindingResource::TextureView(&texture_view),
      },
      wgpu::BindGroupEntry {
        binding: 1,
        resource: wgpu::BindingResource::Sampler(&sampler),
      },
    ],
    label: Some("diffuse_bind_group"),
  })
}
