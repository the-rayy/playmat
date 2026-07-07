use crate::math::{Color, Rect};

#[derive(Debug)]
pub struct Button {
  pub rect: Rect,
  pub state: State,
}

impl Button {
  pub const fn new(rect: Rect) -> Self {
    Self {
      rect,
      state: State::Neutral,
    }
  }

  pub fn color(&self) -> Color {
    match self.state {
      State::Neutral => Color::new(1.0, 0.0, 0.0, 1.0),
      State::Hovered => Color::new(0.0, 1.0, 0.0, 1.0),
      State::Down => Color::new(0.0, 0.0, 1.0, 1.0),
    }
  }
}

#[derive(Debug)]
pub enum State {
  Neutral,
  Hovered,
  Down,
}
