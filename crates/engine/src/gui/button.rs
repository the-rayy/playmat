use crate::{
  gui::{self, Quad},
  math::{Color, Rect},
  rendering::texture::TextureKey,
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
}

impl gui::Widget for Button {
  fn quads(&self) -> Vec<gui::Quad> {
    let quad = Quad::new(
      self.rect,
      Rect::unit(),
      self.color(),
      self.texture_key.clone(),
    );
    vec![quad]
  }

  fn rect(&self) -> Rect {
    self.rect
  }

  fn on_event(&mut self, event: &gui::InternalEvent) {
    self.state = match event {
      gui::InternalEvent::Click => State::Down,
      gui::InternalEvent::Hover => State::Hovered,
      gui::InternalEvent::Clear => State::Neutral,
    }
  }
}

#[derive(Debug)]
pub enum State {
  Neutral,
  Hovered,
  Down,
}
