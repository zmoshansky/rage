#[allow(dead_code)]
pub mod dimension;
#[allow(dead_code)]
pub mod position;
#[allow(dead_code)]
pub mod overflow;
#[allow(dead_code)]
pub mod flow;
#[allow(dead_code)]
pub mod box_model;
#[allow(dead_code)]
pub mod spacing;

use std::rc::Rc;

use rose_tree::{ROOT, NodeIndex};
use piston_window;

use scene_graph::node::Node;
use scene_graph::SceneGraph;
use renderer::{geometry, image};

/// A struct to hold constants needed for the layout engine
// TODO - Rename to LayoutArgs
pub struct Cartographer<'a> {
    pub window: &'a geometry::Xy,
    pub dpi: &'a geometry::Xy,
    pub glyphs: &'a mut piston_window::Glyphs,
    pub images: &'a image::ImageCache<'a>,
}

#[derive(Default, PartialEq, Clone)]
pub struct Layout {
    // pub overflows: overflow::Overflows,
    pub dimensions: dimension::Dimensions,
    pub position: position::Position,

    pub border: geometry::Spacing,
    pub margin: geometry::Spacing,
    pub padding: geometry::Spacing,

    // pub box_model: box_model::BoxModel,

    /// Containers
    pub flow: flow::Flow,
}

// Hybrid BFS/DFS traversal.
/// 1.) Layout(root)
/// 2.) Layout root's children
/// 3.) Layout(root's n'th child)

/// DP, VP, P Can all be handled with just parent & child node.
/// Grid & Flex require the parent and all children.
/// Wrap Needs a DFS.

/// Current Restrictions:
/// Wrap - Only valid on a primitive element

// A node `a` will have it's geometry set before traversing it's children; except if its dimension is of type `wrap`.
// TODO - Special case for wrap... Probably need to return width/height of nodes
/// Layout all of a node's children
fn layout(cartographer: &mut Cartographer, scene_graph: &SceneGraph, parent_index: NodeIndex) {
    // BFS from pet_graph doesn't work since we need to know when we're done traversing a level.
    // The Margin/Border/Padding Pass must be done before Dimensions.
    spacing::calculate(cartographer, scene_graph, parent_index);
    dimension::calculate(cartographer, scene_graph, parent_index);


    let tree = scene_graph.tree.borrow();
    let parent = &tree[parent_index];

    let mut bfs = tree.walk_children(parent_index);
    let mut child_indices = Vec::new();
    while let Some(nx) = bfs.next(&tree) {
        child_indices.push(nx);
    }
    child_indices.reverse();

    // TODO - Add dimension geometry::Xy, to account for max height,width when wrapping
    // TODO - Account for reverse flow directions
    let mut position = geometry::Xy{
        x: parent.geometry.borrow().position.x,
        y: parent.geometry.borrow().position.y
    };

    // Postion & Recursively lay out children
    // TODO - Use layout list instead of doing entire tree
    for nx in child_indices.clone() {
        position_children(&parent, &tree[nx], &mut position, scene_graph);
        // println!("layout::layout {:?} {:?}", tree[nx], tree[nx].geometry.borrow());
        layout(cartographer, scene_graph, nx);
    }
}

// TODO - Another layout method that handles only the `scenegraph.layout` nodes,
pub fn layout_root(cartographer: &mut Cartographer, scene_graph: &SceneGraph) {
    if scene_graph.layout_pass_required() {
        // DEBUG
        scene_graph.debug_print_layout_nodes();

        layout(cartographer, scene_graph, NodeIndex::new(ROOT));

        // TODO - Eventually use a drain iterator to handle this in `layout`.
        // See style::style
        scene_graph.temp_layout_complete();
    }
}

fn position_children<'a>(parent: &Rc<Node<'a>>, node: &Rc<Node<'a>>, bounding_position: &mut geometry::Xy, scene_graph: &SceneGraph<'a>) {
    match node.layout.borrow().position {
        position::Position::Relative(ref pos) => {
            set_position_x(node, parent.geometry.borrow().position.x + pos.x, scene_graph);
            set_position_y(node, parent.geometry.borrow().position.y + pos.y, scene_graph);
        }
        position::Position::Absolute(ref _pos) => {unimplemented!();}
        // TODO - Child cannot override flow yet.
        position::Position::Flow(ref _flow_self) => {
            match parent.layout.borrow().flow.direction {
                flow::Direction::Right => {
                    set_position_x(node, bounding_position.x, scene_graph);
                    set_position_y(node, bounding_position.y, scene_graph);
                    let bounds = node.geometry.borrow().bounding_dimensions();
                    bounding_position.x += bounds.x;
                }
                flow::Direction::Down => {
                    set_position_x(node, bounding_position.x, scene_graph);
                    set_position_y(node, bounding_position.y, scene_graph);
                    let bounds = node.geometry.borrow().bounding_dimensions();
                    bounding_position.y += bounds.y;
                }
                // flow::flow_right(parent, node, &mut bounding_position),
                _=> {}
            }
        }
    }
}

fn set_position_x<'a>(node: &Rc<Node<'a>>, bounding_pos_x: f64, scene_graph: &SceneGraph<'a>) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.set_bounding_position_x(bounding_pos_x) {
        scene_graph.render(node);
    }
}

fn set_position_y<'a>(node: &Rc<Node<'a>>, bounding_pos_y: f64, scene_graph: &SceneGraph<'a>) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.set_bounding_position_y(bounding_pos_y) {
        scene_graph.render(node);
    }
}
