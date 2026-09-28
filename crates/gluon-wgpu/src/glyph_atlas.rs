use gluon_core::Pixels;
use gluon_font::{FontId, FontStore};
use std::collections::HashMap;
use wgpu::{
  AddressMode, Device, Extent3d, FilterMode, Origin3d, Queue, Sampler, SamplerDescriptor,
  TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor,
  TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

const ATLAS_SIZE: u32 = 1024;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GlyphKey {
  font: FontId,
  character: char,
  size_bits: u32,
}

#[derive(Clone, Copy)]
pub(crate) struct AtlasGlyph {
  pub x: u32,
  pub y: u32,
  pub width: u32,
  pub height: u32,
  pub left: f32,
  pub bottom: f32,
  pub advance: f32,
}

pub(crate) struct GlyphAtlas {
  texture: Texture,
  view: TextureView,
  sampler: Sampler,
  glyphs: HashMap<GlyphKey, AtlasGlyph>,
  cursor_x: u32,
  cursor_y: u32,
  row_height: u32,
}

impl GlyphAtlas {
  pub fn new(device: &Device) -> Self {
    let texture = device.create_texture(&TextureDescriptor {
      label: Some("gluon_glyph_atlas"),
      size: Extent3d {
        width: ATLAS_SIZE,
        height: ATLAS_SIZE,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::R8Unorm,
      usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
      view_formats: &[],
    });

    let view = texture.create_view(&TextureViewDescriptor::default());

    let sampler = device.create_sampler(&SamplerDescriptor {
      label: Some("gluon_glyph_atlas_sampler"),
      address_mode_u: AddressMode::ClampToEdge,
      address_mode_v: AddressMode::ClampToEdge,
      address_mode_w: AddressMode::ClampToEdge,
      mag_filter: FilterMode::Linear,
      min_filter: FilterMode::Linear,
      ..Default::default()
    });

    Self {
      texture,
      view,
      sampler,
      glyphs: HashMap::new(),
      cursor_x: 0,
      cursor_y: 0,
      row_height: 0,
    }
  }

  pub fn glyph(
    &mut self,
    fonts: &FontStore,
    font: FontId,
    character: char,
    logical_size: Pixels,
    scale_factor: f32,
    queue: &Queue,
  ) -> Option<AtlasGlyph> {
    let physical_size = Pixels::new(logical_size.0.max(0.0) * scale_factor);

    let key = GlyphKey {
      font,
      character,
      size_bits: physical_size.0.to_bits(),
    };

    if let Some(glyph) = self.glyphs.get(&key) {
      return Some(*glyph);
    }

    let glyph = fonts.rasterize(font, character, physical_size)?;

    if glyph.width > ATLAS_SIZE || glyph.height > ATLAS_SIZE {
      return None;
    }

    if self.cursor_x + glyph.width > ATLAS_SIZE {
      self.cursor_x = 0;
      self.cursor_y += self.row_height;
      self.row_height = 0;
    }

    if self.cursor_y + glyph.height > ATLAS_SIZE {
      return None;
    }

    let atlas_glyph = AtlasGlyph {
      x: self.cursor_x,
      y: self.cursor_y,
      width: glyph.width,
      height: glyph.height,
      left: glyph.left.0,
      bottom: glyph.bottom.0,
      advance: glyph.advance.0,
    };

    self.cursor_x += glyph.width;
    self.row_height = self.row_height.max(glyph.height);

    if glyph.width != 0 && glyph.height != 0 {
      queue.write_texture(
        TexelCopyTextureInfo {
          texture: &self.texture,
          mip_level: 0,
          origin: Origin3d {
            x: atlas_glyph.x,
            y: atlas_glyph.y,
            z: 0,
          },
          aspect: TextureAspect::All,
        },
        &glyph.coverage,
        TexelCopyBufferLayout {
          offset: 0,
          bytes_per_row: Some(glyph.width),
          rows_per_image: Some(glyph.height),
        },
        Extent3d {
          width: glyph.width,
          height: glyph.height,
          depth_or_array_layers: 1,
        },
      );
    }

    self.glyphs.insert(key, atlas_glyph);

    Some(atlas_glyph)
  }

  pub fn view(&self) -> &TextureView {
    &self.view
  }

  pub fn sampler(&self) -> &Sampler {
    &self.sampler
  }
}
