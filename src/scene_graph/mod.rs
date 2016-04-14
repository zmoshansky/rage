pub mod node;

use rose_tree::{RoseTree, NodeIndex, ROOT};
use scene_graph::node::Node;
use std::cell::RefCell;
use std::mem;

use appearance::Appearance;
use appearance::background::Background;

// #[derive(Default)]
pub struct SceneGraph {
  pub tree: RefCell<RoseTree<Node>>,
  // TODO - [list|iterable map] of weak references to Nodes that are absolutely positioned.
  // pub absolute: Vec<Node,
  pub id_counter: u32,
}

impl SceneGraph {
    pub fn new() -> (Self, NodeIndex) {
        let (tree, root) = RoseTree::<Node, u32>::new(Node{
          id: 0,
          appearance: Appearance{
              background: Some(Background::Color([1.0; 4])),
              ..Default::default()
          },
          ..Default::default()
        });
        (SceneGraph{tree:  RefCell::new(tree), id_counter: 1}, root)
    }
    pub fn add_child(&mut self, root: NodeIndex, node: &mut Node) -> NodeIndex {
        // TODO - Figure out if this is possible without the mem::replace
        // Don't want to return anything like `Node::default()`
        let mut moved_node = mem::replace(node, Node::default());
        moved_node.id = self.id_counter;
        self.id_counter += 1;
        self.tree.borrow_mut().add_child(root, moved_node)
    }
    pub fn add_child_root(&mut self, node: &mut Node) -> NodeIndex {
        self.add_child(NodeIndex::new(ROOT), node)
    }
}