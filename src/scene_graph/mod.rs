use rose_tree::{RoseTree, NodeIndex};
use widget::Widget;
use graph_node::GraphNode;
use std::cell::RefCell;
use std::cell::Cell;

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
        let (tree, root) = RoseTree::<GraphNode, u32>::new(GraphNode{id: Cell::new(0), type_id: 0, dirty: Cell::new(true), ..Default::default()});
        (SceneGraph{types: Vec::new(), tree:  RefCell::new(tree), id_counter: 1}, root)
    }
    pub fn add_child(&mut self, root: NodeIndex, node: GraphNode<'a>) -> NodeIndex {
        node.id.set(self.id_counter);
        self.id_counter += 1;
        self.tree.borrow_mut().add_child(root, node)
    }
}