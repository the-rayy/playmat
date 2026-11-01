use crate::{
  gui,
  math::{Color, Rect},
  rendering::texture::TextureKey,
};

#[derive(Debug, Clone)]
pub struct Quad {
  pub rect: Rect,
  pub uv: Rect,
  pub color: Color,
  pub texture_key: TextureKey,

  pub layer: u8,
}

impl Quad {
  pub const fn new(rect: Rect, uv: Rect, color: Color, texture_key: TextureKey, layer: u8) -> Self {
    Self {
      rect,
      uv,
      color,
      texture_key,
      layer,
    }
  }
}

impl gui::Widget for Quad {
  fn quads(&self) -> Vec<Quad> {
    vec![self.clone()]
  }

  fn rect(&self) -> Rect {
    self.rect
  }
}
