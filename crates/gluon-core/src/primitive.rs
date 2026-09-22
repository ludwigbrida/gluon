use crate::color::Color;
use crate::rect::Rect;

pub enum Primitive {
  Rect { rect: Rect, color: Color },
}
