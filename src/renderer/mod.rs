#[allow(dead_code)]
pub mod geometry;
#[allow(dead_code)]
pub mod appearance;

use piston_window::{Context, G2d, Glyphs};
use rose_tree::{petgraph};
use rose_tree;
use tree::Tree;

use widget::{Widget};

pub struct Renderer<'a, 'b: 'a> {
    pub context: Context,
    pub graphics: &'a mut G2d<'b>,
    pub glyphs: &'a mut Glyphs,
    // pub types: [Box<Widget>; 2],
}

pub fn render(renderer: Renderer, ui_tree: &Tree) {
    let dfs = petgraph::DfsIter::new(ui_tree.tree.graph(), petgraph::graph::NodeIndex::new(rose_tree::ROOT));
    for node_index in dfs {
        let node = &ui_tree.tree[node_index];
        // TODO - Figure out how to do occlusion, proper rendering based on z-index.
        // Must re-render every node higher than the dirty one...
        node.dirty.set(true);
        if node.dirty.get() {
            ui_tree.types[node.type_id].render(Renderer{context: renderer.context, graphics: renderer.graphics, glyphs: renderer.glyphs}, &node.appearance, &node.geometry.borrow(), &node.state);
            node.dirty.set(false);
        }
    }
}
