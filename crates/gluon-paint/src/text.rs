use gluon_core::{DisplayList, Rect};
use gluon_style::TextStyle;

// TODO: replace this, it's an extremely stupid temporary approach

pub fn paint_text(display_list: &mut DisplayList, bounds: &Rect, text: &str, style: &TextStyle) {
  let scale = style.size.max(0.0) / 7.0;

  if scale == 0.0 {
    return;
  }

  let x = bounds.x;
  let y = bounds.y + ((bounds.h - style.size) * 0.5).max(0.0);

  for (index, character) in text.chars().enumerate() {
    let glyph_x = x + index as f32 * 6.0 * scale;

    for (row, bits) in glyph(character).iter().enumerate() {
      for column in 0..5 {
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

fn glyph(character: char) -> [u8; 7] {
  match character {
    '0' => [
      0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
    ],
    '1' => [
      0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
    ],
    '2' => [
      0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
    ],
    '3' => [
      0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
    ],
    '4' => [
      0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
    ],
    '5' => [
      0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
    ],
    '6' => [
      0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
    ],
    '7' => [
      0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
    ],
    '8' => [
      0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
    ],
    '9' => [
      0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b11100,
    ],
    'F' => [
      0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
    ],
    'P' => [
      0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
    ],
    'S' => [
      0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
    ],
    _ => [0; 7],
  }
}
