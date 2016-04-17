use appearance::color::Color;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Font {
    pub size: f64,
    pub color: Color,
    // TODO
    // pub font_type: &Glyphs
    pub decoration: Option<Vec<FontDecoration>>
}

#[derive(Clone, PartialEq, Debug)]
pub enum FontDecoration {
    Bold,
    Underline,
    Italic,
    Subscript,
    Superscript,
    Strikethrough
}
