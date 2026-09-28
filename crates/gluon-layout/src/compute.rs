use crate::layout::ContentMeasurer;
use crate::{
  Alignment, ComputedLayout, Constraints, Direction, LayoutItem, LayoutMode, Length, MainAlignment,
  Padding,
};
use gluon_core::Rect;
use gluon_tree::{NodeId, Tree};

pub fn compute_layout<T, M>(tree: &Tree<T>, viewport: Rect, measurer: &M) -> ComputedLayout
where
  T: LayoutItem,
  M: ContentMeasurer<T>,
{
  let mut computed = ComputedLayout::default();

  layout_children(tree, tree.root(), &viewport, measurer, &mut computed);
  computed.insert(tree.root(), viewport);

  computed
}

fn layout_children<T, M>(
  tree: &Tree<T>,
  parent: NodeId,
  parent_rect: &Rect,
  measurer: &M,
  computed: &mut ComputedLayout,
) where
  T: LayoutItem,
  M: ContentMeasurer<T>,
{
  let parent_node = tree.node(parent).unwrap();
  let parent_layout = parent_node.value.layout();
  let children = parent_node.children();
  let content_rect = content_rect(parent_rect, parent_layout.padding);

  let main_available = match parent_layout.direction {
    Direction::Horizontal => content_rect.w,
    Direction::Vertical => content_rect.h,
  };

  let base_gap = main_available * parent_layout.gap.max(0.0);

  let total_main_size: f32 = children
    .iter()
    .map(|&child| {
      let child = tree.node(child).unwrap();
      let layout = child.value.layout();
      let content_size = measurer.measure(
        &child.value,
        Constraints {
          max_width: content_rect.w,
          max_height: content_rect.h,
        },
      );

      match parent_layout.direction {
        Direction::Horizontal => {
          resolve_length(layout.size.width, content_rect.w, content_size.width)
        }
        Direction::Vertical => {
          resolve_length(layout.size.height, content_rect.h, content_size.height)
        }
      }
    })
    .sum::<f32>()
    + base_gap * children.len().saturating_sub(1) as f32;

  let (mut flow_offset, gap) = match parent_layout.main_alignment {
    MainAlignment::Start => (0.0, base_gap),
    MainAlignment::Center => ((main_available - total_main_size) * 0.5, base_gap),
    MainAlignment::End => (main_available - total_main_size, base_gap),
    MainAlignment::SpaceBetween if children.len() > 1 => {
      let free_space = (main_available - total_main_size).max(0.0);
      let gap = base_gap + free_space / (children.len() - 1) as f32;

      (0.0, gap)
    }
    MainAlignment::SpaceBetween => (0.0, base_gap),
  };

  for &child in children {
    let child_node = tree.node(child).unwrap();
    let layout = *child_node.value.layout();
    let content_size = measurer.measure(
      &child_node.value,
      Constraints {
        max_width: content_rect.w,
        max_height: content_rect.h,
      },
    );

    let width = resolve_length(layout.size.width, content_rect.w, content_size.width);
    let height = resolve_length(layout.size.height, content_rect.h, content_size.height);

    let (x, y) = match parent_layout.mode {
      LayoutMode::Flow => match parent_layout.direction {
        Direction::Horizontal => (
          content_rect.x + flow_offset,
          align(
            content_rect.y,
            content_rect.h,
            height,
            parent_layout.cross_alignment,
          ),
        ),
        Direction::Vertical => (
          align(
            content_rect.x,
            content_rect.w,
            width,
            parent_layout.cross_alignment,
          ),
          content_rect.y + flow_offset,
        ),
      },
      LayoutMode::Stack => (
        align(
          content_rect.x,
          content_rect.w,
          width,
          layout.stack_alignment.horizontal,
        ),
        align(
          content_rect.y,
          content_rect.h,
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
        Direction::Horizontal => width + gap,
        Direction::Vertical => height + gap,
      };
    }

    layout_children(tree, child, &rect, measurer, computed);
    computed.insert(child, rect);
  }
}

fn content_rect(rect: &Rect, padding: Padding) -> Rect {
  let left = rect.w * padding.left.max(0.0);
  let right = rect.w * padding.right.max(0.0);
  let top = rect.h * padding.top.max(0.0);
  let bottom = rect.h * padding.bottom.max(0.0);

  Rect {
    x: rect.x + left,
    y: rect.y + top,
    w: (rect.w - left - right).max(0.0),
    h: (rect.h - top - bottom).max(0.0),
  }
}

fn resolve_length(length: Length, available: f32, content: f32) -> f32 {
  match length {
    Length::Content => content.max(0.0),
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
