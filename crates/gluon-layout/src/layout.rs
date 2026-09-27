#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Length {
  #[default]
  Content,
  Pixels(f32),
  Fill,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Size {
  pub width: Length,
  pub height: Length,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Position {
  #[default]
  Flow,
  Absolute {
    top: Option<f32>,
    right: Option<f32>,
    bottom: Option<f32>,
    left: Option<f32>,
  },
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Direction {
  #[default]
  Horizontal,
  Vertical,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Layout {
  pub size: Size,
  pub position: Position,
  pub direction: Direction,
}

pub trait LayoutItem {
  fn layout(&self) -> &Layout;
}

impl LayoutItem for Layout {
  fn layout(&self) -> &Layout {
    self
  }
}
