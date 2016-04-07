pub mod button;

// use graph_node::GraphNode;
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;

#[derive(Default)]
pub struct State<'a> {
    pub text: &'a str
}
pub trait Widget {
    fn render(&self, renderer: Renderer, appearance: &Appearance, geometry: &Geometry, state: &State);
}
