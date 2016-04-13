use appearance::color::Color;

#[derive(Default, Clone)]
pub struct Font {
    pub size: f64,
    pub color: Color,
    // TODO
    // pub font_type: &Glyphs
    pub decoration: Option<Vec<FontDecoration>>
}

#[derive(Clone)]
pub enum FontDecoration {
    Bold,
    Underline,
    Italic,
    Subscript,
    Superscript,
    Strikethrough
}
