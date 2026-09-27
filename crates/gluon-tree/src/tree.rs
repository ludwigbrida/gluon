use crate::node::Node;
use crate::node_id::NodeId;

#[derive(Debug)]
pub struct Tree<T> {
  nodes: Vec<Node<T>>,
  root: NodeId,
}

impl<T> Tree<T> {
  pub fn new(root: T) -> Self {
    Self {
      nodes: vec![Node {
        value: root,
        parent: None,
        children: Vec::new(),
      }],
      root: NodeId(0),
    }
  }

  pub fn root(&self) -> NodeId {
    self.root
  }

  pub fn node(&self, id: NodeId) -> Option<&Node<T>> {
    self.nodes.get(id.0)
  }

  pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
    self.nodes.get_mut(id.0)
  }

  pub fn append_child(&mut self, parent: NodeId, value: T) -> Option<NodeId> {
    let child = NodeId(self.nodes.len());

    self.nodes.get(parent.0)?;

    self.nodes.push(Node {
      value,
      parent: Some(parent),
      children: Vec::new(),
    });

    self.nodes[parent.0].children.push(child);

    Some(child)
  }
}
