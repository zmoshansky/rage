// Individually import items to determine what's needed. Later group so glob imports can be used.
use layout::Layout;
use layout::{flow};
use layout::dimension::{Dimensions, Dimension};
use renderer::geometry;
use scene_graph::node::Node;
use scene_graph::SceneGraph;
use widget::{text, image};
use appearance::Appearance;
use appearance::background::Background;
use appearance::color;
use appearance::font::Font;


// https://www.google.com/design/spec/style/color.html#color-color-palette
pub fn web_browser(scene_graph: &mut SceneGraph) {
    let container = scene_graph.add_child_root(&mut Node{
        appearance: Appearance{
            background: Some(Background::Color(color::hex("FAFAFA"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Viewport(1.0), y: Dimension::Viewport(1.0)},
            flow: flow::Flow{
                direction: flow::Direction::Down,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });


    // Tab Bar
    let tab_bar = scene_graph.add_child(container, &mut Node{
        appearance: Appearance{
            background: Some(Background::Color(color::hex("212121"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(40.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    let tab = Node{
        appearance: Appearance{
            background: Some(Background::Color(color::hex("424242"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::Percent(1.0)},
            padding: geometry::Spacing{top: 8.0, left: 8.0, ..Default::default()},
            margin: geometry::Spacing{top: 4.0, left: 4.0, right: 4.0, bottom: 0.0},
            ..Default::default()
        },
        ..Default::default()
    };

    let tab_0 = scene_graph.add_child(tab_bar, &mut tab.clone());
    let mut tab_1 = tab.clone();
    tab_1.appearance.background = Some(Background::Color(color::hex("626262")));
    let tab_1 = scene_graph.add_child(tab_bar, &mut tab_1);
    let tab_2 = scene_graph.add_child(tab_bar, &mut tab.clone());

    // Spacer for the end since we don't yet have min/max-width
    scene_graph.add_child(tab_bar, &mut Node{
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Grid(3.0), y: Dimension::Percent(1.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    // Tab Text
    let tab_text = Node{
        appearance: Appearance{
            font: Some(Font{
                size: 12.0,
                color: color::WHITE,
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tab_0_text = tab_text.clone();
    tab_0_text.widget = Box::new(text::Text{text: "Getting Started"});
    scene_graph.add_child(tab_0, &mut tab_0_text);

    let mut tab_1_text = tab_text.clone();
    tab_1_text.widget = Box::new(text::Text{text: "YouTube"});
    scene_graph.add_child(tab_1, &mut tab_1_text);

    let mut tab_2_text = tab_text.clone();
    tab_2_text.widget = Box::new(text::Text{text: "StackOverflow"});
    scene_graph.add_child(tab_2, &mut tab_2_text);


    // Address Bar
    let address_bar = scene_graph.add_child(container, &mut Node{
        appearance: Appearance{
            background: Some(Background::Color(color::hex("212121"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(50.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    scene_graph.add_child(address_bar, &mut Node{
        appearance: Appearance{
            font: Some(Font{
                size: 14.0,
                color: color::BLACK,
                ..Default::default()
            }),
            background: Some(Background::Color(color::WHITE)),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Percent(0.30), y: Dimension::DisplayPixel(26.0)},
            margin: geometry::Spacing{top: 6.0, left: 6.0, bottom: 6.0, right: 6.0, ..Default::default()},
            padding: geometry::Spacing{top: 6.0, left: 6.0, bottom: 6.0, right: 6.0, ..Default::default()},
            ..Default::default()
        },
        widget: Box::new(text::Text{text: "https://www.youtube.com"}),
        ..Default::default()
    });

    scene_graph.add_child(container, &mut Node{
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(100.0)},
            ..Default::default()
        },
        widget: Box::new(image::Image{path: "assets/images/rust.png"}),
        ..Default::default()
    });
}
