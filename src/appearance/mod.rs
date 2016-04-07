#[allow(dead_code)]
pub mod font;
#[allow(dead_code)]
pub mod background;
#[allow(dead_code)]
pub mod color;

use appearance::font::Font;
use appearance::background::Background;

// #[derive(Default)]
pub struct Appearance {
    pub opacity: f64,
    // pub border_rounding: f64
    // TODO - Check how piston does this

    // Should these be Optional?
    pub background: Option<Background>,
    pub font: Option<Font>,
}

impl Default for Appearance {
    fn default() -> Appearance {
        Appearance{
            opacity: 1.0,
            background: None,
            font: None,
        }
    }
}
