#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Length {
  #[default]
  Content,
  Fraction(f32),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Size {
  pub width: Length,
  pub height: Length,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Constraints {
  pub max_width: f32,
  pub max_height: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ContentSize {
  pub width: f32,
  pub height: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Direction {
  #[default]
  Horizontal,
  Vertical,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum LayoutMode {
  #[default]
  Flow,
  Stack,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Alignment {
  #[default]
  Start,
  Center,
  End,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum MainAlignment {
  #[default]
  Start,
  Center,
  End,
  SpaceBetween,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct StackAlignment {
  pub horizontal: Alignment,
  pub vertical: Alignment,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Padding {
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
  pub left: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Layout {
  pub mode: LayoutMode,
  pub size: Size,
  pub direction: Direction,
  pub main_alignment: MainAlignment,
  pub cross_alignment: Alignment,
  pub stack_alignment: StackAlignment,
  pub padding: Padding,
  pub gap: f32,
}

pub trait LayoutItem {
  fn layout(&self) -> &Layout;

  fn measure_content(&self, _constraints: Constraints) -> ContentSize {
    ContentSize::default()
  }
}

impl LayoutItem for Layout {
  fn layout(&self) -> &Layout {
    self
  }
}
