use crate::{Composition, Content, Element};
use gluon_layout::Layout;
use gluon_style::Style;
use gluon_tree::NodeId;

pub struct View {
  pub layout: Layout,
  pub style: Style,
  pub children: Children,
}

#[derive(Default)]
pub struct Children(Vec<View>);

impl<const N: usize> From<[View; N]> for Children {
  fn from(views: [View; N]) -> Self {
    Self(views.into())
  }
}

impl View {
  // todo: make crate-public
  pub fn into_composition(self) -> Composition {
    let View {
      layout,
      style,
      children,
    } = self;

    let mut composition = Composition::new(Element {
      layout,
      style,
      content: Content::Empty,
    });

    let root = composition.root();

    for child in children.0 {
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

    let node = composition
      .append_child(
        parent,
        Element {
          layout,
          style,
          content: Content::Empty,
        },
      )
      .unwrap();

    for child in children.0 {
      child.mount_into(composition, node);
    }
  }
}
