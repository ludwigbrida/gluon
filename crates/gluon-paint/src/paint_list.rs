use gluon_core::{Color, Pixels, Rect};
use gluon_font::FontId;
use gluon_style::TextAlignment;

pub struct TextRun {
  pub bounds: Rect,
  pub text: String,
  pub font: FontId,
  pub color: Color,
  pub size: Pixels,
  pub alignment: TextAlignment,
}

pub enum PaintCommand {
  Rect { rect: Rect, color: Color },
  Text(TextRun),
}

#[derive(Default)]
pub struct PaintList {
  commands: Vec<PaintCommand>,
}

impl TextRun {
  pub fn start_x(&self, width: f32) -> f32 {
    match self.alignment {
      TextAlignment::Start => self.bounds.x,
      TextAlignment::Center => self.bounds.x + (self.bounds.w - width) * 0.5,
      TextAlignment::End => self.bounds.x + self.bounds.w - width,
    }
  }
}

impl PaintList {
  pub fn rect(&mut self, rect: Rect, color: Color) {
    self.commands.push(PaintCommand::Rect { rect, color });
  }

  pub fn text(&mut self, text: TextRun) {
    self.commands.push(PaintCommand::Text(text));
  }

  pub fn commands(&self) -> &[PaintCommand] {
    &self.commands
  }

  pub fn rects(&self) -> impl Iterator<Item = (&Rect, &Color)> {
    self.commands.iter().filter_map(|command| match command {
      PaintCommand::Rect { rect, color } => Some((rect, color)),
      PaintCommand::Text(_) => None,
    })
  }
}
