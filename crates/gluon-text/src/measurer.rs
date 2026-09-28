use crate::measure;
use gluon_compose::{Content, Element};
use gluon_layout::{Constraints, ContentMeasurer, ContentSize};

#[derive(Default)]
pub struct BitmapTextMeasurer;

impl ContentMeasurer<Element> for BitmapTextMeasurer {
  fn measure(&self, element: &Element, _constraints: Constraints) -> ContentSize {
    let (Content::Text(text), Some(style)) = (&element.content, &element.style.text) else {
      return ContentSize::default();
    };

    let metrics = measure(text, style);

    ContentSize {
      width: metrics.width,
      height: metrics.height,
    }
  }
}
