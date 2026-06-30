use crate::math::Point;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

impl Rect {
  pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
    Self { x, y, w, h }
  }

  pub const fn top_left(&self) -> Point {
    Point {
      x: self.x,
      y: self.y,
    }
  }

  pub fn top_right(&self) -> Point {
    Point {
      x: self.x + self.w,
      y: self.y,
    }
  }

  pub fn bottom_left(&self) -> Point {
    Point {
      x: self.x,
      y: self.y + self.h,
    }
  }

  pub fn bottom_right(&self) -> Point {
    Point {
      x: self.x + self.w,
      y: self.y + self.h,
    }
  }
}
