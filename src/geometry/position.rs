#[derive(Clone)]
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

#[derive(Clone)]
pub enum Alignment {
    Start,
    Center,
    End,
}
