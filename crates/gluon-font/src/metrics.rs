use gluon_core::Pixels;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphMetrics {
  pub advance: Pixels,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineMetrics {
  pub ascent: Pixels,
  pub descent: Pixels,
  pub height: Pixels,
}
