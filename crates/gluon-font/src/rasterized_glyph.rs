use gluon_core::Pixels;

pub struct RasterizedGlyph {
  pub left: Pixels,
  pub bottom: Pixels,
  pub width: u32,
  pub height: u32,
  pub advance: Pixels,
  pub coverage: Vec<u8>,
}
