use crate::{
  engine::rendering::{canvas::vertex::Vertex, texture::TextureKey},
  math::{Color, Point, Rect},
};

#[derive(Default)]
pub struct DrawList {
  pub primitives: Vec<Primitive>,
}

pub struct Primitive {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
  pub texture_key: TextureKey,
}

pub struct FlatDrawList {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
  pub draws: Vec<DrawCall>,
}

pub struct DrawCall {
  pub texture_key: TextureKey,
  pub index_start: u32,
  pub index_count: u32,
}

impl DrawList {
  pub const fn is_empty(&self) -> bool {
    self.primitives.is_empty()
  }

  pub fn push_rect(&mut self, rect: &Rect, color: Color, texture_key: TextureKey) {
    let vertices = Vec::from([
      Vertex {
        position: rect.top_left(),
        color,
        uv: Point::ZERO,
      },
      Vertex {
        position: rect.top_right(),
        color,
        uv: Point { x: 1.0, y: 0.0 },
      },
      Vertex {
        position: rect.bottom_right(),
        color,
        uv: Point::UNIT,
      },
      Vertex {
        position: rect.bottom_left(),
        color,
        uv: Point { x: 0.0, y: 1.0 },
      },
    ]);

    let indices = Vec::from([0, 1, 2, 0, 2, 3]);

    let primitive = Primitive {
      vertices,
      indices,
      texture_key,
    };

    self.primitives.push(primitive);
  }

  pub fn flatten(&self) -> FlatDrawList {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut draws = Vec::new();

    for primitive in &self.primitives {
      let base_vertex = vertices.len() as u16;
      let index_start = indices.len() as u32;

      vertices.extend_from_slice(&primitive.vertices);
      indices.extend(primitive.indices.iter().map(|i| i + base_vertex));

      draws.push(DrawCall {
        texture_key: primitive.texture_key.clone(),
        index_start,
        index_count: primitive.indices.len() as u32,
      });
    }

    FlatDrawList {
      vertices,
      indices,
      draws,
    }
  }
}
