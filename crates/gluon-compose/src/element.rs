use crate::content::Content;
use gluon_layout::{Layout, LayoutItem};
use gluon_style::Style;

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
}
