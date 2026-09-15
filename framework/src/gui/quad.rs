use engine::rendering::texture::TextureKey;

#[derive(Debug)]
pub struct Quad {
  pub rect: math::Rect,
  pub uv: math::Rect,
  pub color: math::Color,
  pub texture_key: TextureKey,
}

impl Quad {
  pub const fn new(
    rect: math::Rect,
    uv: math::Rect,
    color: math::Color,
    texture_key: TextureKey,
  ) -> Self {
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

  pub fn texture_key(&self) -> TextureKey {
    self.texture_key.clone()
  }
}
