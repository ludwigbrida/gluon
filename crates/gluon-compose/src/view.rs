use crate::{Composition, Content, Element};
use gluon_layout::Layout;
use gluon_style::Style;
use gluon_tree::NodeId;

#[derive(Default)]
pub struct View {
  pub layout: Layout,
  pub style: Style,
  pub children: Children,
}

pub enum Children {
  Views(Vec<View>),
  Text(String),
}

impl Default for Children {
  fn default() -> Self {
    Self::Views(Vec::new())
  }
}

impl<const N: usize> From<[View; N]> for Children {
  fn from(views: [View; N]) -> Self {
    Self::Views(views.into())
  }
}

impl FromIterator<View> for Children {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = View>,
  {
    Self::Views(iter.into_iter().collect())
  }
}

impl From<Option<View>> for Children {
  fn from(view: Option<View>) -> Self {
    Self::Views(view.into_iter().collect())
  }
}

impl From<String> for Children {
  fn from(value: String) -> Self {
    Self::Text(value)
  }
}

impl From<&str> for Children {
  fn from(value: &str) -> Self {
    Self::Text(value.into())
  }
}

impl From<View> for Composition {
  fn from(view: View) -> Self {
    view.into_composition()
  }
}

impl View {
  fn into_composition(self) -> Composition {
    let View {
      layout,
      style,
      children,
    } = self;

    let (content, children) = children.into_parts();
    let mut composition = Composition::new(Element {
      layout,
      style,
      content,
    });

    let root = composition.root();

    for child in children {
      child.mount_into(&mut composition, root);
    }

    composition
  }

  fn mount_into(self, composition: &mut Composition, parent: NodeId) {
    let View {
      layout,
      style,
      children,
    } = self;

    let (content, children) = children.into_parts();
    let node = composition
      .append_child(
        parent,
        Element {
          layout,
          style,
          content,
        },
      )
      .unwrap();

    for child in children {
      child.mount_into(composition, node);
    }
  }
}

impl Children {
  fn into_parts(self) -> (Content, Vec<View>) {
    match self {
      Self::Views(views) => (Content::Empty, views),
      Self::Text(value) => (Content::Text(value), Vec::new()),
    }
  }
}
