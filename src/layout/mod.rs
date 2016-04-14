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
// TODO
#[allow(dead_code)]
// pub mod spacing;


use rose_tree::{ROOT, NodeIndex};

use scene_graph::node::Node;
use scene_graph::SceneGraph;
use renderer::geometry;

use layout::dimension::Dimension;

/// A struct to hold constants needed for the layout engine
// TODO - Rename to LayoutArgs
pub struct Cartographer<'a> {
    pub window: &'a geometry::Xy,
    pub dpi: &'a geometry::Xy,
}

#[derive(Default, Clone)]
pub struct Layout {
    pub overflows: overflow::Overflows,
    pub dimensions: dimension::Dimensions,
    pub position: position::Position,

    pub border: geometry::Spacing,
    pub margin: geometry::Spacing,
    pub padding: geometry::Spacing,

    pub box_model: box_model::BoxModel,

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

/// Overly strict criteria, If grid or flex is used... All siblings must be the same.
// This can be relaxed later at added engine complexity.

// A node `a` will have it's geometry set before traversing it's children; except if its dimension is of type `wrap`.
// TODO - Special case for wrap... Probably need to return width/height of nodes
/// Layout all of a node's children
pub fn layout(cartographer: &Cartographer, scene_graph: &SceneGraph, root: NodeIndex) {
    // BFS = FIFO Queue, DFS = Stack
    // BFS from pet_graph doesn't work since we need to know when we're done traversing a level.
    let tree = scene_graph.tree.borrow();

    // Special case to layout tree's root
    let parent: &Node = &tree[root];
    // TODO - Move special case to graph creation
    if root.index() == ROOT {
        set_dimension_x(parent, compute_viewport_x(&cartographer, 1.0));
        set_dimension_y(parent, compute_viewport_y(&cartographer, 1.0));
        set_position_x(parent, 0.0);
        set_position_y(parent, 0.0);
    }

    let mut bfs = tree.walk_children(root);
    let mut child_indices = Vec::new();
    while let Some(nx) = bfs.next(&tree) {
        child_indices.push(nx);
    }
    child_indices.reverse();
    let (mut sum_x, mut sum_y) = (0.0, 0.0);

    for nx in child_indices.clone() {
        let node = &tree[nx];
        let x_dimension_pixels: Option<f64> = match node.layout.dimensions.x {
            Dimension::DisplayPixel(x) => Some(compute_display_pixel_x(&cartographer, x)),
            Dimension::Viewport(x) => Some(compute_viewport_x(&cartographer, x)),
            Dimension::Percent(x) => Some(compute_percent_x(parent, x)),
            Dimension::Grid(x) => {sum_x += x; None},
        };
        if let Some(x_dimension_pixels) = x_dimension_pixels {
            set_dimension_x(node, x_dimension_pixels);
        }

        let y_dimension_pixels: Option<f64> = match node.layout.dimensions.y {
            Dimension::DisplayPixel(y) => Some(compute_display_pixel_y(&cartographer, y)),
            Dimension::Viewport(y) => Some(compute_viewport_y(&cartographer, y)),
            Dimension::Percent(y) => Some(compute_percent_y(parent, y)),
            Dimension::Grid(y) => {sum_y += y; None},
        };
        if let Some(y_dimension_pixels) = y_dimension_pixels {
            set_dimension_y(node, y_dimension_pixels);
        }

        // TODO - Calc and Set Border, Margins, and Padding from Layout to Geometry
        // Assuming already in pixels for now
        let mut geometry = node.geometry.borrow_mut();
        geometry.border = node.layout.border.clone();
        geometry.padding = node.layout.padding.clone();
        geometry.margin = node.layout.margin.clone();
    }

    // Handle Grid Layouts
    if sum_x > 0.0 || sum_y > 0.0 {
        for nx in child_indices.clone() {
            let node = &tree[nx];
            if sum_x > 0.0 {
                if let Dimension::Grid(x) = node.layout.dimensions.x {
                    set_dimension_x(node, compute_percent_x(parent, x / sum_x));
                }
            }
            if sum_y > 0.0 {
                if let Dimension::Grid(y) = node.layout.dimensions.y {
                    set_dimension_y(node, compute_percent_y(parent, y / sum_y));
                }
            }
        }
    }

    // TODO - Add dimension geometry::Xy, to account for max height,width when wrapping
    // TODO - Account for reverse flow directions
    let mut position = geometry::Xy{
        x: parent.geometry.borrow().position.x,
        y: parent.geometry.borrow().position.y
    };
    // Postion & Recursively lay out children
    for nx in child_indices.clone() {
        position_children(&parent, &tree[nx], &mut position);
        println!("layout::layout {:?} {:?} {}", tree[nx], tree[nx].geometry.borrow(), nx.index());
        layout(cartographer, scene_graph, nx);
    }
}

pub fn layout_root(cartographer: &Cartographer, scene_graph: &SceneGraph) {
    layout(cartographer, scene_graph, NodeIndex::new(ROOT));
}

fn position_children(parent: &Node, node: &Node, bounding_position: &mut geometry::Xy) {
    match node.layout.position {
        position::Position::Relative(ref _pos) => {}
        position::Position::Absolute(ref _pos) => {}
        // TODO - Child cannot override flow yet.
        position::Position::Flow(ref _flow_self) => {
            match parent.layout.flow.direction {
                flow::Direction::Right => {
                    set_position_x(node, bounding_position.x);
                    set_position_y(node, bounding_position.y);
                    let bounds = node.geometry.borrow().bounding_dimensions();
                    bounding_position.x += bounds.x;
                }
                flow::Direction::Down => {
                    set_position_x(node, bounding_position.x);
                    set_position_y(node, bounding_position.y);
                    let bounds = node.geometry.borrow().bounding_dimensions();
                    bounding_position.y += bounds.y;
                }
                // flow::flow_right(parent, node, &mut bounding_position),
                _=> {}
            }
        }
    }
}

fn set_position_x(node: &Node, bounding_pos_x: f64) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.set_bounding_position_x(bounding_pos_x) {
        node.dirty.set(true);
    }
}

fn set_position_y(node: &Node, bounding_pos_y: f64) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.set_bounding_position_y(bounding_pos_y) {
        node.dirty.set(true);
    }
}

// TODO - Account for box-model
fn set_dimension_x(node: &Node, dimension: f64) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.dimensions.x != dimension {
      geometry.dimensions.x = dimension;
      node.dirty.set(true);
    };
}
// TODO - Account for box-model
fn set_dimension_y(node: &Node, dimension: f64) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.dimensions.y != dimension {
      geometry.dimensions.y = dimension;
      node.dirty.set(true);
    };
}

fn compute_percent_x(parent: &Node, dimension: f64) -> f64 {dimension * (parent.geometry.borrow().dimensions.x)}
fn compute_percent_y(parent: &Node, dimension: f64) -> f64 {dimension * (parent.geometry.borrow().dimensions.y)}
fn compute_viewport_x(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * cartographer.window.x}
fn compute_viewport_y(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * cartographer.window.y}
fn compute_display_pixel_x(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * (cartographer.dpi.x / 160.0)}
fn compute_display_pixel_y(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * (cartographer.dpi.y / 160.0)}
