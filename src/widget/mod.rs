pub mod text;
pub mod div;
pub mod image;

use renderer::Renderer;
use renderer::geometry;
use layout;
use appearance;
use collision::HoverState;
use std::fmt::Debug;

#[derive(Default, Clone, Debug)]
pub struct State<'a> {
    pub id: &'a str,
    pub hover_state: HoverState,
}


pub trait Widget: WidgetClone + Debug {
    fn layout(&self, cartographer: &mut layout::Cartographer, appearance: &appearance::Appearance) -> geometry::Xy;
    fn render(&self, renderer: &mut Renderer, appearance: &appearance::Appearance, geometry: &geometry::Geometry, state: &State);
}

// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-trait-object
// Pure magic
pub trait WidgetClone {
    fn clone_box(&self) -> Box<Widget>;
}

impl<T> WidgetClone for T where T: 'static + Widget + Clone {
    fn clone_box(&self) -> Box<Widget> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<Widget> {
    fn clone(&self) -> Box<Widget> {
        self.clone_box()
    }
}
