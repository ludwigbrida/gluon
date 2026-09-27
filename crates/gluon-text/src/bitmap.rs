use gluon_style::TextStyle;

pub const GLYPH_WIDTH: usize = 5;
pub const GLYPH_HEIGHT: usize = 7;
pub const GLYPH_ADVANCE: usize = 6;

#[derive(Default)]
pub struct TextMetrics {
  pub width: f32,
  pub height: f32,
}

pub fn measure(text: &str, style: &TextStyle) -> TextMetrics {
  let scale = style.size.0.max(0.0) / GLYPH_HEIGHT as f32;
  let count = text.chars().count();

  TextMetrics {
    width: count
      .checked_mul(GLYPH_ADVANCE)
      .and_then(|width| width.checked_sub(1))
      .unwrap_or(0) as f32
      * scale,
    height: if count == 0 {
      0.0
    } else {
      GLYPH_HEIGHT as f32 * scale
    },
  }
}

pub fn glyph(character: char) -> [u8; GLYPH_HEIGHT] {
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
    _ => [0; GLYPH_HEIGHT],
  }
}
