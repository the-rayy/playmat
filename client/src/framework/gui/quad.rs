use crate::{
  engine::rendering::texture::TextureKey,
  math::{Color, Rect},
};

#[derive(Debug)]
pub struct Quad {
  pub rect: Rect,
  pub uv: Rect,
  pub color: Color,
  pub texture_key: TextureKey,
}

impl Quad {
  pub const fn new(rect: Rect, uv: Rect, color: Color, texture_key: TextureKey) -> Self {
    Self {
      rect,
      uv,
      color,
      texture_key,
    }
  }

  pub fn color(&self) -> Color {
      self.color
    
  }

  pub fn texture_key(&self) -> TextureKey {
    self.texture_key.clone()
  }
}


