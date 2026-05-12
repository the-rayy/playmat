use wgpu::util::DeviceExt;

use crate::engine::rendering::vertex::Vertex;

pub struct Mesh {
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub vertex_count: u32,
  pub index_count: u32,
}

impl Mesh {
  pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
    Self {
      vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("cube vertex"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
      }),
      index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("cube indices"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
      }),
      vertex_count: vertices.len() as u32,
      index_count: indices.len() as u32,
    }
  }

  pub fn debug_triangle(device: &wgpu::Device) -> Self {
    let vertices = vec![
      Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
      },
      Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
      },
      Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
      },
    ];
    let indices = vec![];

    Self::new(device, &vertices, &indices)
  }

  pub fn debug_cube(device: &wgpu::Device) -> Self {
    let vertices = vec![
      // front  (z+)  red
      Vertex {
        position: [-0.5, -0.5, 0.5],
        color: [0.9, 0.2, 0.2],
      },
      Vertex {
        position: [0.5, -0.5, 0.5],
        color: [0.9, 0.2, 0.2],
      },
      Vertex {
        position: [0.5, 0.5, 0.5],
        color: [0.9, 0.2, 0.2],
      },
      Vertex {
        position: [-0.5, 0.5, 0.5],
        color: [0.9, 0.2, 0.2],
      },
      // back   (z-)  blue
      Vertex {
        position: [0.5, -0.5, -0.5],
        color: [0.2, 0.2, 0.9],
      },
      Vertex {
        position: [-0.5, -0.5, -0.5],
        color: [0.2, 0.2, 0.9],
      },
      Vertex {
        position: [-0.5, 0.5, -0.5],
        color: [0.2, 0.2, 0.9],
      },
      Vertex {
        position: [0.5, 0.5, -0.5],
        color: [0.2, 0.2, 0.9],
      },
      // top    (y+)  green
      Vertex {
        position: [-0.5, 0.5, 0.5],
        color: [0.2, 0.8, 0.3],
      },
      Vertex {
        position: [0.5, 0.5, 0.5],
        color: [0.2, 0.8, 0.3],
      },
      Vertex {
        position: [0.5, 0.5, -0.5],
        color: [0.2, 0.8, 0.3],
      },
      Vertex {
        position: [-0.5, 0.5, -0.5],
        color: [0.2, 0.8, 0.3],
      },
      // bottom (y-)  yellow
      Vertex {
        position: [-0.5, -0.5, -0.5],
        color: [0.9, 0.8, 0.1],
      },
      Vertex {
        position: [0.5, -0.5, -0.5],
        color: [0.9, 0.8, 0.1],
      },
      Vertex {
        position: [0.5, -0.5, 0.5],
        color: [0.9, 0.8, 0.1],
      },
      Vertex {
        position: [-0.5, -0.5, 0.5],
        color: [0.9, 0.8, 0.1],
      },
      // right  (x+)  magenta
      Vertex {
        position: [0.5, -0.5, 0.5],
        color: [0.8, 0.2, 0.8],
      },
      Vertex {
        position: [0.5, -0.5, -0.5],
        color: [0.8, 0.2, 0.8],
      },
      Vertex {
        position: [0.5, 0.5, -0.5],
        color: [0.8, 0.2, 0.8],
      },
      Vertex {
        position: [0.5, 0.5, 0.5],
        color: [0.8, 0.2, 0.8],
      },
      // left   (x-)  cyan
      Vertex {
        position: [-0.5, -0.5, -0.5],
        color: [0.1, 0.8, 0.8],
      },
      Vertex {
        position: [-0.5, -0.5, 0.5],
        color: [0.1, 0.8, 0.8],
      },
      Vertex {
        position: [-0.5, 0.5, 0.5],
        color: [0.1, 0.8, 0.8],
      },
      Vertex {
        position: [-0.5, 0.5, -0.5],
        color: [0.1, 0.8, 0.8],
      },
    ];
    let indices = vec![
      0, 1, 2, 0, 2, 3, // front
      4, 5, 6, 4, 6, 7, // back
      8, 9, 10, 8, 10, 11, // top
      12, 13, 14, 12, 14, 15, // bottom
      16, 17, 18, 16, 18, 19, // right
      20, 21, 22, 20, 22, 23, // left
    ];

    Self::new(device, &vertices, &indices)
  }
}
