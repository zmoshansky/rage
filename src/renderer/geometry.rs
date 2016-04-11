/// This is what the layout engine produces when it's finished.
#[derive(Default, Debug)]
pub struct Geometry {
    pub position: Xyz,
    // Inner content dimensions
    pub dimensions: Xyz,
    // TODO - Border
    pub margin: Spacing,
    pub padding: Spacing,
}

impl Geometry {
    pub fn contained(&self, cursor: [f64; 2]) -> bool {
        let (x, y) = (cursor[0], cursor[1]);
        self.position.x <= x && self.position.x + self.dimensions.x  >= x &&
        self.position.y <= y && self.position.y + self.dimensions.y >= y
    }
}

#[derive(Debug)]
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

#[derive(Default, Debug)]
pub struct Spacing {
    pub top: f64,
    pub bottom: f64,
    pub right: f64,
    pub left: f64,
    // Z-TODO
    // pub front: f64,
    // pub back: f64,
}

#[derive(Default)]
pub struct Xy {
    pub x: f64,
    pub y: f64,
}