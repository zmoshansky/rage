extern crate graphics;
use piston_window::{Transformed};
use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use graphics::character::CharacterCache;

pub struct Button;
impl Widget for Button{
    fn render<'a>(&self, renderer: Renderer, _appearance: &Appearance, geometry: &Geometry, state: &'a State) {
        let font_size = 20;
        let black = [0.0, 0.0, 0.0, 1.0];
        let red = [1.0, 0.0, 0.0, 1.0];
        // Unsure if magic numbers or legit related to font system in rust.
        // 400:(-94..<97.249>..-100.4980)
        // 400 / 97.249 = 4.113152834

        graphics::rectangle(red,
            [geometry.position.x, geometry.position.y, renderer.glyphs.width(font_size, state.text), font_size as f64],
            renderer.context.transform, renderer.graphics);

        // TODO - Get font size/Color from _appearance
        graphics::text(black,
            font_size,
            state.text,
            renderer.glyphs,
            renderer.context.transform.trans(geometry.position.x, geometry.position.y + (font_size as f64 - (font_size as f64 / 4.113152834))),
            renderer.graphics);
    }
}

pub struct Background;
impl Widget for Background{
    fn render<'a>(&self, renderer: Renderer, _appearance: &Appearance, _geometry: &Geometry, _state: &'a State) {
        graphics::clear([1.0; 4], renderer.graphics);
    }
}
