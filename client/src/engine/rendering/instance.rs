use wgpu::util::DeviceExt;

pub struct Instance {
  pub buffer: wgpu::Buffer,
}

impl Instance {
  pub fn new(device: &wgpu::Device, mtx: &Mat4) -> Self {
    Self {
      buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("instance vertex"),
        contents: bytemuck::cast_slice(mtx),
        usage: wgpu::BufferUsages::VERTEX,
      }),
    }
  }

  pub fn debug(device: &wgpu::Device, frame_no: u64) -> Self {
    let base = identity();
    let t = frame_no as f32 * 0.005;

    // Different speeds on each axis
    let rx = rotation_x(t * 0.7);
    let ry = rotation_y(t * 1.1);
    let rz = rotation_z(t * 0.4);

    // Combine rotations
    let rotation = multiply(multiply(rx, ry), rz);
    let mtx = multiply(base, rotation);

    Self::new(device, &mtx)
  }

  pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Mat4>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 5,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
          shader_location: 6,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
          shader_location: 7,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
          shader_location: 8,
          format: wgpu::VertexFormat::Float32x4,
        },
      ],
    }
  }
}

type Mat4 = [[f32; 4]; 4];

fn identity() -> Mat4 {
  [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]
}

fn multiply(a: Mat4, b: Mat4) -> Mat4 {
  let mut result = [[0.0; 4]; 4];

  for i in 0..4 {
    for j in 0..4 {
      for k in 0..4 {
        result[i][j] += a[i][k] * b[k][j];
      }
    }
  }

  result
}

fn rotation_x(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();

  [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, c, -s, 0.0],
    [0.0, s, c, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]
}

fn rotation_y(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();

  [
    [c, 0.0, s, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [-s, 0.0, c, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]
}

fn rotation_z(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();

  [
    [c, -s, 0.0, 0.0],
    [s, c, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
  ]
}
