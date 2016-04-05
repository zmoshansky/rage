extern crate piston_window;
extern crate graphics;
extern crate rose_tree;
use std::path::Path;

// use piston_window::*;
use piston_window::{PistonWindow, WindowSettings, Glyphs, Transformed};


mod graph_node;
mod button;
mod widget;
mod renderer;

use graph_node::GraphNode;
use button::Button;
use renderer::Renderer;
use widget::Widget;

struct WidgetType {
  render: fn(render: Renderer, node: &GraphNode) -> (),
  // render: fn(c: Context, g: &mut G2d, node: &GraphNode) -> (),
}

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
fn default_node() -> GraphNode {GraphNode{w: 10, h: 10, type_id: 1, ..Default::default()}}

fn main() {
    let window: PistonWindow =
        WindowSettings::new("Rage", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    // let types = vec![Button{}];
    let types = vec![WidgetType{render: render_type0}, WidgetType{render: render_type1}];

    // Actual Layout
    let (mut tree, root) = rose_tree::RoseTree::<GraphNode, u32>::new(GraphNode {w: WIDTH, h: HEIGHT, type_id: 0, ..Default::default()});
    tree.add_child(root, GraphNode{x: 0.0, y: 0.0, ..default_node()});
    tree.add_child(root, GraphNode{x: 150.0, y: 10.0, ..default_node()});
    tree.add_child(root, GraphNode{x: 150.0, y: 150.0, w: 50, h: 100, ..default_node()});

    // Load text
    // let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let font_path = Path::new("assets/fonts/NotoSans/NotoSans-Regular.ttf");
    let mut glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone()).unwrap();

    for e in window {
        let dfs = rose_tree::petgraph::DfsIter::new(tree.graph(), root);

        e.draw_2d(|c, g| {
            for node_index in dfs {
                let node = tree.node_weight(node_index).unwrap();
                node.dirty.set(true);
                if node.dirty.get() {
                    ((types[node.type_id]).render)(Renderer{context: c, graphics: g, glyphs: &mut glyph_cache}, node);
                    node.dirty.set(false);
                }
            }
        });
    }
}

fn render_type0(renderer: Renderer, _node: &GraphNode) -> () {graphics::clear([1.0; 4], renderer.graphics);}
fn render_type1(renderer: Renderer, node: &GraphNode) -> () {
    graphics::rectangle([1.0, 0.0, 0.0, 1.0], // red
            [node.x as f64, node.y as f64, node.w as f64, node.h as f64],
            renderer.context.transform, renderer.graphics);

    graphics::text([0.0, 0.0, 0.0, 1.0], // black
        22,
        "Hello world!",
        renderer.glyphs,
        renderer.context.transform.trans(node.x, node.y + 18.0),
        renderer.graphics);
}