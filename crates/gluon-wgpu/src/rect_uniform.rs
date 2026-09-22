use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RectUniform {
  pub rect: [f32; 4],
  pub color: [f32; 4],
  pub viewport: [f32; 2],
  pub padding: [f32; 2],
}
