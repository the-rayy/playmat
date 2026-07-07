use crate::math::{Color, Rect};

#[derive(Debug)]
pub struct Button {
  pub rect: Rect,
  pub color: Color,
  pub state: State,
}

impl Button {
  pub fn new(rect: Rect) -> Self {
    Self {
      rect,
      color: Color::new(0.5, 0.0, 0.0, 1.0),
      state: State::Neutral,
    }
  }

  pub fn color(&self) -> Color {
    match self.state {
      State::Neutral => self.color.clone(),
      State::Hovered => self.color.clone() + Color::new(0.01, 0.01, 0.01, 0.0),
      State::Down => self.color.clone() + Color::new(0.05, 0.05, 0.05, 0.0),
    }
  }
}

#[derive(Debug)]
pub enum State {
  Neutral,
  Hovered,
  Down,
}
