// use geometry::{Dimensions, Positioning, Overflows, Layouts};
// use appearance::{Background, Font};

// /// Styles - Template for what [map|struct of options] of styles would look like
// // ---------------------------------------------------------------

// /// Map gives flexibility for any style, struct of options would be [higher performance?] but more rigid...
// /// May allow maps to be used in `.ui` file that get merged together to form the struct...

// #[derive(Default)]
// pub struct Style {
//     pub dimensions: Dimensions,
//     pub positioning: Positioning,
//     pub opacity: f64,
//     pub overflows: Overflows,

//     /// Containers
//     pub layout: Layouts,

//     // Should these be Optional?
//     pub background: Option<Background>,
//     pub font: Option<Font>,
// }
// // impl Default for Style {fn default() -> Style { Style{opacity: 0.0, ..Default::default()} }}
