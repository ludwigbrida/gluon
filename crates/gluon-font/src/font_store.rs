use crate::FontId;
use fontdue::{Font, FontSettings};

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
}
