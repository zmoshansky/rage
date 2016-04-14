#[derive(Default, Clone)]
pub struct Dimensions {
    pub x: Dimension,
    pub y: Dimension,
    // Z-TODO
    // pub z: Dimension,
}

#[derive(Clone)]
pub enum Dimension {
    /// px = dp * (dpi / 160)
    DisplayPixel(f64),

    /// Percent of viewport (Easy to resolve into a width/height/)
    Viewport(f64),

    /// Percent of parent's content-box
    Percent(f64),

    /// Each Grid unit is the same size
    /// An automatic way to do percentages (Where 1 grid = parent_x / sum(grid))
    Grid(f64),

    // Chilren take their "minimum space"; Then the remaining space is treated like a grid.
    // Flex(f64),

    // Just as big as item needs to be to contain children;
    // If the widget is primitive, it runs it's layout method.
    // Else, Returns 0 if it isn't a bounded quantity (ex. all children are percentage based.)
    // Wrap,
}
impl Default for Dimension {fn default() -> Dimension { Dimension::Viewport(1.0) }}
// impl Default for Dimension {fn default() -> Dimension { Dimension::Flex(1.0) }}
