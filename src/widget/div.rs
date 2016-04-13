extern crate graphics;
use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use appearance::background::Background;
use collision;
// use collision::self, HoverState};
use appearance::color;

pub struct Div;
// https://www.google.com/design/spec/style/color.html#color-color-palette
fn draw(renderer: Renderer, geometry: &Geometry, color: color::Color) {
    graphics::rectangle(color,
        [geometry.position.x, geometry.position.y, geometry.dimensions.x, geometry.dimensions.y],
        renderer.context.transform, renderer.graphics);
}

impl Widget for Div{
    /// Draws if there is a background set
    /// DEBUG - Always draws if hovering
    fn render<'a>(&self, renderer: Renderer, appearance: &Appearance, geometry: &Geometry, state: &'a State) {
        if collision::over(&state.hover_state) {
            draw(renderer, geometry, color::hex("F4433660"));
        }
        else {
            if let Some(Background::Color(bg)) = appearance.background {
                draw(renderer, geometry, bg);
            }
        };
    }
}