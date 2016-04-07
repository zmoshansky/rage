#[allow(dead_code)]
pub mod dimension;
#[allow(dead_code)]
pub mod position;
#[allow(dead_code)]
pub mod overflow;
#[allow(dead_code)]
pub mod layout;
#[allow(dead_code)]
pub mod spacing;


use geometry::dimension::Dimensions;
use geometry::position::Position;
use geometry::overflow::Overflows;
use geometry::layout::Layouts;
use geometry::spacing::Spacing;

#[derive(Default)]
pub struct Geometry {
    pub overflows: Overflows,
    pub dimensions: Dimensions,
    pub position: Position,
    pub margins: Spacing,
    pub padding: Spacing,

    /// Containers
    pub layout: Layouts,
}
