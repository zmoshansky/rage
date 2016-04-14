extern crate graphics;
use piston_window::Transformed;

use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;


#[derive(Default, Clone, Debug)]
pub struct Image{
    pub path: &'static str,
}

impl Widget for Image{
    fn render<'a>(&self, renderer: &mut Renderer, _appearance: &Appearance, geometry: &Geometry, _state: &'a State) {
        // TODO - Allow loading images on the fly
        // BLOCKED - https://github.com/PistonDevelopers/piston_window/issues/123

        if let Some(meta) = renderer.images.get(self.path) {
            graphics::image(meta,
                renderer.context.transform.trans(geometry.position.x, geometry.position.y),
                renderer.graphics);
        }
    }
}
