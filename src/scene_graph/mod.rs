pub mod node;

use std::cell::RefCell;
use std::cell::Cell;
use std::boxed::Box;

use rose_tree::{RoseTree, NodeIndex, ROOT};

use scene_graph::node::Node;
use appearance::Appearance;
use appearance::background::Background;

// #[derive(Default)]
pub struct SceneGraph {
  pub tree: RefCell<RoseTree<Node>>,
  // TODO - [list|iterable map] of weak references to Nodes that are absolutely positioned.
  // pub absolute: Vec<Node,
  pub id_counter: u32,
  pub needs_layout: Cell<bool>
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
        (SceneGraph{tree:  RefCell::new(tree), id_counter: 1, needs_layout: Cell::new(true)}, root)
    }
    pub fn add_child(&mut self, root: NodeIndex, node: Box<Node>) -> NodeIndex {
        let mut moved_node = *node;
        moved_node.id = self.id_counter;
        self.id_counter += 1;
        self.tree.borrow_mut().add_child(root, moved_node)
    }
    pub fn add_child_root(&mut self, node: Box<Node>) -> NodeIndex {
        self.add_child(NodeIndex::new(ROOT), node)
    }
}