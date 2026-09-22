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
}

impl Default for DisplayList {
  fn default() -> Self {
    Self::new()
  }
}
