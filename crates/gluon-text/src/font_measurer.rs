use gluon_compose::{Content, Element};
use gluon_font::FontStore;
use gluon_layout::{Constraints, ContentMeasurer, ContentSize};

pub struct FontTextMeasurer<'a> {
  fonts: &'a FontStore,
}

impl<'a> FontTextMeasurer<'a> {
  pub const fn new(fonts: &'a FontStore) -> Self {
    Self { fonts }
  }
}

impl ContentMeasurer<Element> for FontTextMeasurer<'_> {
  fn measure(&self, element: &Element, _constraints: Constraints) -> ContentSize {
    let (Content::Text(text), Some(style)) = (&element.content, &element.style.text) else {
      return ContentSize::default();
    };

    if text.is_empty() {
      return ContentSize::default();
    }

    let Some(line) = self.fonts.line_metrics(style.font, style.size) else {
      return ContentSize::default();
    };

    let mut width = 0.0;

    for character in text.chars() {
      let Some(metrics) = self.fonts.glyph_metrics(style.font, character, style.size) else {
        return ContentSize::default();
      };

      width += metrics.advance.0;
    }

    ContentSize {
      width,
      height: line.height.0,
    }
  }
}
