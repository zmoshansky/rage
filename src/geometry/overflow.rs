#[derive(Default)]
pub struct Overflows {
    x: Overflow,
    y: Overflow,
    // Z-TODO
    // z: Overflow,
}
enum ScrollBarType {
    /// Overlays on top of content, implement an opacity < 1 when !hover
    Overlay,
    Inline,
    Hidden,
}
enum Overflow {
    // TODO - Add step % to scroll bar?
    Scroll(ScrollBarType),
    Hidden,
}
impl Default for Overflow {fn default() -> Overflow { Overflow::Scroll(ScrollBarType::Overlay) }}
