use std::cell::Cell;
use std::cell::RefCell;
use std::fmt;

use layout;
use widget;
use renderer::geometry::Geometry;
use appearance::Appearance;

#[derive(Default, Clone)]
pub struct Node<'a> {
    pub id: u32,
    // TODO - Make the state type customizable
    pub state: widget::State<'a>,

    /// Layout is turned into geometry by Layout pass
    pub layout: layout::Layout,
    pub geometry: RefCell<Geometry>,

    // pub styles/appearance_uncached: StyleRules,
    pub appearance: Appearance,

    // Use Cell to allow mutable value...
    pub dirty: Cell<bool>,
    pub type_id: usize,
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ id: {}, type_id: {} }}", self.id, self.type_id)
    }
}
