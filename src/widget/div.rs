extern crate graphics;
use widget::{Widget, State};
use renderer::{Renderer, geometry};
use appearance;
use layout;

#[derive(Default, Clone, Debug)]
pub struct Div;

impl Widget for Div{
    fn layout(&self, _cartographer: &mut layout::Cartographer, _appearance: &appearance::Appearance) -> geometry::Xy {
        unimplemented!();
    }

    fn render<'a>(&self, _renderer: &mut Renderer, _appearance: &appearance::Appearance, _geometry: &geometry::Geometry, _state: &'a State) {}
}