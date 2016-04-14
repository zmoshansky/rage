extern crate graphics;
use std::path::Path;
use piston_window::{Texture, TextureSettings, Flip};

use widget::{Widget, State};
use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use appearance::background::Background;
use graphics::image;


#[derive(Default, Clone, Debug)]
pub struct Image{
    pub path: &'static str,
}

impl Widget for Image{
    fn render<'a>(&self, renderer: Renderer, appearance: &Appearance, geometry: &Geometry, _state: &'a State) {
        // TODO - Figure out and send appropriate types for window/window.factory

        // let image = graphics::Image::new();
        // let texture = Texture::from_path(
        //     &mut *renderer.window.factory.borrow_mut(),
        //     Path::new(self.path),
        //     Flip::None,
        //     &TextureSettings::new()
        // ).unwrap();
        // // image.draw(&texture, default_draw_state(), renderer.context.transform, renderer.graphics);

        // graphics::image(&texture, renderer.context.transform, renderer.graphics);
    }
}
