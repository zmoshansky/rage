use layout::flow;
use renderer::geometry::Xyz;

#[derive(Clone)]
pub enum Position {
    /// This item is flowed by the parent container, or overridden here.
    Flow(flow::FlowSelfType),

    /// Absolute based on Viewport
    Absolute(Xyz),

    /// Relative to Parent
    Relative(Xyz)
    // perhaps allow some sort of anchor id in future?

    // /// Offset from Parent
    // Offset {
    //     top: Dimension,
    //     right: Dimension,
    //     left: Dimension,
    //     bottom: Dimension,
    //     // Z-TODO
    //     // front: Dimension,
    //     // back: Dimension,
    // }
}
impl Default for Position {fn default() -> Position { Position::Flow(flow::FlowSelfType::Block) }}
