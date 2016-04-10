extern crate rose_tree;

use geometry::dimension::{Dimensions, Dimension};
use geometry::Geometry as GeometryUncached;
use graph_node::GraphNode;

use renderer::geometry::{Geometry, Xy, Xyz};

/// A struct to hold constants needed for the layout engine
pub struct Cartographer<'a> {
    pub window: &'a Xy,
    pub dpi: &'a Xy,
}

// impl<'a> Default for Cartographer<'a> {
//     fn default() -> Cartographer<'a> {
//         Cartographer{window: &'a Xy{x: 800.0, y: 600.0}, dpi: &Xy{x:96.0, y: 96.0}}
//     }
// }

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
// TODO - Standardize geometry to account for padding/margins

/// Layout all of root's children
pub fn layout(cartographer: &Cartographer, tree: &rose_tree::RoseTree<GraphNode>, root: rose_tree::NodeIndex) {
    // BFS = FIFO Queue, DFS = Stack
    // BFS from pet_graph doesn't work since we need to know when we're done traversing a level.

    // Special case to layout tree's root
    let parent: &GraphNode = &tree[root];
    if root.index() == rose_tree::ROOT {
        set_x(parent, compute_viewport_x(&cartographer, 1.0));
        set_y(parent, compute_viewport_y(&cartographer, 1.0));
    }

    let mut bfs = tree.walk_children(root);
    let (mut sum_x, mut sum_y) = (0.0, 0.0);

    while let Some(nx) = bfs.next(tree) {
        let node = &tree[nx];

        let x_dimension_pixels: Option<f64> = match node.geometry_uncached.dimensions.x {
            Dimension::DisplayPixel(x) => Some(compute_display_pixel_x(&cartographer, x)),
            Dimension::Viewport(x) => Some(compute_viewport_x(&cartographer, x)),
            Dimension::Percent(x) => Some(compute_percent_x(parent, x)),
            Dimension::Grid(x) => {sum_x += x; None},
        };
        if let Some(x_dimension_pixels) = x_dimension_pixels {
            set_x(node, x_dimension_pixels);
        }


        let y_dimension_pixels: Option<f64> = match node.geometry_uncached.dimensions.y {
            Dimension::DisplayPixel(y) => Some(compute_display_pixel_y(&cartographer, y)),
            Dimension::Viewport(y) => Some(compute_viewport_y(&cartographer, y)),
            Dimension::Percent(y) => Some(compute_percent_y(parent, y)),
            Dimension::Grid(y) => {sum_y += y; None},
        };
        if let Some(y_dimension_pixels) = y_dimension_pixels {
            set_y(node, y_dimension_pixels);
        }

        println!("layout::layout {:?} x: {:?}, y: {:?}", node, x_dimension_pixels, y_dimension_pixels);
    }

    // Handle Grid Layouts
    if sum_x > 0.0 || sum_y > 0.0 {
        let mut bfs = tree.walk_children(root);
        while let Some(nx) = bfs.next(tree) {
            let node = &tree[nx];
            if sum_x > 0.0 {
                if let Dimension::Grid(x) = node.geometry_uncached.dimensions.x {
                    set_x(node, compute_percent_x(parent, x / sum_x));
                }
            }
            if sum_y > 0.0 {
                if let Dimension::Grid(y) = node.geometry_uncached.dimensions.y {
                    set_y(node, compute_percent_y(parent, y / sum_y));
                }
            }
        }
    }

    // Recursively lay out children
    let mut bfs = tree.walk_children(root);
    let (mut sum_x, mut sum_y) = (0.0, 0.0);

    while let Some(nx) = bfs.next(tree) {
        layout(cartographer, tree, nx);
    }
}

fn set_x(node: &GraphNode, dimension: f64) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.dimensions.x != dimension {
      geometry.dimensions.x = dimension;
      node.dirty.set(true);
    };
}
fn set_y(node: &GraphNode, dimension: f64) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.dimensions.y != dimension {
      geometry.dimensions.y = dimension;
      node.dirty.set(true);
    };
}

fn compute_percent_x(parent: &GraphNode, dimension: f64) -> f64 {dimension * parent.geometry.borrow().dimensions.x}
fn compute_percent_y(parent: &GraphNode, dimension: f64) -> f64 {dimension * parent.geometry.borrow().dimensions.y}
fn compute_viewport_x(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * cartographer.window.x}
fn compute_viewport_y(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * cartographer.window.y}
fn compute_display_pixel_x(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * (cartographer.dpi.x / 160.0)}
fn compute_display_pixel_y(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * (cartographer.dpi.y / 160.0)}
