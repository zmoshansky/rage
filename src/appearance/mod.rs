#[allow(dead_code)]
pub mod font;
#[allow(dead_code)]
pub mod background;
#[allow(dead_code)]
pub mod color;

use appearance::font::Font;
use appearance::background::Background;

#[derive(Debug, Clone)]
pub struct Appearance {
    // pub filters: Option<Vec<Filters>>,

    pub background: Option<Background>,

    // pub border_rounding: f64
    pub border: Option<color::Color>,

    // TODO - Handle widget specific things, like font
    pub font: Option<Font>,
}

impl Default for Appearance {
    fn default() -> Appearance {
        Appearance{
            background: None,
            border: None,
            font: None,
        }
    }
}
