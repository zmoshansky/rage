extern crate graphics;

#[allow(dead_code)]
pub mod geometry;
pub mod image;

use piston_window::{Context, G2d, Glyphs};
use rose_tree::{ROOT, petgraph};

use scene_graph::{SceneGraph, node};
use appearance::background;


use widget::{Widget};

pub struct Renderer<'a, 'b: 'a> {
    pub context: Context,
    pub graphics: &'a mut G2d<'b>,
    pub glyphs: &'a mut Glyphs,
    pub images: &'a image::ImageCache<'b>,
}

pub fn render(renderer: &mut Renderer, scene_graph: &SceneGraph) {
    // TODO - Pre-Render pass to determine occlusion, proper rendering based on z-index, handling opacity, re-rendering higher (z-axis) nodes.
    if scene_graph.render_pass_required() {
        scene_graph.debug_print_render_nodes();

        // TODO - Use render list instead of doing entire tree
        // See style::style
        let tree = scene_graph.tree.borrow();
        let dfs = petgraph::DfsIter::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
        for node_index in dfs {
            let node = &tree[node_index];
            render_node(renderer, node);
        }
        scene_graph.temp_render_complete();
    }
}

fn render_node<'a>(renderer: &mut Renderer, node: &node::Node) {
    // TODO - Only renders proper if Background || Border + Background.
    // Render Border
    if let Some(color) = node.appearance.borrow().border {
        graphics::rectangle(color,
            node.geometry.borrow().border_box(),
            renderer.context.transform, renderer.graphics);
    }

    // Render Background
    if let Some(background::Background::Color(color)) = node.appearance.borrow().background {
        graphics::rectangle(color,
            node.geometry.borrow().padding_box(),
            renderer.context.transform, renderer.graphics);
    }

    node.widget.render(renderer, &node.appearance.borrow(), &node.geometry.borrow(), &node.state.borrow());
}