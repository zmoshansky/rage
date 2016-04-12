
use rose_tree::{RoseTree, NodeIndex};
use widget::Widget;
use graph_node::GraphNode;
use std::cell::RefCell;

// #[derive(Default)]
pub struct Tree<'a> {
  // pub tree: RoseTree<GraphNode<'a>>,
  pub tree: RefCell<RoseTree<GraphNode<'a>>>,
  // TODO - Remove types from here...Unless renderer and layout need it
  pub types: Vec<Box<Widget>>,
  // TODO - [list|iterable map] of weak references to Nodes that are absolutely positioned.
  // pub absolute: Vec<GraphNode<'a>,
  pub id_counter: u32,
}

impl<'a> Tree<'a> {
    pub fn new(root: GraphNode<'a>) -> (Self, NodeIndex) {
        let (tree, root) = RoseTree::<GraphNode, u32>::new(root);
        (Tree{types: Vec::new(), tree:  RefCell::new(tree), id_counter: 0}, root)
    }
    pub fn add_child(&mut self, root: NodeIndex, node: GraphNode<'a>) -> NodeIndex {
        node.id.set(self.id_counter);
        self.id_counter += 1;
        self.tree.borrow_mut().add_child(root, node)
    }
    // pub fn tree_mut(&mut self) -> &mut RoseTree<GraphNode<'a>> {
    //   &mut self.tree
    // }
}