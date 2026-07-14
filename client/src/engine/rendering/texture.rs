#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct TextureKey(pub String);

pub struct Texture {
  data: [u8; 2 * 2 * 4],
}

impl Texture {
  pub fn new_white() -> Self {
    Self {
      data: [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
      ],
    }
  }

  pub fn new_checkerboard() -> Self {
    Self {
      data: [
        255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
      ],
    }
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }

  pub fn descriptor(&self) -> wgpu::TextureDescriptor<'static> {
    wgpu::TextureDescriptor {
      label: "foo".into(),
      size: self.size(),
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
      view_formats: &[],
    }
  }

  pub fn size(&self) -> wgpu::Extent3d {
    wgpu::Extent3d {
      width: 2,
      height: 2,
      depth_or_array_layers: 1,
    }
  }

  pub fn sampler_descriptor(&self) -> wgpu::SamplerDescriptor<'static> {
    wgpu::SamplerDescriptor {
      label: "foo".into(),
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::MipmapFilterMode::Nearest,
      lod_min_clamp: 0.0,
      lod_max_clamp: 32.0,
      compare: None,
      anisotropy_clamp: 1,
      border_color: None,
    }
  }
}
