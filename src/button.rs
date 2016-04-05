// extern crate piston_window;
// use piston_window::{Context, G2d};
extern crate graphics;
use piston_window::{Transformed};
use renderer::Renderer;
use graph_node::GraphNode;
use widget::Widget;

pub struct Button;

impl Widget for Button {
  fn render(renderer: Renderer, node: &GraphNode) {
     graphics::rectangle([1.0, 0.0, 0.0, 1.0], // red
            [node.x as f64, node.y as f64, node.w as f64, node.h as f64],
            renderer.context.transform, renderer.graphics);

    graphics::text([0.0, 0.0, 0.0, 1.0], // black
        22,
        "Hello world!",
        renderer.glyphs,
        renderer.context.transform.trans(node.x, node.y + 18.0),
        renderer.graphics);
    // add code here
  }
}
