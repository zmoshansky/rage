extern crate graphics;
use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use collision::HoverState;

pub struct Div;
impl Widget for Div{
    fn render<'a>(&self, renderer: Renderer, _appearance: &Appearance, geometry: &Geometry, state: &'a State) {
        let color = if state.hover_state != HoverState::Up {[1.0, 1.0, 0.0, 1.0]} else {[0.0, 0.0, 0.0, 1.0]};

        graphics::rectangle(color,
            [geometry.position.x, geometry.position.y, geometry.dimensions.x, geometry.dimensions.y],
            renderer.context.transform, renderer.graphics);
    }
}