use crate::layout::Layout;
use crate::{ComputedLayout, Direction, Length, Position};
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
  let parent_node = tree.node(parent).unwrap();
  let direction = parent_node.value.direction;
  let children = parent_node.children();
  let mut flow_offset = 0.0;

  for &child in children {
    let layout = tree.node(child).unwrap().value;

    let rect = match layout.position {
      Position::Flow => {
        let rect = resolve_flow(layout, parent_rect, direction, flow_offset);

        flow_offset += match direction {
          Direction::Horizontal => rect.w,
          Direction::Vertical => rect.h,
        };

        rect
      }
      Position::Absolute {
        top,
        right,
        bottom,
        left,
      } => resolve_absolute(layout, parent_rect, top, right, bottom, left),
    };

    layout_children(tree, child, &rect, computed);
    computed.insert(child, rect);
  }
}

fn resolve_flow(layout: Layout, parent: &Rect, direction: Direction, offset: f32) -> Rect {
  let available_width = match direction {
    Direction::Horizontal => (parent.w - offset).max(0.0),
    Direction::Vertical => parent.w,
  };

  let available_height = match direction {
    Direction::Horizontal => parent.h,
    Direction::Vertical => (parent.h - offset).max(0.0),
  };

  let width = resolve_length(layout.size.width, available_width, None, None);
  let height = resolve_length(layout.size.height, available_height, None, None);

  let (x, y) = match direction {
    Direction::Horizontal => (parent.x + offset, parent.y),
    Direction::Vertical => (parent.x, parent.y + offset),
  };

  Rect {
    x,
    y,
    w: width,
    h: height,
  }
}

fn resolve_absolute(
  layout: Layout,
  parent: &Rect,
  top: Option<f32>,
  right: Option<f32>,
  bottom: Option<f32>,
  left: Option<f32>,
) -> Rect {
  let width = resolve_length(layout.size.width, parent.w, left, right);
  let height = resolve_length(layout.size.height, parent.h, top, bottom);

  let x = match (left, right) {
    (Some(left), _) => parent.x + left,
    (None, Some(right)) => parent.x + parent.w - right - width,
    (None, None) => parent.x,
  };

  let y = match (top, bottom) {
    (Some(top), _) => parent.y + top,
    (None, Some(bottom)) => parent.y + parent.h - bottom - height,
    (None, None) => parent.y,
  };

  Rect {
    x,
    y,
    w: width,
    h: height,
  }
}

fn resolve_length(length: Length, available: f32, start: Option<f32>, end: Option<f32>) -> f32 {
  match length {
    Length::Content => 0.0,
    Length::Pixels(pixels) => pixels,
    Length::Fill => (available - start.unwrap_or(0.0) - end.unwrap_or(0.0)).max(0.0),
  }
}
