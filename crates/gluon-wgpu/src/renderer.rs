use bytemuck::{Pod, Zeroable, bytes_of, cast_slice};
use gluon_core::DisplayList;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
  BindGroup, BindGroupDescriptor, BindGroupEntry, BlendState, Buffer, BufferUsages,
  ColorTargetState, ColorWrites, CommandEncoder, Device, FragmentState, LoadOp, MultisampleState,
  Operations, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor,
  RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StoreOp,
  TextureFormat, TextureView, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState,
  VertexStepMode,
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RectInstance {
  rect: [f32; 4],
  color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct ViewportUniform {
  size: [f32; 2],
  padding: [f32; 2],
}

pub struct Renderer {
  device: Device,
  pipeline: RenderPipeline,
  rect_buffer: Buffer,
  rect_capacity: usize,
  viewport_buffer: Buffer,
  viewport_bind_group: BindGroup,
}

impl Renderer {
  pub fn new(device: &Device, target_format: TextureFormat) -> Self {
    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
      label: Some("gluon_shader_module"),
      source: ShaderSource::Wgsl(include_str!("rect.wgsl").into()),
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
      label: Some("gluon_pipeline"),
      layout: None,
      vertex: VertexState {
        module: &shader_module,
        entry_point: Some("vertex_main"),
        compilation_options: Default::default(),
        buffers: &[Some(VertexBufferLayout {
          array_stride: size_of::<RectInstance>() as u64,
          step_mode: VertexStepMode::Instance,
          attributes: &[
            VertexAttribute {
              format: VertexFormat::Float32x4,
              offset: 0,
              shader_location: 0,
            },
            VertexAttribute {
              format: VertexFormat::Float32x4,
              offset: 16,
              shader_location: 1,
            },
          ],
        })],
      },
      primitive: PrimitiveState::default(),
      depth_stencil: None,
      multisample: MultisampleState::default(),
      fragment: Some(FragmentState {
        module: &shader_module,
        entry_point: Some("fragment_main"),
        compilation_options: Default::default(),
        targets: &[Some(ColorTargetState {
          format: target_format,
          blend: Some(BlendState::ALPHA_BLENDING),
          write_mask: ColorWrites::ALL,
        })],
      }),
      multiview_mask: None,
      cache: None,
    });

    let rect_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("gluon_rect_buffer"),
      contents: bytes_of(&RectInstance::zeroed()),
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    });

    let viewport_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("gluon_viewport_buffer"),
      contents: bytes_of(&ViewportUniform::zeroed()),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let viewport_bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some("gluon_viewport_bind_group"),
      layout: &pipeline.get_bind_group_layout(0),
      entries: &[BindGroupEntry {
        binding: 0,
        resource: viewport_buffer.as_entire_binding(),
      }],
    });

    Self {
      device: device.clone(),
      pipeline,
      rect_buffer,
      rect_capacity: 1,
      viewport_buffer,
      viewport_bind_group,
    }
  }

  pub fn render(
    &mut self,
    display_list: &DisplayList,
    width: u32,
    height: u32,
    queue: &Queue,
    encoder: &mut CommandEncoder,
    target: &TextureView,
  ) {
    let instances: Vec<_> = display_list
      .rects()
      .map(|(rect, color)| RectInstance {
        rect: [rect.x, rect.y, rect.w, rect.h],
        color: [color.r, color.g, color.b, color.a],
      })
      .collect();

    if instances.is_empty() || width == 0 || height == 0 {
      return;
    }

    if instances.len() > self.rect_capacity {
      self.rect_capacity = instances.len().next_power_of_two();

      self.rect_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("gluon_rect_buffer"),
        size: (self.rect_capacity * size_of::<RectInstance>()) as u64,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
      });
    }

    queue.write_buffer(&self.rect_buffer, 0, cast_slice(&instances));

    queue.write_buffer(
      &self.viewport_buffer,
      0,
      bytes_of(&ViewportUniform {
        size: [width as f32, height as f32],
        padding: [0.0, 0.0],
      }),
    );

    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
      label: Some("gluon_render_pass"),
      color_attachments: &[Some(RenderPassColorAttachment {
        view: target,
        depth_slice: None,
        resolve_target: None,
        ops: Operations {
          load: LoadOp::Load,
          store: StoreOp::Store,
        },
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
      multiview_mask: None,
    });

    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &self.viewport_bind_group, &[]);
    render_pass.set_vertex_buffer(0, self.rect_buffer.slice(..));
    render_pass.draw(0..6, 0..instances.len() as u32);
  }
}
