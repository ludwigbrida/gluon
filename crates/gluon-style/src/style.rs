use gluon_core::Color;

#[derive(Default)]
pub struct Style {
  pub background: Option<Color>,
  pub border: Border,
}

#[derive(Default)]
pub struct Border {
  pub top: Option<BorderSide>,
  pub right: Option<BorderSide>,
  pub bottom: Option<BorderSide>,
  pub left: Option<BorderSide>,
}

pub struct BorderSide {
  pub width: f32,
  pub color: Color,
}
