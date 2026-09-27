use gluon_core::Rect;
use gluon_tree::NodeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct Layout {
  rects: HashMap<NodeId, Rect>,
}

impl Layout {
  pub fn rect(&self, node: NodeId) -> Option<&Rect> {
    self.rects.get(&node)
  }

  pub(crate) fn insert(&mut self, node: NodeId, rect: Rect) {
    self.rects.insert(node, rect);
  }
}
