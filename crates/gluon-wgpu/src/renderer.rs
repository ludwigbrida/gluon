use gluon_core::DisplayList;
use wgpu::{CommandEncoder, Device, TextureFormat, TextureView};

pub struct Renderer;

impl Renderer {
  pub fn new(_device: &Device, _target_format: TextureFormat) -> Self {
    Self
  }

  pub fn render(
    &self,
    _display_list: &DisplayList,
    _encoder: &mut CommandEncoder,
    _target: &TextureView,
  ) {
  }
}
