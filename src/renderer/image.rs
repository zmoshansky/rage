extern crate gfx_device_gl;

use std::collections::HashMap;
use std::path::Path;

use piston_window;
use sdl2_window;

#[derive(Default)]
pub struct ImageCache<'a> {
    // TODO - Allow different dimensioned images
    // Useful for SVG
    pub images: HashMap<&'a str, piston_window::Texture<gfx_device_gl::Resources>>,
    // pub images: &'a mut HashMap<&'a str, Vec<ImageMeta>>,
}

impl<'a> ImageCache<'a> {
    pub fn get(&self, key: &str) -> Option<&piston_window::Texture<gfx_device_gl::Resources>> {
        self.images.get(key)
    }
    // TODO - Make generic over window backends
    pub fn load_image_from_path(&mut self, window: &piston_window::PistonWindow<(), sdl2_window::Sdl2Window>, path: &'a str) {
        if let Ok(texture) = piston_window::Texture::from_path(
            &mut *window.factory.borrow_mut(),
            Path::new(path),
            piston_window::Flip::None,
            &piston_window::TextureSettings::new()
            ) {

            self.images.insert(path, texture);
        }
    }
}
