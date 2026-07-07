#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self {
      r: clamp01(r),
      g: clamp01(g), 
      b: clamp01(b), 
      a: clamp01(a)
    }
  }
}

fn clamp01(x: f32) -> f32 {
  clamp(0.0, x, 1.0)
}

fn clamp(min: f32, x: f32, max: f32) -> f32 {
  if x < min { min } else if x > max { max } else { x }
}
