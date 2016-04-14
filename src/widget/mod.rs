pub mod text;
pub mod div;

use renderer::Renderer;
use renderer::geometry::Geometry;
use appearance::Appearance;
use collision::HoverState;
use std::fmt::Debug;

#[derive(Default, Clone)]
pub struct State {
    pub hover_state: HoverState,
}


pub trait Widget: WidgetClone + Debug {
    fn render(&self, renderer: Renderer, appearance: &Appearance, geometry: &Geometry, state: &State);
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
