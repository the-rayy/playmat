use crate::{
  gui, math::{Color, Rect}, rendering::texture::TextureKey,
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
}

impl gui::Widget for Quad {
    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn uv(&self) -> &Rect {
        &self.uv
    }

    fn color(&self) -> Color {
        self.color
    }

    fn texture_key(&self) -> TextureKey {
        self.texture_key.clone()
    }
}
