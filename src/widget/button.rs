extern crate graphics;
use piston_window::{Transformed};
use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;

pub struct Button;
impl Widget for Button{
    fn render<'a>(&self, renderer: Renderer, appearance: &Appearance, geometry: &Geometry, state: &'a State) {
        // Determine font y-position related to size...
        // Unsure if magic numbers, specific to a font, or related to font system in rust.
        // 400:(-94..<97.249>..-100.4980)
        // 400 / 97.249 = 4.113152834

        let mut size = 20.0;
        let mut color = [0.0, 0.0, 0.0, 1.0];
        if let Some(ref font) = appearance.font {
            size = font.size;
            color = font.color;
        }

        // TODO - Font's don't render very nicely, seems partly related to sub-pixel positioning
        graphics::text(color,
            size as u32,
            state.text,
            renderer.glyphs,
            renderer.context.transform.trans(geometry.position.x, geometry.position.y + (size - (size / 4.113152834))),
            renderer.graphics);
    }
}
