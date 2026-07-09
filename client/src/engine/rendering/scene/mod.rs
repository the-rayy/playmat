mod instance;
mod mesh;
mod vertex;

pub struct Renderer {
  pipeline: wgpu::RenderPipeline,
}

impl Renderer {
  pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Scene render pipeline layout"),
      bind_group_layouts: &[],
      immediate_size: 0,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Scene shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("shaders/default.wgsl").into()),
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Scene render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[
          vertex::Vertex::buffer_layout(),
          instance::Instance::buffer_layout(),
        ],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: surface_format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      multiview_mask: None,
      cache: None,
    });

    Self { pipeline }
  }

  pub fn render(
    &self,
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    texture_view: &wgpu::TextureView,
    frame_no: u64,
  ) {
    let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None,
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color {
            r: 0.03,
            g: 0.03,
            b: 0.03,
            a: 1.0,
          }),
          store: wgpu::StoreOp::Store,
        },
        depth_slice: None,
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
      multiview_mask: None,
    });
    let mesh = mesh::Mesh::debug_cube(device);
    renderpass.set_pipeline(&self.pipeline);
    renderpass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

    let instance = instance::Instance::debug(device, frame_no);
    renderpass.set_vertex_buffer(1, instance.buffer.slice(..));

    renderpass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    renderpass.draw_indexed(0..mesh.index_count, 0, 0..1);
  }
}
