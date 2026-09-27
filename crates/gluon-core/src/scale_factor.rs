#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct ScaleFactor(pub f32);

impl ScaleFactor {
  pub const ONE: Self = Self(1.0);

  pub const fn new(value: f32) -> Self {
    Self(value)
  }
}
