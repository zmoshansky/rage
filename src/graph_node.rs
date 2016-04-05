use std::cell::Cell;

#[derive(Default)]
pub struct GraphNode {
    pub x: f64,
    pub y: f64,
    pub w: u32,
    pub h: u32,
    pub dirty: Cell<bool>,
    pub type_id: usize,
}
