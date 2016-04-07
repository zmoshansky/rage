use appearance::color::Color;

pub struct Font {
    font_size: f64,
    font_color: Color,
    // font_type: &Glyphs
    decoration: Vec<FontDecoration>
    // font_style: FontStyle
}

enum FontDecoration {
    Bold,
    Underline,
    Italic,
    Subscript,
    Superscript,
    Strikethrough
}
