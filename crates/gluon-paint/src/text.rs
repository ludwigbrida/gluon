use gluon_core::{DisplayList, Rect};
use gluon_style::{TextAlignment, TextStyle};
use gluon_text::{GLYPH_ADVANCE, GLYPH_HEIGHT, GLYPH_WIDTH, glyph, measure};

pub fn paint_text(display_list: &mut DisplayList, bounds: &Rect, text: &str, style: &TextStyle) {
  let scale = style.size.max(0.0) / GLYPH_HEIGHT as f32;

  if scale == 0.0 {
    return;
  }

  let metrics = measure(text, style);

  let x = match style.alignment {
    TextAlignment::Start => bounds.x,
    TextAlignment::Center => bounds.x + (bounds.w - metrics.width) * 0.5,
    TextAlignment::End => bounds.x + bounds.w - metrics.width,
  };
  let y = bounds.y + ((bounds.h - style.size) * 0.5).max(0.0);

  for (index, character) in text.chars().enumerate() {
    let glyph_x = x + index as f32 * GLYPH_ADVANCE as f32 * scale;

    for (row, bits) in glyph(character).iter().enumerate() {
      for column in 0..GLYPH_WIDTH {
        if bits & (0b1_0000 >> column) != 0 {
          display_list.rect(
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
