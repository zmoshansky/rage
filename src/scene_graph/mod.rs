use rose_tree::{RoseTree, NodeIndex, ROOT};
use widget::Widget;
use graph_node::GraphNode;
use std::cell::RefCell;
use std::mem;

use appearance::Appearance;
use appearance::background::Background;

// #[derive(Default)]
pub struct SceneGraph<'a> {
  pub tree: RefCell<RoseTree<GraphNode<'a>>>,
  // TODO - Remove types from here...Unless renderer and layout need it
  pub types: Vec<Box<Widget>>,
  // TODO - [list|iterable map] of weak references to Nodes that are absolutely positioned.
  // pub absolute: Vec<GraphNode<'a>,
  pub id_counter: u32,
}

impl<'a> SceneGraph<'a> {
    pub fn new() -> (Self, NodeIndex) {
        let (tree, root) = RoseTree::<GraphNode, u32>::new(GraphNode{
          id: 0,
          type_id: 0,
          appearance: Appearance{
              background: Some(Background::Color([1.0; 4])),
              ..Default::default()
          },
          ..Default::default()
        });
        (SceneGraph{types: Vec::new(), tree:  RefCell::new(tree), id_counter: 1}, root)
    }
    pub fn add_child(&mut self, root: NodeIndex, node: &mut GraphNode<'a>) -> NodeIndex {
        // TODO - Figure out if this is possible without the mem::replace
        let mut moved_node = mem::replace(node, GraphNode::default());
        moved_node.id = self.id_counter;
        self.id_counter += 1;
        self.tree.borrow_mut().add_child(root, moved_node)
    }
    pub fn add_child_root(&mut self, node: &mut GraphNode<'a>) -> NodeIndex {
        self.add_child(NodeIndex::new(ROOT), node)
    }
}