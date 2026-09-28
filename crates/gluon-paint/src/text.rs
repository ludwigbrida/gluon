use crate::{PaintList, TextRun};
use gluon_core::Rect;
use gluon_style::TextStyle;

pub fn paint_text(paint_list: &mut PaintList, bounds: &Rect, text: &str, style: &TextStyle) {
  if text.is_empty() || style.size.0 <= 0.0 {
    return;
  }

  paint_list.text(TextRun {
    bounds: *bounds,
    text: text.to_owned(),
    font: style.font,
    color: style.color,
    size: style.size,
    alignment: style.alignment,
  });
}
