#[derive(Default)]
pub struct Dimensions {
    x: Dimension,
    y: Dimension,
    // Z-TODO
    // z: Dimension,
}

#[derive(Default)]
struct Dimension {
    qty: f64,
    unit: DimensionUnit,
}

enum DimensionUnit {
    /// px = dp * (dpi / 160)
    DisplayPixel,
    /// Percent of parent
    Percent,
    /// Percent of viewport
    Viewport,
    /// Each Grid unit is the same size
    Grid,
    /// Items are scaled according to fill ratio
    Flex,
    /// Just as big as item needs to be to contain children
    /// No need for qty
    Wrap,
}
impl Default for DimensionUnit {fn default() -> DimensionUnit { DimensionUnit::DisplayPixel }}
