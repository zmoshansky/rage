#[derive(Default, Clone)]
pub struct Layouts {
    direction: Direction,
    x: Layout,
    y: Layout,
    // Z-TODO
    // z: Layout,
}

#[derive(Clone)]
enum Layout {
    Start,
    Center,
    End,
    /// Distribute items equally between start and end.
    Justify,
    /// Distribute items equally, including around start and end.
    Space,
}
impl Default for Layout {fn default() -> Layout { Layout::Start }}

#[derive(Clone)]
enum Direction {
    Right,
    Left,
    Top,
    Bottom,
    // Z-TODO
    // Front,
    // Back
}
impl Default for Direction {fn default() -> Direction { Direction::Right }}
