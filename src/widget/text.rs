extern crate graphics;
use piston_window::{Transformed};
use widget::{Widget, State};
use appearance;
use layout;
use renderer::{self, geometry};
use graphics::character::CharacterCache;

#[derive(Default, Clone, Debug)]
pub struct Text{
    pub text: &'static str,
}

impl Widget for Text{
    fn layout(&self, cartographer: &mut layout::Cartographer, appearance: &appearance::Appearance) -> geometry::Xy {
        let mut size = 20.0;
         if let Some(ref font) = appearance.font {
            size = font.size;
        }

        geometry::Xy{
            x: cartographer.glyphs.width(size as u32, self.text),
            y: size
        }
    }

    fn render<'a>(&self, renderer: &mut renderer::Renderer, appearance: &appearance::Appearance, geometry: &geometry::Geometry, _state: &'a State) {
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
            self.text,
            renderer.glyphs,
            renderer.context.transform.trans(geometry.position.x, geometry.position.y + (size - (size / 4.113152834))),
            renderer.graphics);
    }
}
