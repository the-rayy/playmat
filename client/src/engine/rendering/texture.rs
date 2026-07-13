pub struct Texture {
  data: [u8; 2 * 2 * 4],
}

impl Texture {
  pub fn new() -> Self {
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
}
