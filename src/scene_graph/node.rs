use std::cell::Cell;
use std::cell::RefCell;
use std::fmt;

use layout;
use widget;
use renderer::geometry::Geometry;
use appearance::Appearance;

#[derive(Clone)]
pub struct Node {
    pub id: u32,
    pub state: widget::State,

    /// Layout is turned into geometry by Layout pass
    pub layout: layout::Layout,
    pub geometry: RefCell<Geometry>,

    // pub styles/appearance_uncached: StyleRules,
    pub appearance: Appearance,

    // Use Cell to allow mutable value...
    // TODO - Change this to reflect what's needed, layout pass, render pass, etc.
    pub dirty: Cell<bool>,
    pub widget: Box<widget::Widget>
}
impl Default for Node {
    fn default() -> Node {
        Node{
            id: 0,
            state: widget::State::default(),
            layout: layout::Layout::default(),
            geometry: RefCell::new(Geometry::default()),
            appearance: Appearance::default(),
            dirty: Cell::new(false),
            widget: Box::new(widget::div::Div)
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ id: {}, type: {:?} }}", self.id, self.widget)
    }
}
