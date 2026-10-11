use crate::ViewId;
use crate::element::Element;
use gluon_tree::{NodeId, Tree};
use std::collections::HashMap;
use std::ops::Deref;

pub struct Composition {
  tree: Tree<Element>,
  view_nodes: HashMap<ViewId, NodeId>,
}

impl Composition {
  pub fn new(root: Element) -> Self {
    let mut view_nodes = HashMap::new();
    let tree = Tree::new(root);
    let root_id = tree.root();
    if let Some(id) = tree.node(root_id).unwrap().value.id {
      view_nodes.insert(id, root_id);
    }
    Self { tree, view_nodes }
  }

  pub fn node_for_view_id(&self, id: ViewId) -> Option<NodeId> {
    self.view_nodes.get(&id).copied()
  }

  pub fn append_child(&mut self, parent: NodeId, element: Element) -> Option<NodeId> {
    let id = element.id;
    let node = self.tree.append_child(parent, element)?;
    if let Some(id) = id {
      self.view_nodes.entry(id).or_insert(node);
    }
    Some(node)
  }
}

impl Deref for Composition {
  type Target = Tree<Element>;

  fn deref(&self) -> &Self::Target {
    &self.tree
  }
}
