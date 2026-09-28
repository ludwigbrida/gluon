use crate::glyph_atlas::GlyphAtlas;
use bytemuck::{Pod, Zeroable, bytes_of, cast_slice};
use gluon_core::Viewport;
use gluon_font::FontStore;
use gluon_paint::{PaintCommand, PaintList, TextRun};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
  BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
  BindingType, BlendState, Buffer, BufferBindingType, BufferUsages, ColorTargetState, ColorWrites,
  CommandEncoder, Device, FragmentState, LoadOp, MultisampleState, Operations,
  PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor,
  RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, ShaderModuleDescriptor,
  ShaderSource, ShaderStages, StoreOp, TextureFormat, TextureSampleType, TextureView,
  TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState,
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
struct GlyphInstance {
  rect: [f32; 4],
  uv: [f32; 4],
  color: [f32; 4],
}

enum DrawCommand {
  Rect(usize),
  Glyphs { start: usize, count: usize },
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct ViewportUniform {
  physical_size: [f32; 2],
  scale_factor: f32,
  padding: f32,
}

pub struct Renderer {
  device: Device,
  pipeline: RenderPipeline,
  rect_buffer: Buffer,
  rect_capacity: usize,
  viewport_buffer: Buffer,
  viewport_bind_group: BindGroup,
  glyph_pipeline: RenderPipeline,
  glyph_buffer: Buffer,
  glyph_capacity: usize,
  glyph_atlas: GlyphAtlas,
  glyph_atlas_bind_group: BindGroup,
}

impl Renderer {
  pub fn new(device: &Device, target_format: TextureFormat) -> Self {
    let viewport_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      label: Some("gluon_viewport_bind_group_layout"),
      entries: &[BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
          ty: BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    let glyph_atlas_bind_group_layout =
      device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("gluon_glyph_atlas_bind_group_layout"),
        entries: &[
          BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Texture {
              sample_type: TextureSampleType::Float { filterable: true },
              view_dimension: TextureViewDimension::D2,
              multisampled: false,
            },
            count: None,
          },
          BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Sampler(SamplerBindingType::Filtering),
            count: None,
          },
        ],
      });

    let glyph_atlas = GlyphAtlas::new(device);

    let glyph_atlas_bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some("gluon_glyph_atlas_bind_group"),
      layout: &glyph_atlas_bind_group_layout,
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(glyph_atlas.view()),
        },
        BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(glyph_atlas.sampler()),
        },
      ],
    });

    let glyph_shader = device.create_shader_module(ShaderModuleDescriptor {
      label: Some("gluon_glyph_shader"),
      source: ShaderSource::Wgsl(include_str!("glyph.wgsl").into()),
    });

    let glyph_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: Some("gluon_glyph_pipeline_layout"),
      bind_group_layouts: &[
        Some(&viewport_bind_group_layout),
        Some(&glyph_atlas_bind_group_layout),
      ],
      immediate_size: 0,
    });

    let glyph_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
      label: Some("gluon_glyph_pipeline"),
      layout: Some(&glyph_pipeline_layout),
      vertex: VertexState {
        module: &glyph_shader,
        entry_point: Some("vertex_main"),
        compilation_options: Default::default(),
        buffers: &[Some(VertexBufferLayout {
          array_stride: size_of::<GlyphInstance>() as u64,
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
            VertexAttribute {
              format: VertexFormat::Float32x4,
              offset: 32,
              shader_location: 2,
            },
          ],
        })],
      },
      primitive: PrimitiveState::default(),
      depth_stencil: None,
      multisample: MultisampleState::default(),
      fragment: Some(FragmentState {
        module: &glyph_shader,
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

    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
      label: Some("gluon_shader_module"),
      source: ShaderSource::Wgsl(include_str!("rect.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: Some("gluon_rect_pipeline_layout"),
      bind_group_layouts: &[Some(&viewport_bind_group_layout)],
      immediate_size: 0,
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
      label: Some("gluon_pipeline"),
      layout: Some(&pipeline_layout),
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

    let glyph_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("gluon_glyph_buffer"),
      contents: bytes_of(&GlyphInstance::zeroed()),
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    });

    let viewport_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("gluon_viewport_buffer"),
      contents: bytes_of(&ViewportUniform::zeroed()),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let viewport_bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some("gluon_viewport_bind_group"),
      layout: &viewport_bind_group_layout,
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
      glyph_pipeline,
      glyph_buffer,
      glyph_capacity: 1,
      glyph_atlas,
      glyph_atlas_bind_group,
    }
  }

  pub fn render(
    &mut self,
    paint_list: &PaintList,
    viewport: Viewport,
    fonts: &FontStore,
    queue: &Queue,
    encoder: &mut CommandEncoder,
    target: &TextureView,
  ) {
    let mut rect_instances = Vec::new();
    let mut glyph_instances = Vec::new();
    let mut commands = Vec::new();

    for command in paint_list.commands() {
      match command {
        PaintCommand::Rect { rect, color } => {
          rect_instances.push(RectInstance {
            rect: [rect.x, rect.y, rect.w, rect.h],
            color: [color.r, color.g, color.b, color.a],
          });

          commands.push(DrawCommand::Rect(rect_instances.len() - 1));
        }
        PaintCommand::Text(text) => {
          if let Some((start, count)) =
            self.append_text_run(text, fonts, queue, &mut glyph_instances)
          {
            if count != 0 {
              commands.push(DrawCommand::Glyphs { start, count });
            }
          }
        }
      }
    }

    if commands.is_empty()
      || viewport.physical_width == 0
      || viewport.physical_height == 0
      || !viewport.scale_factor.0.is_finite()
      || viewport.scale_factor.0 <= 0.0
    {
      return;
    }

    if rect_instances.len() > self.rect_capacity {
      self.rect_capacity = rect_instances.len().next_power_of_two();

      self.rect_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("gluon_rect_buffer"),
        size: (self.rect_capacity * size_of::<RectInstance>()) as u64,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
      });
    }

    if !rect_instances.is_empty() {
      queue.write_buffer(&self.rect_buffer, 0, cast_slice(&rect_instances));
    }

    if glyph_instances.len() > self.glyph_capacity {
      self.glyph_capacity = glyph_instances.len().next_power_of_two();

      self.glyph_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("gluon_glyph_buffer"),
        size: (self.glyph_capacity * size_of::<GlyphInstance>()) as u64,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
      });
    }

    if !glyph_instances.is_empty() {
      queue.write_buffer(&self.glyph_buffer, 0, cast_slice(&glyph_instances));
    }

    queue.write_buffer(
      &self.viewport_buffer,
      0,
      bytes_of(&ViewportUniform {
        physical_size: [
          viewport.physical_width as f32,
          viewport.physical_height as f32,
        ],
        scale_factor: viewport.scale_factor.0,
        padding: 0.0,
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

    for command in commands {
      match command {
        DrawCommand::Rect(index) => {
          let offset = index as u64 * size_of::<RectInstance>() as u64;

          render_pass.set_pipeline(&self.pipeline);
          render_pass.set_bind_group(0, &self.viewport_bind_group, &[]);
          render_pass.set_vertex_buffer(
            0,
            self
              .rect_buffer
              .slice(offset..offset + size_of::<RectInstance>() as u64),
          );
          render_pass.draw(0..6, 0..1);
        }
        DrawCommand::Glyphs { start, count } => {
          let offset = start as u64 * size_of::<GlyphInstance>() as u64;
          let end = offset + count as u64 * size_of::<GlyphInstance>() as u64;

          render_pass.set_pipeline(&self.glyph_pipeline);
          render_pass.set_bind_group(0, &self.viewport_bind_group, &[]);
          render_pass.set_bind_group(1, &self.glyph_atlas_bind_group, &[]);
          render_pass.set_vertex_buffer(0, self.glyph_buffer.slice(offset..end));
          render_pass.draw(0..6, 0..count as u32);
        }
      }
    }
  }

  fn append_text_run(
    &mut self,
    text: &TextRun,
    fonts: &FontStore,
    queue: &Queue,
    instances: &mut Vec<GlyphInstance>,
  ) -> Option<(usize, usize)> {
    let line = fonts.line_metrics(text.font, text.size)?;

    let mut glyphs = Vec::with_capacity(text.text.chars().count());
    let mut width = 0.0;

    for character in text.text.chars() {
      let glyph = self
        .glyph_atlas
        .glyph(fonts, text.font, character, text.size, queue)?;

      width += glyph.advance;
      glyphs.push(glyph);
    }

    let start = instances.len();
    let mut pen_x = text.start_x(width);

    let baseline_y =
      text.bounds.y + ((text.bounds.h - line.height.0) * 0.5).max(0.0) + line.ascent.0;

    let atlas_size = 1024.0;

    for glyph in glyphs {
      if glyph.width != 0 && glyph.height != 0 {
        instances.push(GlyphInstance {
          rect: [
            pen_x + glyph.left,
            baseline_y - glyph.bottom - glyph.height as f32,
            glyph.width as f32,
            glyph.height as f32,
          ],
          uv: [
            glyph.x as f32 / atlas_size,
            glyph.y as f32 / atlas_size,
            glyph.width as f32 / atlas_size,
            glyph.height as f32 / atlas_size,
          ],
          color: [text.color.r, text.color.g, text.color.b, text.color.a],
        });
      }

      pen_x += glyph.advance;
    }

    Some((start, instances.len() - start))
  }
}
