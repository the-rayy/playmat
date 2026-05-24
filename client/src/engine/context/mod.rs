use crate::engine::rendering::canvas::vertex::Vertex;

#[derive(Default)]
pub struct Context {
  pub gui_draw_list: DrawList,
}

#[derive(Default)]
pub struct DrawList{
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>
}

impl DrawList {
  pub fn clear(&mut self) {
    self.vertices.clear();
    self.indices.clear();
  }

  pub fn is_empty(&self) -> bool {
    self.indices.is_empty()
  }

  pub fn debug_push(&mut self) {
    self.vertices = vec![
      Vertex {
        position: [-0.5, 0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      Vertex {
        position: [0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      Vertex {
        position: [-0.5, 0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      Vertex {
        position: [0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      Vertex {
        position: [0.5, 0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
    ];

    self.indices = vec![0, 1, 2, 3, 4, 5];
  }
}

