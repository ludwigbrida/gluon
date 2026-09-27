use crate::ComputedLayout;
use crate::layout::Layout;
use gluon_core::Rect;
use gluon_tree::Tree;

pub fn compute_layout(tree: &Tree<Layout>, viewport: Rect) -> ComputedLayout {
  let mut computed = ComputedLayout::default();

  computed.insert(tree.root(), viewport);

  computed
}
