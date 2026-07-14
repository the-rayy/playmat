use crate::{
  engine::rendering::texture::TextureKey,
  math::{Color, Rect},
};

#[derive(Debug)]
pub struct Button {
  pub rect: Rect,
  pub color: Color,
  pub texture_key: TextureKey,
  pub state: State,
}

impl Button {
  pub const fn new(rect: Rect, color: Color, texture_key: TextureKey) -> Self {
    Self {
      rect,
      color,
      texture_key,
      state: State::Neutral,
    }
  }

  pub fn color(&self) -> Color {
    match self.state {
      State::Neutral => self.color,
      State::Hovered => self.color + Color::new(0.01, 0.01, 0.01, 0.0),
      State::Down => self.color + Color::new(0.05, 0.05, 0.05, 0.0),
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
