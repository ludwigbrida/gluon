use crate::RectUniform;
use bytemuck::{Zeroable, bytes_of};
use gluon_core::{DisplayList, Primitive};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
  BindGroup, BindGroupDescriptor, BindGroupEntry, BlendState, Buffer, BufferUsages,
  ColorTargetState, ColorWrites, CommandEncoder, Device, FragmentState, LoadOp, MultisampleState,
  Operations, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor,
  RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StoreOp,
  TextureFormat, TextureView, VertexState,
};

pub struct Renderer {
  pipeline: RenderPipeline,
  rect_buffer: Buffer,
  rect_bind_group: BindGroup,
}

impl Renderer {
  pub fn new(device: &Device, target_format: TextureFormat) -> Self {
    let shader_module_descriptor = ShaderModuleDescriptor {
      label: Some("gluon_shader_module"),
      source: ShaderSource::Wgsl(include_str!("rect.wgsl").into()),
    };

    let shader_module = device.create_shader_module(shader_module_descriptor);

    let pipeline_descriptor = &RenderPipelineDescriptor {
      label: Some("gluon_pipeline"),
      layout: None,
      vertex: VertexState {
        module: &shader_module,
        entry_point: Some("vertex_main"),
        compilation_options: Default::default(),
        buffers: &[],
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
    };

    let pipeline = device.create_render_pipeline(pipeline_descriptor);

    let initial_rect = RectUniform::zeroed();

    let rect_buffer_descriptor = &BufferInitDescriptor {
      label: Some("gluon_rect_buffer"),
      contents: bytes_of(&initial_rect),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    };

    let rect_buffer = device.create_buffer_init(rect_buffer_descriptor);

    let rect_bind_group_descriptor = &BindGroupDescriptor {
      label: Some("gluon_rect_bind_group"),
      layout: &pipeline.get_bind_group_layout(0),
      entries: &[BindGroupEntry {
        binding: 0,
        resource: rect_buffer.as_entire_binding(),
      }],
    };

    let rect_bind_group = device.create_bind_group(rect_bind_group_descriptor);

    Self {
      pipeline,
      rect_buffer,
      rect_bind_group,
    }
  }

  pub fn render(
    &self,
    display_list: &DisplayList,
    width: u32,
    height: u32,
    queue: &Queue,
    encoder: &mut CommandEncoder,
    target: &TextureView,
  ) {
    let Some(Primitive::Rect { rect, color }) = display_list.primitives.first() else {
      return;
    };

    let uniform = RectUniform::from_rect(rect, color, width, height);

    queue.write_buffer(&self.rect_buffer, 0, bytes_of(&uniform));

    let render_pass_descriptor = &RenderPassDescriptor {
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
    };

    let mut render_pass = encoder.begin_render_pass(render_pass_descriptor);

    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &self.rect_bind_group, &[]);
    render_pass.draw(0..6, 0..1);
  }
}
