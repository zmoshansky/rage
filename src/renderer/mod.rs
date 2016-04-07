#[allow(dead_code)]
pub mod geometry;
#[allow(dead_code)]
pub mod appearance;

use piston_window::{Context, G2d, Glyphs};
use rose_tree::{RoseTree, petgraph};
use rose_tree;
use graph_node::GraphNode;

use widget::{Widget, State};
use widget::button::{Button as WButton, Background};

pub struct Renderer<'a, 'b: 'a> {
    pub context: Context,
    pub graphics: &'a mut G2d<'b>,
    pub glyphs: &'a mut Glyphs,
    // pub types: [Box<Widget>; 2],
}

pub fn render(renderer: Renderer, tree: RoseTree<GraphNode, u32>) {

}
// impl<'a, 'b: 'a> Renderer<'a, 'b> {
//     pub fn render(&self, tree: RoseTree<GraphNode, u32>) {
//         let dfs = petgraph::DfsIter::new(tree.graph(), rose_tree::petgraph::graph::NodeIndex::new(rose_tree::ROOT));
//         for node_index in dfs {
//             let node = tree.node_weight(node_index).unwrap();
//             node.dirty.set(true);
//             if node.dirty.get() {
//                 self.types[node.type_id].render(self, &node.appearance, &node.geometry, &node.state);
//                 // types[node.type_id].render(Renderer{context: c, graphics: g, glyphs: &mut glyph_cache}, &node.appearance, &node.geometry, &node.state);
//                 node.dirty.set(false);
//             }
//         }
//     }
// }