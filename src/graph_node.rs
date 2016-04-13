use std::cell::Cell;
use std::cell::RefCell;
use std::fmt;

use geometry::Geometry as GeometryUncached;
use renderer::geometry::Geometry;
use appearance::Appearance;
use widget::State;

#[derive(Default, Clone)]
pub struct GraphNode<'a> {
    pub id: u32,
    // TODO - Make the state type customizable
    pub state: State<'a>,
    pub geometry_uncached: GeometryUncached,
    pub geometry: RefCell<Geometry>,

    // pub styles/appearance_uncached: StyleRules,
    pub appearance: Appearance,

    // Use Cell to allow mutable value...
    pub dirty: Cell<bool>,
    pub type_id: usize,
}

impl<'a> fmt::Debug for GraphNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ id: {}, type_id: {} }}", self.id, self.type_id)
    }
}
