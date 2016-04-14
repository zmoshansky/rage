#[allow(dead_code)]
pub mod font;
#[allow(dead_code)]
pub mod background;
#[allow(dead_code)]
pub mod color;

use appearance::font::Font;
use appearance::background::Background;

#[derive(Clone)]
pub struct Appearance {
    // pub filters: Option<Vec<Filters>>,
    // TODO - Check how piston does this
    // pub border_rounding: f64

    // Should these be Optional?
    pub background: Option<Background>,
    pub font: Option<Font>,
}

impl Default for Appearance {
    fn default() -> Appearance {
        Appearance{
            background: None,
            font: None,
        }
    }
}
