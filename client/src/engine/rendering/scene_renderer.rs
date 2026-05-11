// src/renderer/scene_renderer.rs

use super::{mesh::Mesh, vertex::Vertex};

pub struct SceneRenderer {
    pipeline:    wgpu::RenderPipeline,
    uniform_buf: wgpu::Buffer,
    bind_group:  wgpu::BindGroup,
    depth_view:  wgpu::TextureView,
}

impl SceneRenderer {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,

        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label:              Some("scene_uniforms"),
            size:               64,
            usage:              wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("scene_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty:                 wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("scene_bg"),
            layout:  &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: uniform_buf.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label:  Some("scene_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/scene.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label:                Some("scene_pl"),
                bind_group_layouts:   &[Some(&bgl)],
                immediate_size: 0,
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("scene_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module:      &shader,
                entry_point: Some("vs_main"),
                buffers:     &[Vertex::buffer_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module:      &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format:     surface_format,
                    blend:      Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology:  wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode:  Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format:              wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare:       Some(wgpu::CompareFunction::Less),
                stencil:             wgpu::StencilState::default(),
                bias:                wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            cache:       None,
            multiview_mask: None,
        });

        let depth_view = Self::create_depth_texture(device, size.width, size.height);

        Self { pipeline, uniform_buf, bind_group, depth_view }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.depth_view = Self::create_depth_texture(device, width, height);
    }

    pub fn update_mvp(&self, queue: &wgpu::Queue, mvp: &[f32; 16]) {
        queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(mvp));
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface_view: &wgpu::TextureView,
        mesh: &Mesh,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view:           surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load:  wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.08, g: 0.08, b: 0.10, a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load:  wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Discard,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..mesh.index_count, 0, 0..1);
    }

    fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32)
        -> wgpu::TextureView
    {
        device.create_texture(&wgpu::TextureDescriptor {
            label:  Some("depth_tex"),
            size:   wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count:    1,
            dimension:       wgpu::TextureDimension::D2,
            format:          wgpu::TextureFormat::Depth32Float,
            usage:           wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats:    &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
    }
}
