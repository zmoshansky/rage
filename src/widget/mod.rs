pub mod button;
pub mod div;

// use graph_node::GraphNode;
use std::cell::Cell;
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;

#[derive(Default)]
pub struct State<'a> {
    pub text: &'a str,
    pub hover: Cell<bool>,
}

// TODO - Generalize State
pub trait Widget {
    fn render(&self, renderer: Renderer, appearance: &Appearance, geometry: &Geometry, state: &State);
}
