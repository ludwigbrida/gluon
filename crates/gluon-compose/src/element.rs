use crate::{Content, ViewId};
use gluon_layout::{Layout, LayoutItem};
use gluon_style::Style;

#[derive(Default)]
pub struct Element {
  pub id: Option<ViewId>,
  pub layout: Layout,
  pub style: Style,
  pub content: Content,
}

impl LayoutItem for Element {
  fn layout(&self) -> &Layout {
    &self.layout
  }
}
