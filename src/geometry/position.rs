// #[derive(Default)]
// pub struct RelativePosition {
//     /// Uses DisplayPixel, should it just be a `Dimension`?
//     qty: f64,
//     unit: PositionUnit,
// }

// enum PositionUnit {
//     /// Relative to parent
//     Parent,
//     /// Relative to viewport
//     Viewport,
// }
// impl Default for PositionUnit {fn default() -> PositionUnit { PositionUnit::Parent }}


pub enum Position {
    Block,
    Inline,
    Align {
        x: Alignment,
        y: Alignment,
        // Z-TODO
        // z: Alignment,
    },
    Absolute {
        x: f64,
        y: f64,
        // z: f64,
    },
    Relative {
        x: f64,
        y: f64,
        // z: f64,
    }
}
impl Default for Position {fn default() -> Position { Position::Block }}

pub enum Alignment {
    Start,
    Center,
    End,
}