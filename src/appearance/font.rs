use appearance::color::Color;

#[derive(Default, Clone, Debug)]
pub struct Font {
    pub size: f64,
    pub color: Color,
    // TODO
    // pub font_type: &Glyphs
    pub decoration: Option<Vec<FontDecoration>>
}

#[derive(Clone, Debug)]
pub enum FontDecoration {
    Bold,
    Underline,
    Italic,
    Subscript,
    Superscript,
    Strikethrough
}
