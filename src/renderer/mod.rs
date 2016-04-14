#[allow(dead_code)]
pub mod geometry;

use piston_window::{Context, G2d, Glyphs};
use rose_tree::{ROOT, petgraph};
use scene_graph::SceneGraph;

use widget::{Widget};

pub struct Renderer<'a, 'b: 'a> {
    pub context: Context,
    pub graphics: &'a mut G2d<'b>,
    pub glyphs: &'a mut Glyphs,
    // pub types: [Box<Widget>; 2],
}

pub fn render(renderer: Renderer, scene_graph: &SceneGraph) {
    let tree = scene_graph.tree.borrow();
    let dfs = petgraph::DfsIter::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    for node_index in dfs {
        let node = &tree[node_index];
        // TODO - Figure out occlusion, proper rendering based on z-index, handling opacity.
        // Intermediate Optimization - Set dirty for entire tree.
        // Must re-render every node higher than the dirty one...
        node.dirty.set(true);
        if node.dirty.get() {
            scene_graph.types[node.type_id].render(Renderer{context: renderer.context, graphics: renderer.graphics, glyphs: renderer.glyphs}, &node.appearance, &node.geometry.borrow(), &node.state);
            node.dirty.set(false);
        }
    }
}
