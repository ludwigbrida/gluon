use bytemuck::{Pod, Zeroable};
use gluon_core::{Color, Rect};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RectUniform {
  pub rect: [f32; 4],
  pub color: [f32; 4],
  pub viewport: [f32; 2],
  pub padding: [f32; 2],
}

impl RectUniform {
  pub fn from_rect(rect: &Rect, color: &Color, width: u32, height: u32) -> Self {
    Self {
      rect: [rect.x, rect.y, rect.w, rect.h],
      color: [color.r, color.g, color.b, color.a],
      viewport: [width as f32, height as f32],
      padding: [0.0, 0.0],
    }
  }
}
