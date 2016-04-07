/// This is what the layout engine produces when it's finished.
#[derive(Default)]
pub struct Geometry {
    pub position: Xyz,
    pub dimensions: Xyz,
    // TODO - Unsure if this is really necessary or if layout should have dealt with any needs for this...
    // margins: Spacing,
    // padding: Spacing,
}

// #[derive(Default)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    // Z-TODO
    // pub z: f64,
}
// Just for testing
impl Default for Xyz {
    fn default() -> Xyz {
        Xyz{
            x: 10.0,
            y: 10.0,
        }
    }
}

#[derive(Default)]
struct Spacing {
    pub top: f64,
    pub bottom: f64,
    pub right: f64,
    pub left: f64,
    // Z-TODO
    // pub front: f64,
    // pub back: f64,
}