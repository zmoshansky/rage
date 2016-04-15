use std::fmt;

/// This is what the layout engine produces when it's finished.
#[derive(Default, Clone)]
pub struct Geometry {
    // Content Box
    pub position: Xyz,
    pub dimensions: Xyz,

    pub border: Spacing,
    pub margin: Spacing,
    pub padding: Spacing,
}

impl Geometry {
    pub fn within_border_box(&self, point: &Xy) -> bool {
        self.position.x - self.border.left - self.padding.left < point.x &&
        self.position.x + self.border.right + self.padding.right + self.dimensions.x > point.x &&
        self.position.y - self.border.top - self.padding.top < point.y &&
        self.position.y + self.border.bottom + self.padding.bottom + self.dimensions.y > point.y
    }

    /// [x, y, w, h]
    pub fn border_box(&self) -> [f64;4] {
        let padding_box = self.padding_box();
        [
            padding_box[0] - self.border.left,
            padding_box[1] - self.border.top,
            padding_box[2] + self.border.left + self.border.right,
            padding_box[3] + self.border.top + self.border.bottom,
        ]
    }

    /// [x, y, w, h]
    pub fn padding_box(&self) -> [f64;4] {
        [
            self.position.x - self.padding.left,
            self.position.y - self.padding.top,
            self.padding.left + self.dimensions.x + self.padding.right,
            self.padding.top + self.dimensions.y + self.padding.bottom,
        ]
    }

    pub fn bounding_dimensions(&self) -> Xy {
        Xy{
            x: self.margin.left + self.margin.right + self.padding.left + self.padding.right + self.border.left + self.border.right + self.dimensions.x,
            y: self.margin.top + self.margin.bottom + self.padding.top + self.padding.bottom + self.border.top + self.border.bottom + self.dimensions.y,
        }
    }

    pub fn set_bounding_position_x(&mut self, bounding_pos_x: f64) -> bool {
        let content_position_x = bounding_pos_x + self.margin.left + self.border.left + self.padding.left;
        let changed = self.position.x != content_position_x;
        self.position.x = content_position_x;
        changed
    }
    pub fn set_bounding_position_y(&mut self, bounding_pos_y: f64) -> bool {
        let content_position_y = bounding_pos_y + self.margin.top + self.border.top + self.padding.top;
        let changed = self.position.y != content_position_y;
        self.position.y = content_position_y;
        changed
    }
}

impl fmt::Debug for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Geometry {{ position: {:?}, dimensions: {:?} }}", self.position, self.dimensions)
    }
}


#[derive(Default, Clone, Debug)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    // Z-TODO
    // pub z: f64,
}

#[derive(Default, Clone, Debug)]
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
