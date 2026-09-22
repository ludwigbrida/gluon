use gluon::DisplayList;
use wgpu::Device;

pub struct Renderer;

impl Renderer {
  pub fn new(_device: &Device) -> Self {
    Self
  }

  pub fn render(&self, _display_list: &DisplayList) {}
}
