extern crate graphics;

#[allow(dead_code)]
pub mod geometry;
pub mod image;

use piston_window::{Context, G2d, Glyphs};
use rose_tree::{ROOT, petgraph};

use scene_graph::SceneGraph;
use appearance::background;


use widget::{Widget};

pub struct Renderer<'a, 'b: 'a> {
    pub context: Context,
    pub graphics: &'a mut G2d<'b>,
    pub glyphs: &'a mut Glyphs,
    pub images: &'a image::ImageCache<'b>,
}

pub fn render(renderer: &mut Renderer, scene_graph: &SceneGraph) {
    let tree = scene_graph.tree.borrow();
    let dfs = petgraph::DfsIter::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    for node_index in dfs {
        let node = &tree[node_index];
        // TODO - Figure out occlusion, proper rendering based on z-index, handling opacity.
        // Intermediate Optimization - Set dirty for entire tree.
        // Must re-render every node higher (z-axis) than the dirty one...
        node.dirty.set(true);
        if node.dirty.get() {


            // TODO - Only renders proper if Background || Border + Background.
            // Render Border
            if let Some(color) = node.appearance.border {
                graphics::rectangle(color,
                    node.geometry.borrow().border_box(),
                    renderer.context.transform, renderer.graphics);
            }

            // Render Background
            if let Some(background::Background::Color(color)) = node.appearance.background {
                graphics::rectangle(color,
                    node.geometry.borrow().padding_box(),
                    renderer.context.transform, renderer.graphics);
            }

            node.widget.render(renderer, &node.appearance, &node.geometry.borrow(), &node.state);
            node.dirty.set(false);
        }
    }
}
