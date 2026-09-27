use crate::content::Content;
use gluon_layout::{Constraints, ContentSize, Layout, LayoutItem};
use gluon_style::Style;
use gluon_text::measure;

#[derive(Default)]
pub struct Element {
  pub layout: Layout,
  pub style: Style,
  pub content: Content,
}

impl LayoutItem for Element {
  fn layout(&self) -> &Layout {
    &self.layout
  }

  fn measure_content(&self, _constraints: Constraints) -> ContentSize {
    let (Content::Text(text), Some(style)) = (&self.content, &self.style.text) else {
      return ContentSize::default();
    };

    let metrics = measure(text, style);

    ContentSize {
      width: metrics.width,
      height: metrics.height,
    }
  }
}
