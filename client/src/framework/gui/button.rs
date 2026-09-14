use crate::engine::rendering::texture::TextureKey;

#[derive(Debug)]
pub struct Button {
  pub rect: math::Rect,
  pub color: math::Color,
  pub texture_key: TextureKey,
  pub state: State,
}

impl Button {
  pub const fn new(rect: math::Rect, color: math::Color, texture_key: TextureKey) -> Self {
    Self {
      rect,
      color,
      texture_key,
      state: State::Neutral,
    }
  }

  pub fn color(&self) -> math::Color {
    match self.state {
      State::Neutral => self.color,
      State::Hovered => self.color + math::Color::new(0.01, 0.01, 0.01, 0.0),
      State::Down => self.color + math::Color::new(0.05, 0.05, 0.05, 0.0),
    }
  }

  pub fn texture_key(&self) -> TextureKey {
    self.texture_key.clone()
  }
}

#[derive(Debug)]
pub enum State {
  Neutral,
  Hovered,
  Down,
}
