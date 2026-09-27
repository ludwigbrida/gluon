use crate::text::paint_text;
use gluon_compose::{Composition, Content};
use gluon_core::{DisplayList, Rect};
use gluon_layout::ComputedLayout;
use gluon_style::Border;
use gluon_tree::NodeId;

pub fn paint(composition: &Composition, layout: &ComputedLayout) -> DisplayList {
  let mut display_list = DisplayList::new();

  paint_node(composition, composition.root(), layout, &mut display_list);

  display_list
}

fn paint_node(
  composition: &Composition,
  node: NodeId,
  layout: &ComputedLayout,
  display_list: &mut DisplayList,
) {
  let Some(rect) = layout.rect(node) else {
    return;
  };

  let element = &composition.node(node).unwrap().value;

  if let Some(background) = &element.style.background {
    display_list.rect(*rect, *background);
  }

  paint_borders(rect, &element.style.border, display_list);

  if let (Content::Text(text), Some(style)) = (&element.content, &element.style.text) {
    paint_text(display_list, rect, text, style);
  }

  for &child in composition.node(node).unwrap().children() {
    paint_node(composition, child, layout, display_list);
  }
}

fn paint_borders(rect: &Rect, border: &Border, display_list: &mut DisplayList) {
  if let Some(side) = &border.top {
    let height = side.width.max(0.0).min(rect.h);

    display_list.rect(
      Rect {
        x: rect.x,
        y: rect.y,
        w: rect.w,
        h: height,
      },
      side.color,
    );
  }

  if let Some(side) = &border.right {
    let width = side.width.max(0.0).min(rect.w);

    display_list.rect(
      Rect {
        x: rect.x + rect.w - width,
        y: rect.y,
        w: width,
        h: rect.h,
      },
      side.color,
    );
  }

  if let Some(side) = &border.bottom {
    let height = side.width.max(0.0).min(rect.h);

    display_list.rect(
      Rect {
        x: rect.x,
        y: rect.y + rect.h - height,
        w: rect.w,
        h: height,
      },
      side.color,
    );
  }

  if let Some(side) = &border.left {
    let width = side.width.max(0.0).min(rect.w);

    display_list.rect(
      Rect {
        x: rect.x,
        y: rect.y,
        w: width,
        h: rect.h,
      },
      side.color,
    );
  }
}
