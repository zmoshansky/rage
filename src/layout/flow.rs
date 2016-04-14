#[derive(Default, Clone)]
pub struct Flow {
    pub direction: Direction,
    pub x: FlowAxisType,
    pub y: FlowAxisType,
    // Z-TODO
    // z: FlowAxisType,
}

#[derive(Clone)]
pub enum FlowSelfType {
    /// Causes flow to break before and restart after this item
    Block,
    /// Lays out inline.
    Inline,
    /// Causes the flow to restart after this item.
    Break,
    /// Restarts the flow.
    Restart,
    /// Overrides parents flow setting for this item, doesn't affect other items.
    Align {
        x: Alignment,
        y: Alignment,
        // Z-TODO
        // z: Alignment,
    },
}
impl Default for FlowSelfType {fn default() -> FlowSelfType { FlowSelfType::Block }}

#[derive(Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Clone)]
pub enum FlowAxisType {
    Start,
    Center,
    End,
    /// Distribute items equally between start and end.
    Justify,
    /// Distribute items equally, including around start and end.
    Space,
}
impl Default for FlowAxisType {fn default() -> FlowAxisType { FlowAxisType::Start }}

#[derive(Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    // Z-TODO
    // Forward,
    // Backward
}
impl Default for Direction {fn default() -> Direction { Direction::Right }}

// pub fn flow_right(parent: &Node, node: &Node, position: &mut Xy) {
//     set_position_x(node, position.x);
//     set_position_y(node, position.y);
//     let bounds = bounding_box(node);
//     position.x += bounds.x;
// }