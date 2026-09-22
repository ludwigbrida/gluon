use gluon_core::DisplayList;
use wgpu::{
  BlendState, ColorTargetState, ColorWrites, CommandEncoder, Device, FragmentState, LoadOp,
  MultisampleState, Operations, PrimitiveState, Queue, RenderPassColorAttachment,
  RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
  ShaderSource, StoreOp, TextureFormat, TextureView, VertexState,
};

pub struct Renderer {
  pipeline: RenderPipeline,
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

    Self { pipeline }
  }

  pub fn render(
    &self,
    _display_list: &DisplayList,
    _width: u32,
    _height: u32,
    _queue: &Queue,
    encoder: &mut CommandEncoder,
    target: &TextureView,
  ) {
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
    render_pass.draw(0..6, 0..1);
  }
}
