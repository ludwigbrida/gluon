use crate::{Rect, ScaleFactor};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
  pub physical_width: u32,
  pub physical_height: u32,
  pub scale_factor: ScaleFactor,
}

impl Viewport {
  pub const fn new(physical_width: u32, physical_height: u32, scale_factor: ScaleFactor) -> Self {
    Self {
      physical_width,
      physical_height,
      scale_factor,
    }
  }

  pub fn logical_rect(self) -> Rect {
    Rect {
      x: 0.0,
      y: 0.0,
      w: self.physical_width as f32 / self.scale_factor.0,
      h: self.physical_height as f32 / self.scale_factor.0,
    }
  }
}
