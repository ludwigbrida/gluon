use crate::color::Color;
use crate::primitive::Primitive;
use crate::rect::Rect;

pub struct DisplayList {
  pub primitives: Vec<Primitive>,
}

impl DisplayList {
  pub fn new() -> Self {
    Self {
      primitives: Vec::new(),
    }
  }

  pub fn rect(&mut self, rect: Rect, color: Color) {
    self.primitives.push(Primitive::Rect { rect, color })
  }

  pub fn rects(&self) -> impl Iterator<Item = (&Rect, &Color)> {
    self
      .primitives
      .iter()
      .filter_map(|primitive| match primitive {
        Primitive::Rect { rect, color } => Some((rect, color)),
      })
  }
}

impl Default for DisplayList {
  fn default() -> Self {
    Self::new()
  }
}
