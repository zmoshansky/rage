use std::cell::RefCell;
use std::fmt;

use appearance;
use layout;
use renderer::geometry::Geometry;
use style;
use widget;

#[derive(Clone)]
pub struct Node<'a> {
    pub id: u32,
    pub state: RefCell<widget::State<'a>>,

    // Styles are turned into layout and appearance in style pass
    pub style_rules: Vec<style::Rule>,

    /// Layout is turned into geometry by Layout pass
    pub layout:  RefCell<layout::Layout>,
    pub geometry: RefCell<Geometry>,

    // pub styles/appearance_uncached: StyleRules,
    pub appearance:  RefCell<appearance::Appearance>,

    // Use Cell to allow mutable value...
    // TODO - Change this to reflect what's needed, layout pass, render pass, etc.
    pub widget: Box<widget::Widget>
}
impl<'a> Default for Node<'a> {
    fn default() -> Node<'a> {
        Node{
            id: 0,
            state: RefCell::new(widget::State::default()),
            style_rules: Vec::new(),
            layout: RefCell::new(layout::Layout::default()),
            geometry: RefCell::new(Geometry::default()),
            appearance: RefCell::new(appearance::Appearance::default()),
            widget: Box::new(widget::div::Div)
        }
    }
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ id: {}, type: {:?} }}", self.id, self.widget)
    }
}
