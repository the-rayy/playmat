use crate::rendering::texture::Id;

#[derive(Debug)]
pub struct Quad {
  pub rect: math::Rect,
  pub uv: math::Rect,
  pub color: math::Color,
  pub texture_key: Id,
}

impl Quad {
  pub const fn new(rect: math::Rect, uv: math::Rect, color: math::Color, texture_key: Id) -> Self {
    Self {
      rect,
      uv,
      color,
      texture_key,
    }
  }

  pub const fn color(&self) -> math::Color {
    self.color
  }

  pub const fn texture_key(&self) -> Id {
    self.texture_key
  }
}
