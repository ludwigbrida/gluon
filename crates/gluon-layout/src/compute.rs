use crate::layout::Layout;
use crate::{ComputedLayout, Length, Position};
use gluon_core::Rect;
use gluon_tree::{NodeId, Tree};

pub fn compute_layout(tree: &Tree<Layout>, viewport: Rect) -> ComputedLayout {
  let mut computed = ComputedLayout::default();

  layout_children(tree, tree.root(), &viewport, &mut computed);
  computed.insert(tree.root(), viewport);

  computed
}

fn layout_children(
  tree: &Tree<Layout>,
  parent: NodeId,
  parent_rect: &Rect,
  computed: &mut ComputedLayout,
) {
  let children = tree.node(parent).unwrap().children();

  for &child in children {
    let layout = tree.node(child).unwrap().value;

    let Position::Absolute {
      top,
      right,
      bottom,
      left,
    } = layout.position
    else {
      continue;
    };

    let width = resolve_length(layout.size.width, parent_rect.w, left, right);
    let height = resolve_length(layout.size.height, parent_rect.h, top, bottom);

    let x = match (left, right) {
      (Some(left), _) => parent_rect.x + left,
      (None, Some(right)) => parent_rect.x + parent_rect.w - right - width,
      (None, None) => parent_rect.x,
    };

    let y = match (top, bottom) {
      (Some(top), _) => parent_rect.y + top,
      (None, Some(bottom)) => parent_rect.y + parent_rect.h - bottom - height,
      (None, None) => parent_rect.y,
    };

    let rect = Rect {
      x,
      y,
      w: width,
      h: height,
    };

    layout_children(tree, child, &rect, computed);

    computed.insert(child, rect);
  }

  fn resolve_length(length: Length, available: f32, start: Option<f32>, end: Option<f32>) -> f32 {
    match length {
      Length::Content => 0.0,
      Length::Pixels(pixels) => pixels,
      Length::Fill => (available - start.unwrap_or(0.0) - end.unwrap_or(0.0)).max(0.0),
    }
  }
}
