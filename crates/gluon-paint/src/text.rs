use crate::{PaintList, TextRun};
use gluon_core::Rect;
use gluon_style::{TextAlignment, TextStyle};
use gluon_text::{GLYPH_ADVANCE, GLYPH_HEIGHT, GLYPH_WIDTH, glyph, measure};

pub fn paint_text(paint_list: &mut PaintList, bounds: &Rect, text: &str, style: &TextStyle) {
  let size = style.size.0.max(0.0);
  let scale = size / GLYPH_HEIGHT as f32;

  if scale == 0.0 {
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

  let metrics = measure(text, style);

  let x = match style.alignment {
    TextAlignment::Start => bounds.x,
    TextAlignment::Center => bounds.x + (bounds.w - metrics.width) * 0.5,
    TextAlignment::End => bounds.x + bounds.w - metrics.width,
  };
  let y = bounds.y + ((bounds.h - size) * 0.5).max(0.0);

  for (index, character) in text.chars().enumerate() {
    let glyph_x = x + index as f32 * GLYPH_ADVANCE as f32 * scale;

    for (row, bits) in glyph(character).iter().enumerate() {
      for column in 0..GLYPH_WIDTH {
        if bits & (0b1_0000 >> column) != 0 {
          paint_list.rect(
            Rect {
              x: glyph_x + column as f32 * scale,
              y: y + row as f32 * scale,
              w: scale,
              h: scale,
            },
            style.color,
          );
        }
      }
    }
  }
}
