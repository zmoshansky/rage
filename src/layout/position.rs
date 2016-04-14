use layout::flow;
// TODO Convert to using an Xyz of `layout::dimension`
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
    // Offset(layout::Spacing)
}
impl Default for Position {fn default() -> Position { Position::Flow(flow::FlowSelfType::Block) }}
