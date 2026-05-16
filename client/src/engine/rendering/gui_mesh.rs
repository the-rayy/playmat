use wgpu::util::DeviceExt;

use crate::engine::rendering::gui_vertex::GuiVertex;

pub struct GuiMesh {
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub index_count: u32,
}

impl GuiMesh {
  pub fn new(device: &wgpu::Device, vertices: &[GuiVertex], indices: &[u16]) -> Self {
    Self {
      vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("gui vertex"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
      }),
      index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("gui index"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
      }),
      index_count: indices.len() as u32,
    }
  }

  pub fn debug_quad(device: &wgpu::Device) -> Self {
    let vertices = vec![
      GuiVertex {
        position: [-0.5, 0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      GuiVertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      GuiVertex {
        position: [0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      GuiVertex {
        position: [-0.5, 0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      GuiVertex {
        position: [0.5, -0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
      GuiVertex {
        position: [0.5, 0.5],
        color: [1.0, 0.0, 0.0, 1.0],
      },
    ];

    let indices = vec![0, 1, 2, 3, 4, 5];

    Self::new(device, &vertices, &indices)
  }
}
