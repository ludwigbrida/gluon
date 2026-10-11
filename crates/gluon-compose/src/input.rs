use crate::{Composition, ViewId};
use gluon_layout::ComputedLayout;
use gluon_tree::NodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointerButton {
  Left,
  Right,
  Middle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointerEventKind {
  Move,
  Down,
  Up,
  Click,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerEvent {
  pub kind: PointerEventKind,
  pub x: f32,
  pub y: f32,
  pub button: Option<PointerButton>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TargetedPointerEvent {
  pub target: ViewId,
  pub event: PointerEvent,
}

pub fn hit_test(
  composition: &Composition,
  layout: &ComputedLayout,
  x: f32,
  y: f32,
) -> Option<NodeId> {
  hit_test_node(composition, layout, composition.root(), x, y)
}

pub fn hit_test_view(
  composition: &Composition,
  layout: &ComputedLayout,
  x: f32,
  y: f32,
) -> Option<ViewId> {
  hit_test(composition, layout, x, y)
    .and_then(|node| composition.node(node))
    .and_then(|node| node.value.id)
}

fn hit_test_node(
  composition: &Composition,
  layout: &ComputedLayout,
  node: NodeId,
  x: f32,
  y: f32,
) -> Option<NodeId> {
  let tree_node = composition.node(node)?;
  for &child in tree_node.children().iter().rev() {
    if let Some(target) = hit_test_node(composition, layout, child, x, y) {
      return Some(target);
    }
  }
  let rect = layout.rect(node)?;
  let contains = x.is_finite()
    && y.is_finite()
    && x >= rect.x
    && x <= rect.x + rect.w
    && y >= rect.y
    && y <= rect.y + rect.h;
  contains
    .then_some(node)
    .filter(|_| tree_node.value.id.is_some())
}

pub fn dispatch_pointer_event(
  composition: &Composition,
  layout: &ComputedLayout,
  event: PointerEvent,
) -> Option<ViewId> {
  hit_test_view(composition, layout, event.x, event.y)
}

#[derive(Default)]
pub struct PointerDispatcher {
  pressed_target: Option<ViewId>,
}

impl PointerDispatcher {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn dispatch(
    &mut self,
    composition: &Composition,
    layout: &ComputedLayout,
    event: PointerEvent,
  ) -> Vec<TargetedPointerEvent> {
    let target = dispatch_pointer_event(composition, layout, event);
    let mut dispatched = Vec::new();
    match event.kind {
      PointerEventKind::Down => self.pressed_target = target,
      PointerEventKind::Up => {
        let pressed_target = self.pressed_target.take();
        if pressed_target == target
          && let Some(target) = target
        {
          dispatched.push(TargetedPointerEvent {
            target,
            event: PointerEvent {
              kind: PointerEventKind::Click,
              ..event
            },
          });
        }
      }
      PointerEventKind::Move | PointerEventKind::Click => {}
    }
    if let Some(target) = target {
      dispatched.insert(0, TargetedPointerEvent { target, event });
    }
    dispatched
  }
}
