use std::cell::Cell;
use geometry::Geometry as GeometryUncached;
use renderer::geometry::Geometry;
use appearance::Appearance;
use widget::State;

#[derive(Default)]
pub struct GraphNode<'a> {
    // TODO - Make Ref/Box to whatever type
    pub state: State<'a>,

    pub geometry_uncached: GeometryUncached,
    pub geometry: Geometry,

    // pub styles/appearance_uncached: StyleRules,
    pub appearance: Appearance,

    // Use Cell to allow mutable value...
    pub dirty: Cell<bool>,
    pub type_id: usize,
}

// impl Default for GraphNode {
//     fn default() -> GraphNode {
//         GraphNode{
//             state: State::new(),

//         }
//     }
// }
