use crate::math::Rect;

#[derive(Debug)]
pub struct Button {
  pub rect: Rect,
  pub state: State,
}

impl Button {
  pub const fn new(rect: Rect) -> Self {
    Self { rect, state: State::Neutral }
  }

  pub fn color(&self) -> [f32; 4] {
    match self.state {
        State::Neutral => [1.0, 0.0, 0.0, 1.0],
        State::Hovered => [0.0, 1.0, 0.0, 1.0],
        State::Down => [0.0, 0.0, 1.0, 1.0],
    }
  }
}

#[derive(Debug)]
pub enum State {
  Neutral,
  Hovered,
  Down,
}
