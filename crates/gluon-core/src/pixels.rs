#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pixels(pub f32);

impl Pixels {
  pub const ZERO: Self = Self(0.0);

  pub const fn new(value: f32) -> Self {
    Self(value)
  }
}
