extern crate graphics;
use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use appearance::background::Background;
// use collision;
use appearance::color;

#[derive(Default, Clone, Debug)]
pub struct Div;

fn draw(renderer: &mut Renderer, geometry: &Geometry, color: color::Color) {
    // Background
    graphics::rectangle(color,
        geometry.border_box(),
        renderer.context.transform, renderer.graphics);
}

impl Widget for Div{
    /// Draws if there is a background set
    /// DEBUG - Always draws if hovering
    fn render<'a>(&self, renderer: &mut Renderer, appearance: &Appearance, geometry: &Geometry, _state: &'a State) {
        // if collision::over(&state.hover_state) {
        //     draw(renderer, geometry, color::hex("F4433660"));
        // }
        // else {
            if let Some(Background::Color(bg)) = appearance.background {
                draw(renderer, geometry, bg);
            }
        // };
    }
}