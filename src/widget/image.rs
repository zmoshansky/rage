extern crate graphics;
extern crate gfx_texture;

use piston_window::Transformed;
use self::gfx_texture::ImageSize;

use widget::{Widget, State};
use renderer::Renderer;
use appearance;
use layout;
use renderer::geometry;

#[derive(Default, Clone, Debug)]
pub struct Image{
    pub path: &'static str,
}

impl Widget for Image{
    fn layout(&self, cartographer: &mut layout::Cartographer, _appearance: &appearance::Appearance) -> geometry::Xy {
        if let Some(texture) = cartographer.images.get(self.path) {
            let (x, y) = texture.get_size();
            geometry::Xy{x: x as f64, y: y as f64}
        } else {
            geometry::Xy::default()
        }
    }

    fn render<'a>(&self, renderer: &mut Renderer, _appearance: &appearance::Appearance, geometry: &geometry::Geometry, _state: &'a State) {
        // TODO - Allow loading images on the fly
        // BLOCKED - https://github.com/PistonDevelopers/piston_window/issues/123

        // TODO - Have settings to specify how image is scaled

        if let Some(texture) = renderer.images.get(self.path) {
            let (x, y) = texture.get_size();

            graphics::image(texture,
                renderer.context.transform.trans(geometry.position.x, geometry.position.y).scale(geometry.dimensions.x / x as f64, geometry.dimensions.y / y as f64),
                renderer.graphics);
        }
    }
}
