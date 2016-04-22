pub mod node;

use std::cell::RefCell;
use std::boxed::Box;
use std::rc::{Rc, Weak};
// use std::collections::HashSet;

use rose_tree::{RoseTree, NodeIndex, ROOT, petgraph};

use scene_graph::node::Node;
use appearance::Appearance;
use appearance::background::Background;

// #[derive(Default)]
pub struct SceneGraph<'a> {
    pub id_counter: u32,
    // TODO - Keep track of an x,y offset for rendering, to allow multiple scene_graphs at once

    /// TODO - Enforce these conditions
    /// A tree of nodes, where a parent fully contains it's children, except those on absolute list below
    pub tree: RefCell<RoseTree<Rc<Node<'a>>>>,

    // May need to move these to a higher abstraction if it grows to big
    // TODO - Switch to a Set if possible
    // Would need to have `weak<Node>` use #[derive(PartialEq, Eq, Hash)]
    layout: RefCell<Vec<Weak<Node<'a>>>>,
    render: RefCell<Vec<Weak<Node<'a>>>>,
    style: RefCell<Vec<Weak<Node<'a>>>>,
    // absolute: RefCell<Vec<Weak<Node<'a>>>>,
    // hover: RefCell<Vec<Weak<Node<'a>>>>,
}

impl<'a> SceneGraph<'a> {
    pub fn new() -> (Self, NodeIndex) {
        let (tree, root) = RoseTree::<Rc<Node>, u32>::new(Rc::new(Node{
            id: 0,
            appearance: RefCell::new(Appearance{
                background: Some(Background::Color([1.0; 4])),
                ..Default::default()
            }),
            ..Default::default()
        }));
        (SceneGraph{
            tree:  RefCell::new(tree),
            id_counter: 1,
            layout: RefCell::new(Vec::new()),
            render: RefCell::new(Vec::new()),
            style: RefCell::new(Vec::new()),
        }, root)
    }
    pub fn add_child(&mut self, root: NodeIndex, node: Box<Node<'a>>) -> NodeIndex {
        let mut moved_node = *node;
        moved_node.id = self.id_counter;
        self.id_counter += 1;
        let rc_node = Rc::new(moved_node);
        let result = self.tree.borrow_mut().add_child(root, rc_node.clone());
        self.style(&rc_node);
        // TODO - Relies on styles calling layout at least once, but it's an informal contract
        // self.layout(&rc_node);
        result
    }
    pub fn add_child_root(&mut self, node: Box<Node<'a>>) -> NodeIndex {
        self.add_child(NodeIndex::new(ROOT), node)
    }

    pub fn layout(&self, node: &Rc<Node<'a>>) {
        self.layout.borrow_mut().push(Rc::downgrade(node));
    }
    pub fn layout_pass_required(&self) -> bool {
        !self.layout.borrow().is_empty()
    }
    pub fn temp_layout_complete(&self) {
        self.layout.borrow_mut().clear();
    }

    pub fn render(&self, node: &Rc<Node<'a>>) {
        self.render.borrow_mut().push(Rc::downgrade(node));
    }
    pub fn render_pass_required(&self) -> bool {
        !self.render.borrow().is_empty()
    }
    pub fn temp_render_complete(&self) {
        self.render.borrow_mut().clear();
    }

    pub fn style(&self, node: &Rc<Node<'a>>) {
        self.style.borrow_mut().push(Rc::downgrade(node));
    }
    pub fn style_pass_required(&self) -> bool {
        !self.style.borrow().is_empty()
    }
    pub fn style_nodes(&self) -> &RefCell<Vec<Weak<Node<'a>>>> {
        &self.style
    }


    // Functions for debugging & development
    ////////////////////////////////////////

    /// Requires Nightly unstable `rc_counts`
    pub fn debug_print_render_nodes(&self) {
        println!("\nRendering:");
        Self::debug_print_list(&self.render.borrow());
        println!("");
    }

    /// Requires Nightly unstable `rc_counts`
    pub fn debug_print_layout_nodes(&self) {
        println!("\nLaying out:");
        Self::debug_print_list(&self.layout.borrow());
        println!("");
    }

    /// Requires Nightly unstable `rc_counts`
    pub fn debug_print_ref_counts(&self) {
        println!("\nNode References:");
        let tree = self.tree.borrow();
        let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
        while let Some(node_index) = dfs.next(tree.graph()) {
            let node = &tree[node_index];
            println!("S{:?}:W{:?} - {:?}", Rc::strong_count(&node), Rc::weak_count(&node), node);
        }
        println!("");
    }

    fn debug_print_list(nodes: &Vec<Weak<Node<'a>>>) {
        for maybe_node in nodes {
            if let Some(node) = maybe_node.upgrade() {
                println!("S{:?}:W{:?} - {:?}", Rc::strong_count(&node), Rc::weak_count(&node), node);
            } else {
                println!("Weak Reference Gone...");
            }
        }
    }
}