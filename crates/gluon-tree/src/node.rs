use crate::node_id::NodeId;

#[derive(Debug)]
pub struct Node<T> {
  pub value: T,
  parent: Option<NodeId>,
  children: Vec<NodeId>,
}

impl<T> Node<T> {
  pub fn parent(&self) -> Option<NodeId> {
    self.parent
  }

  pub fn children(&self) -> &[NodeId] {
    &self.children
  }
}
