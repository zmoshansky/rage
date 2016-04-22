use std::rc::Rc;

use scene_graph::node::Node;
use scene_graph::SceneGraph;
use renderer::geometry;
use rose_tree::{ROOT, NodeIndex};
use layout::{Cartographer, position};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Dimensions {
    pub x: Dimension,
    pub y: Dimension,
    // Z-TODO
    // pub z: Dimension,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Dimension {
    /// An exact mapping to a pixel
    // Unsure if this will be permanent
    // Pixel(f64),

    /// px = dp * (dpi / 160)
    DisplayPixel(f64),

    /// Percent of viewport (Easy to resolve into a width/height/)
    Viewport(f64),

    /// Percent of parent's content-box
    Percent(f64),

    /// Each Grid unit is the same size
    /// An automatic way to do percentages (Where 1 grid = parent_x / sum(grid))
    Grid(f64),

    // Chilren take their "minimum space"; Then the remaining space is treated like a grid.
    Flex(f64),

    // Just as big as item needs to be to contain children;
    // If the widget is primitive, it runs it's layout method.
    // Else, Returns 0 if it isn't a bounded quantity (ex. all children are percentage based.)
    Wrap,
}
impl Default for Dimension {fn default() -> Dimension { Dimension::Percent(1.0) }}
// impl Default for Dimension {fn default() -> Dimension { Dimension::Flex(1.0) }}

pub fn calculate(cartographer: &mut Cartographer, scene_graph: &SceneGraph, parent_index: NodeIndex) {
    let tree = scene_graph.tree.borrow();
    let parent = &tree[parent_index];

    // Special case to layout tree's root
    if parent_index.index() == ROOT {
        set_dimension_x(parent, compute_viewport_x(&cartographer, 1.0), scene_graph);
        set_dimension_y(parent, compute_viewport_y(&cartographer, 1.0), scene_graph);
    }

    // TODO - Doesn't account for overflow
    let mut flex_units = geometry::Xy::default();
    let parent_dimensions = parent.geometry.borrow().dimensions.clone();
    let mut free_space_dimensions = geometry::Xy{
        x: parent_dimensions.x,
        y: parent_dimensions.y,
    };

    let mut bfs = tree.walk_children(parent_index);
    while let Some(nx) = bfs.next(&tree) {
        let node = &tree[nx];
        let x_dimension_pixels: Option<f64> = match node.layout.borrow().dimensions.x {
            Dimension::DisplayPixel(x) => Some(compute_display_pixel_x(&cartographer, x)),
            Dimension::Viewport(x) => Some(compute_viewport_x(&cartographer, x)),
            Dimension::Percent(x) => Some(compute_percent(parent_dimensions.x, x)),
            Dimension::Wrap => Some(node.widget.layout(cartographer, &node.appearance.borrow()).x),
            Dimension::Grid(x) => {flex_units.x += x; None},
            Dimension::Flex(x) => {
                flex_units.x += x;
                Some(node.widget.layout(cartographer, &node.appearance.borrow()).x)
            },
        };
        if let Some(x_dimension_pixels) = x_dimension_pixels {
            set_dimension_x(node, x_dimension_pixels, scene_graph);

            // Only flowed items take up space from free_space_dimensions.
            if let position::Position::Flow(_) = node.layout.borrow().position {
                free_space_dimensions.x -= node.geometry.borrow().bounding_dimensions().x;
            }
        }

        let y_dimension_pixels: Option<f64> = match node.layout.borrow().dimensions.y {
            Dimension::DisplayPixel(y) => Some(compute_display_pixel_y(&cartographer, y)),
            Dimension::Viewport(y) => Some(compute_viewport_y(&cartographer, y)),
            Dimension::Percent(y) => Some(compute_percent(parent_dimensions.y, y)),
            Dimension::Wrap => Some(node.widget.layout(cartographer, &node.appearance.borrow()).y),
            Dimension::Grid(y) => {flex_units.y += y; None},
            Dimension::Flex(y) => {
                flex_units.y += y;
                Some(node.widget.layout(cartographer, &node.appearance.borrow()).y)
            },
        };
        if let Some(y_dimension_pixels) = y_dimension_pixels {
            set_dimension_y(node, y_dimension_pixels, scene_graph);

            // Only flowed items take up space from free_space_dimensions.
            if let position::Position::Flow(_) = node.layout.borrow().position {
                free_space_dimensions.y -= node.geometry.borrow().bounding_dimensions().y;
            }
        }
    }

    // Handles Flex Units in second pass
    if flex_units.x > 0.0 || flex_units.y > 0.0 {
        if free_space_dimensions.x < 0.0 {free_space_dimensions.x = 0.0}
        if free_space_dimensions.y < 0.0 {free_space_dimensions.y = 0.0}

        // TODO - Free space = (parent's content-box) - (childrens' bounding-box)
        // However, below we divide it and assign it to childrens' content-box, instead of bounding-box.

        let mut bfs = tree.walk_children(parent_index);
        while let Some(nx) = bfs.next(&tree) {
            let node = &tree[nx];
            if flex_units.x > 0.0 {
                match node.layout.borrow().dimensions.x {
                    Dimension::Grid(x) => {set_dimension_x(node, compute_percent(free_space_dimensions.x, x / flex_units.x), scene_graph)},
                    Dimension::Flex(x) => {
                        let existing_x = node.geometry.borrow().dimensions.x;
                        set_dimension_x(node, existing_x + compute_percent(free_space_dimensions.x, x / flex_units.x), scene_graph)
                    }
                    _ => {},
                }
            }
            if flex_units.y > 0.0 {
                match node.layout.borrow().dimensions.y {
                    Dimension::Grid(y) => {set_dimension_y(node, compute_percent(free_space_dimensions.y, y / flex_units.y), scene_graph)},
                    Dimension::Flex(y) => {
                        let existing_y = node.geometry.borrow().dimensions.y;
                        set_dimension_y(node, existing_y + compute_percent(free_space_dimensions.y, y / flex_units.y), scene_graph)
                    }
                    _ => {},
                }
            }
        }
    }
}

// TODO - Account for box-model
// TODO - Dirty checking here is useless for flex since we first set its dimension based on wrap, then add any leftover space in a second pass.
fn set_dimension_x<'a>(node: &Rc<Node<'a>>, dimension: f64, scene_graph: &SceneGraph<'a>) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.dimensions.x != dimension {
      geometry.dimensions.x = dimension;
      scene_graph.render(node);
    };
}
// TODO - Account for box-model
fn set_dimension_y<'a>(node: &Rc<Node<'a>>, dimension: f64, scene_graph: &SceneGraph<'a>) {
    let mut geometry = node.geometry.borrow_mut();
    if geometry.dimensions.y != dimension {
      geometry.dimensions.y = dimension;
      scene_graph.render(node);
    };
}

fn compute_percent(parent_dimension: f64, dimension: f64) -> f64 {dimension * parent_dimension}
fn compute_viewport_x(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * cartographer.window.x}
fn compute_viewport_y(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * cartographer.window.y}
pub fn compute_display_pixel_x(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * (cartographer.dpi.x / 160.0)}
pub fn compute_display_pixel_y(cartographer: &Cartographer, dimension: f64) -> f64 {dimension * (cartographer.dpi.y / 160.0)}
