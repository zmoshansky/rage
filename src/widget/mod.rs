pub mod button;
pub mod div;

use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use collision::HoverState;

#[derive(Default, Clone)]
pub struct State<'a> {
    pub text: &'a str,
    pub hover_state: HoverState,
}

// TODO - Generalize State
pub trait Widget {
    fn render(&self, renderer: Renderer, appearance: &Appearance, geometry: &Geometry, state: &State);
}
