#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}
