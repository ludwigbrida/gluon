use crate::{ComputedLayout, Direction, LayoutItem, LayoutMode, Length};
use gluon_core::Rect;
use gluon_tree::{NodeId, Tree};

pub fn compute_layout<T: LayoutItem>(tree: &Tree<T>, viewport: Rect) -> ComputedLayout {
  let mut computed = ComputedLayout::default();

  layout_children(tree, tree.root(), &viewport, &mut computed);
  computed.insert(tree.root(), viewport);

  computed
}

fn layout_children<T: LayoutItem>(
  tree: &Tree<T>,
  parent: NodeId,
  parent_rect: &Rect,
  computed: &mut ComputedLayout,
) {
  let parent_node = tree.node(parent).unwrap();
  let direction = parent_node.value.layout().direction;
  let mode = parent_node.value.layout().mode;
  let children = parent_node.children();
  let mut flow_offset = 0.0;

  for &child in children {
    let layout = tree.node(child).unwrap().value.layout();

    let width = resolve_length(layout.size.width, parent_rect.w);
    let height = resolve_length(layout.size.height, parent_rect.h);

    let (x, y) = match mode {
      LayoutMode::Flow => match direction {
        Direction::Horizontal => (parent_rect.x + flow_offset, parent_rect.y),
        Direction::Vertical => (parent_rect.x, parent_rect.y + flow_offset),
      },
      LayoutMode::Stack => (parent_rect.x, parent_rect.y),
    };

    let rect = Rect {
      x,
      y,
      w: width,
      h: height,
    };

    if mode == LayoutMode::Flow {
      flow_offset += match direction {
        Direction::Horizontal => width,
        Direction::Vertical => height,
      };
    }

    layout_children(tree, child, &rect, computed);
    computed.insert(child, rect);
  }
}

fn resolve_length(length: Length, available: f32) -> f32 {
  match length {
    Length::Content => 0.0,
    Length::Fraction(fraction) => available * fraction.max(0.0),
  }
}
