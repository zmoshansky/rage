extern crate graphics;
use piston_window::{Transformed};
use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use appearance::background::Background;

#[derive(Default, Clone, Debug)]
pub struct Text{
    pub text: &'static str,
}

impl Widget for Text{
    fn render<'a>(&self, renderer: &mut Renderer, appearance: &Appearance, geometry: &Geometry, _state: &'a State) {
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

        if let Some(Background::Color(bg)) = appearance.background {
            graphics::rectangle(bg,
                geometry.border_box(),
                renderer.context.transform, renderer.graphics);
        }

        // TODO - Font's don't render very nicely, seems partly related to sub-pixel positioning
        graphics::text(color,
            size as u32,
            self.text,
            renderer.glyphs,
            renderer.context.transform.trans(geometry.position.x, geometry.position.y + (size - (size / 4.113152834))),
            renderer.graphics);
    }
}
