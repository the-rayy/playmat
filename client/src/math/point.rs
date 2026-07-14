#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
  pub const UNIT: Self = Self { x: 1.0, y: 1.0 };
}
