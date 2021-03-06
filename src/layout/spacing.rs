use scene_graph::SceneGraph;
use renderer::geometry;
use rose_tree::{ROOT, NodeIndex};
use layout::Cartographer;
use layout::dimension;

// TODO - Values should be of dimension type, otherwise, just use renderer::geometry
// #[derive(Default, Clone)]
// pub struct Spacing {
//     pub top: f64,
//     pub bottom: f64,
//     pub left: f64,
//     pub right: f64,
//     // Z-TODO
//     // pub front: f64,
//     // pub back: f64,
// }

pub fn calculate(cartographer: &mut Cartographer, scene_graph: &SceneGraph, parent_index: NodeIndex) {
    // Special case for tree's root
    if parent_index.index() == ROOT {
        let mut tree = scene_graph.tree.borrow_mut();
        let parent = &mut tree[parent_index];
        *parent.geometry.borrow_mut() = geometry::Geometry::default();
    }

    let tree = scene_graph.tree.borrow();
    let mut bfs = tree.walk_children(parent_index);

    // TODO - Assuming display pixels
    while let Some(nx) = bfs.next(&tree) {
        let node = &tree[nx];
        let mut geometry = node.geometry.borrow_mut();

        geometry.border.left = dimension::compute_display_pixel_x(&cartographer, node.layout.borrow().border.left);
        geometry.padding.left = dimension::compute_display_pixel_x(&cartographer, node.layout.borrow().padding.left);
        geometry.margin.left = dimension::compute_display_pixel_x(&cartographer, node.layout.borrow().margin.left);
        geometry.border.right = dimension::compute_display_pixel_x(&cartographer, node.layout.borrow().border.right);
        geometry.padding.right = dimension::compute_display_pixel_x(&cartographer, node.layout.borrow().padding.right);
        geometry.margin.right = dimension::compute_display_pixel_x(&cartographer, node.layout.borrow().margin.right);

        geometry.border.top = dimension::compute_display_pixel_y(&cartographer, node.layout.borrow().border.top);
        geometry.padding.top = dimension::compute_display_pixel_y(&cartographer, node.layout.borrow().padding.top);
        geometry.margin.top = dimension::compute_display_pixel_y(&cartographer, node.layout.borrow().margin.top);
        geometry.border.bottom = dimension::compute_display_pixel_y(&cartographer, node.layout.borrow().border.bottom);
        geometry.padding.bottom = dimension::compute_display_pixel_y(&cartographer, node.layout.borrow().padding.bottom);
        geometry.margin.bottom = dimension::compute_display_pixel_y(&cartographer, node.layout.borrow().margin.bottom);
    }
}