use std::io::Cursor;

use image::ImageReader;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Default for Id {
  fn default() -> Self {
    Self::new()
  }
}

impl Id {
  pub fn new() -> Self {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    Self(NEXT.fetch_add(1, Ordering::Relaxed))
  }

  pub const fn get(self) -> u64 {
    self.0
  }
}

pub struct Texture {
  id: Id,
  pixels: Vec<u8>,
  height: u32,
  width: u32,
}

impl Texture {
  pub fn new(pixels: Vec<u8>, height: u32, width: u32) -> Self {
    Self {
      id: Id::new(),
      pixels,
      height,
      width,
    }
  }

  pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
    let img = ImageReader::new(Cursor::new(data))
      .with_guessed_format()
      .map_err(|e| e.to_string())?
      .decode()
      .map_err(|e| e.to_string())?
      .into_rgba8();

    let (width, height) = img.dimensions();
    let pixels = img.into_raw();
    Ok(Self::new(pixels, height, width))
  }

  pub const fn id(&self) -> &Id {
    &self.id
  }

  pub fn pixels(&self) -> &[u8] {
    &self.pixels
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

  pub const fn size(&self) -> wgpu::Extent3d {
    wgpu::Extent3d {
      width: self.width,
      height: self.height,
      depth_or_array_layers: 1,
    }
  }

  pub fn sampler_descriptor(&self) -> wgpu::SamplerDescriptor<'static> {
    wgpu::SamplerDescriptor {
      label: "foo".into(),
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
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
