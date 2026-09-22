use gluon_core::DisplayList;
use wgpu::{
  BlendState, ColorTargetState, ColorWrites, CommandEncoder, Device, FragmentState,
  MultisampleState, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
  ShaderModuleDescriptor, ShaderSource, TextureFormat, TextureView, VertexState,
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
    _encoder: &mut CommandEncoder,
    _target: &TextureView,
  ) {
  }
}
