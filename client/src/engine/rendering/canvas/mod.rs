mod mesh;
mod vertex;

pub struct Renderer {
  pipeline: wgpu::RenderPipeline,
}

impl Renderer {
  pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Canvas shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("shaders/default.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Canvas Render pipeline layout"),
      bind_group_layouts: &[],
      immediate_size: 0,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Canvas render pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[
          vertex::Vertex::buffer_layout(),
        ],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: surface_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: None,
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

    Self {pipeline}

  }

  pub fn render(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, texture_view: &wgpu::TextureView) {
    let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None,
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &texture_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Load,
          store: wgpu::StoreOp::Store,
        },
        depth_slice: None,
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
      multiview_mask: None,
    });
    let mesh = mesh::Mesh::debug_quad(device);
    renderpass.set_pipeline(&self.pipeline);
    renderpass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

    renderpass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    renderpass.draw_indexed(0..mesh.index_count, 0, 0..1);
  }
}
