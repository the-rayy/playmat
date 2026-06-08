use crate::math::Rect;

pub struct Button {
  pub rect: Rect,
}

impl Button {
  pub const fn new(rect: Rect) -> Self {
    Self { rect }
  }
}
