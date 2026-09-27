use gluon_core::Color;
use gluon_core::Pixels;

#[derive(Default)]
pub struct Style {
  pub background: Option<Color>,
  pub border: Border,
  pub text: Option<TextStyle>,
}

#[derive(Default)]
pub struct Border {
  pub top: Option<BorderSide>,
  pub right: Option<BorderSide>,
  pub bottom: Option<BorderSide>,
  pub left: Option<BorderSide>,
}

pub struct BorderSide {
  pub width: Pixels,
  pub color: Color,
}

#[derive(Default)]
pub enum TextAlignment {
  #[default]
  Start,
  Center,
  End,
}

pub struct TextStyle {
  pub color: Color,
  pub size: Pixels,
  pub alignment: TextAlignment,
}
