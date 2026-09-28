use crate::{FontId, GlyphMetrics, LineMetrics};
use fontdue::{Font, FontSettings};
use gluon_core::Pixels;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontLoadError {
  Parse(&'static str),
}

#[derive(Default)]
pub struct FontStore {
  fonts: Vec<Font>,
}

impl FontStore {
  pub fn load(&mut self, bytes: &[u8]) -> Result<FontId, FontLoadError> {
    let font = Font::from_bytes(bytes, FontSettings::default()).map_err(FontLoadError::Parse)?;

    let id = FontId(self.fonts.len());

    self.fonts.push(font);

    Ok(id)
  }

  pub fn glyph_metrics(&self, font: FontId, character: char, size: Pixels) -> Option<GlyphMetrics> {
    let font = self.fonts.get(font.0)?;
    let metrics = font.metrics(character, size.0.max(0.0));

    Some(GlyphMetrics {
      advance: Pixels::new(metrics.advance_width),
    })
  }

  pub fn line_metrics(&self, font: FontId, size: Pixels) -> Option<LineMetrics> {
    let font = self.fonts.get(font.0)?;
    let metrics = font.horizontal_line_metrics(size.0.max(0.0))?;

    Some(LineMetrics {
      ascent: Pixels::new(metrics.ascent),
      descent: Pixels::new(metrics.descent),
      height: Pixels::new(metrics.new_line_size),
    })
  }
}
