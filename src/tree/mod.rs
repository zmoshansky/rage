
use rose_tree::{RoseTree, NodeIndex};
use widget::Widget;
use graph_node::GraphNode;

// #[derive(Default)]
pub struct Tree<'a> {
  pub tree: RoseTree<GraphNode<'a>>,
  pub types: Vec<Box<Widget>>,
  pub id_counter: u32,
}

impl<'a> Tree<'a> {
    pub fn new(root: GraphNode<'a>) -> (Self, NodeIndex) {
        let (tree, root) = RoseTree::<GraphNode, u32>::new(root);
        (Tree{types: Vec::new(), tree: tree, id_counter: 0}, root)
    }
    pub fn add_child(&mut self, root: NodeIndex, node: GraphNode<'a>) -> NodeIndex {
        node.id.set(self.id_counter);
        self.id_counter += 1;
        self.tree.add_child(root, node)
    }
}