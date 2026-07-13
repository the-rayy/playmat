use crate::{
  engine::rendering::canvas::vertex::Vertex,
  math::{Color, Point, Rect},
};

#[derive(Default)]
pub struct DrawList {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
}

impl DrawList {
  pub const fn is_empty(&self) -> bool {
    self.indices.is_empty()
  }

  pub fn push_rect(&mut self, rect: &Rect, color: Color) {
    let base = self.vertices.len() as u16;

    self.vertices.extend_from_slice(&[
      Vertex {
        position: rect.top_left(),
        color,
        uv: Point::ZERO,
      },
      Vertex {
        position: rect.top_right(),
        color,
        uv: Point::ZERO,
      },
      Vertex {
        position: rect.bottom_right(),
        color,
        uv: Point::ZERO
      },
      Vertex {
        position: rect.bottom_left(),
        color,
        uv: Point::ZERO
      },
    ]);

    self
      .indices
      .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
  }
}
