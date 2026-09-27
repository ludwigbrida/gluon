use crate::{Alignment, ComputedLayout, Direction, LayoutItem, LayoutMode, Length};
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
  let parent_layout = parent_node.value.layout();
  let children = parent_node.children();

  let main_available = match parent_layout.direction {
    Direction::Horizontal => parent_rect.w,
    Direction::Vertical => parent_rect.h,
  };

  let total_main_size: f32 = children
    .iter()
    .map(|&child| {
      let layout = tree.node(child).unwrap().value.layout();

      match parent_layout.direction {
        Direction::Horizontal => resolve_length(layout.size.width, parent_rect.w),
        Direction::Vertical => resolve_length(layout.size.height, parent_rect.h),
      }
    })
    .sum();

  let mut flow_offset = match parent_layout.main_alignment {
    Alignment::Start => 0.0,
    Alignment::Center => (main_available - total_main_size) * 0.5,
    Alignment::End => main_available - total_main_size,
  };

  for &child in children {
    let layout = *tree.node(child).unwrap().value.layout();

    let width = resolve_length(layout.size.width, parent_rect.w);
    let height = resolve_length(layout.size.height, parent_rect.h);

    let (x, y) = match parent_layout.mode {
      LayoutMode::Flow => match parent_layout.direction {
        Direction::Horizontal => (
          parent_rect.x + flow_offset,
          align(
            parent_rect.y,
            parent_rect.h,
            height,
            parent_layout.cross_alignment,
          ),
        ),
        Direction::Vertical => (
          align(
            parent_rect.x,
            parent_rect.w,
            width,
            parent_layout.cross_alignment,
          ),
          parent_rect.y + flow_offset,
        ),
      },
      LayoutMode::Stack => (
        align(
          parent_rect.x,
          parent_rect.w,
          width,
          layout.stack_alignment.horizontal,
        ),
        align(
          parent_rect.y,
          parent_rect.h,
          height,
          layout.stack_alignment.vertical,
        ),
      ),
    };

    let rect = Rect {
      x,
      y,
      w: width,
      h: height,
    };

    if parent_layout.mode == LayoutMode::Flow {
      flow_offset += match parent_layout.direction {
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

fn align(start: f32, available: f32, size: f32, alignment: Alignment) -> f32 {
  match alignment {
    Alignment::Start => start,
    Alignment::Center => start + (available - size) * 0.5,
    Alignment::End => start + available - size,
  }
}
