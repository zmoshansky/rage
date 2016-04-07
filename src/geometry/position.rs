#[derive(Default)]
pub struct RelativePosition {
    /// Uses DisplayPixel, should it just be a `Dimension`?
    qty: f64,
    unit: PositionUnit,
}

enum PositionUnit {
    /// Relative to parent
    Parent,
    /// Relative to viewport
    Viewport,
}
impl Default for PositionUnit {fn default() -> PositionUnit { PositionUnit::Parent }}


pub enum Position {
    Block,
    Inline,
    Align {
        x: Alignment,
        y: Alignment,
        // Z-TODO
        // z: Alignment,
    },
    Relative {
        x: RelativePosition,
        y: RelativePosition,
        z: RelativePosition,
    }
}
impl Default for Position {fn default() -> Position { Position::Block }}

pub enum Alignment {
    Start,
    Center,
    End,
}