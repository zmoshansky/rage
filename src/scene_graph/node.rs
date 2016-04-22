use std::cell::RefCell;
use std::fmt;

use appearance;
use layout;
use renderer::geometry::Geometry;
use style;
use widget;
use event;

#[derive(Clone)]
pub struct Node<'a> {
    pub id: u32,
    pub state: RefCell<widget::State<'a>>,

    /// Styles are turned into layout and appearance in style pass
    pub style_rules: Vec<style::Rule>,

    /// Layout is turned into geometry by layout pass
    pub layout:  RefCell<layout::Layout>,
    pub geometry: RefCell<Geometry>,

    // pub styles/appearance_uncached: StyleRules,
    pub appearance:  RefCell<appearance::Appearance>,
    pub event_handlers: Vec<event::EventHandler>,
    pub widget: Box<widget::Widget>
}
impl<'a> Default for Node<'a> {
    fn default() -> Node<'a> {
        Node{
            appearance: RefCell::new(appearance::Appearance::default()),
            event_handlers: Vec::new(),
            geometry: RefCell::new(Geometry::default()),
            id: 0,
            layout: RefCell::new(layout::Layout::default()),
            state: RefCell::new(widget::State::default()),
            style_rules: Vec::new(),
            widget: Box::new(widget::div::Div)
        }
    }
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ id: {}, type: {:?} }}", self.id, self.widget)
    }
}
