use rose_tree;

// Individually import items to determine what's needed. Later group so glob imports can be used.
use layout::Layout;
use layout::{flow, position};
use layout::dimension::{Dimensions, Dimension};
use renderer::geometry;
use scene_graph::node::Node;
use scene_graph::SceneGraph;
use widget::{text, image};
use appearance::{Appearance, background};
use appearance::background::Background;
use appearance::color;
use appearance::font::Font;
use style;
use collision;
// ::{Appearance, AppearanceRule, RuleType, Rule};


// https://www.google.com/design/spec/style/color.html#color-color-palette
pub fn web_browser(scene_graph: &mut SceneGraph) {

    let container = scene_graph.add_child_root(Box::new(Node{
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
    }));

    tabs(scene_graph, container);
    address_bar(scene_graph, container);

    scene_graph.add_child(container, Box::new(Node{
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Percent(1.0), y: Dimension::Percent(1.0)},
            ..Default::default()
        },
        widget: Box::new(image::Image{path: "assets/images/page.png"}),
        ..Default::default()
    }));
}

fn tabs(scene_graph: &mut SceneGraph, container: rose_tree::NodeIndex) {
    // Tab Bar
    let tab_bar = scene_graph.add_child(container, Box::new(Node{
        appearance: Appearance{
            background: Some(Background::Color(color::hex("393f3f"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(40.0)},
            ..Default::default()
        },
        ..Default::default()
    }));

    // Tab Template
    let tab = Node{
        appearance: Appearance{
            background: Some(Background::Color(color::hex("424242"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::Percent(1.0)},
            padding: geometry::Spacing{left: 8.0, ..Default::default()},
            margin: geometry::Spacing{top: 4.0, left: 4.0, right: 4.0, bottom: 0.0},
            ..Default::default()
        },
        ..Default::default()
    };

    // Tabs
    let mut tab_0 = tab.clone();
    tab_0.appearance.background = Some(Background::Color(color::hex("949898")));
    tab_0.layout.margin = geometry::Spacing{top: 4.0, left: 20.0, right: 4.0, bottom: 0.0};
    let tab_0 = scene_graph.add_child(tab_bar, Box::new(tab_0));

    let tab_1 = scene_graph.add_child(tab_bar, Box::new(tab.clone()));

    // New Tab Icon
    scene_graph.add_child(tab_bar, Box::new(Node{
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Wrap, y: Dimension::Wrap},
            margin: geometry::Spacing{top: 4.0, bottom: 4.0, left: 8.0, ..Default::default()},
            border: geometry::Spacing{left: 2.0, ..Default::default()},
            padding: geometry::Spacing{top: 4.0, left: 8.0, bottom: 4.0, ..Default::default()},
            ..Default::default()
        },
        style_rules: vec![
            // Workaround for partial border render implementation
            style::Rule::new(None, style::RuleType::Appearance(style::AppearanceRule::Background(background::Background::Color(color::hex("393f3f"))))),
            style::Rule::new(None, style::RuleType::Appearance(style::AppearanceRule::Border(color::hex("d0d1cf")))),
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Border(geometry::Spacing{left: 2.0, ..Default::default()}))),
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Margin(geometry::Spacing{top: 4.0, bottom: 4.0, left: 8.0, ..Default::default()}))),
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Padding(geometry::Spacing{top: 4.0, left: 8.0, bottom: 4.0, ..Default::default()}))),
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Dimensions(Dimensions{x: Dimension::Wrap, y: Dimension::Wrap}))),

            // For now, conditional rules must come after others
            style::Rule::new(Some(collision::HoverState::Hover), style::RuleType::Appearance(style::AppearanceRule::Background(background::Background::Color(color::hex("949898"))))),
        ],
        widget: Box::new(image::Image{path: "assets/icons/plus.png"}),
        ..Default::default()
    }));

    // Spacer for the end since we don't yet have min/max-width
    scene_graph.add_child(tab_bar, Box::new(Node{
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Grid(3.0), y: Dimension::Percent(1.0)},
            ..Default::default()
        },
        ..Default::default()
    }));

    // Tab Text Template
    let tab_text = Node{
        appearance: Appearance{
            font: Some(Font{
                size: 12.0,
                color: color::WHITE,
                ..Default::default()
            }),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Flex(1.0), y: Dimension::Wrap},
            margin: geometry::Spacing{top: 8.0, left: 8.0, ..Default::default()},
            ..Default::default()
        },
        ..Default::default()
    };

    let tab_close = Node{
        style_rules: vec![
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Dimensions(Dimensions{x: Dimension::Wrap, y: Dimension::Wrap}))),
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Margin(geometry::Spacing{top: 6.0, right: 8.0, ..Default::default()}))),
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Padding(geometry::Spacing{top: 4.0, left: 4.0, bottom: 4.0, right: 4.0}))),

            // For now, conditional rules must come after others
            style::Rule::new(Some(collision::HoverState::Hover), style::RuleType::Appearance(style::AppearanceRule::Background(background::Background::Color([1.0, 0.0, 0.0, 1.0])))),
        ],
        widget: Box::new(image::Image{path: "assets/icons/close.png"}),
        ..Default::default()
    };

    // Tab 0
    let mut tab_0_text = tab_text.clone();
    tab_0_text.widget = Box::new(text::Text{text: "Fedora Project - Start Page"});
    scene_graph.add_child(tab_0, Box::new(tab_0_text));
    scene_graph.add_child(tab_0, Box::new(tab_close.clone()));

    // Tab 1
    scene_graph.add_child(tab_1, Box::new(Node{
        style_rules: vec![
            style::Rule::new(None, style::RuleType::Layout(style::LayoutRule::Dimensions(Dimensions{x: Dimension::DisplayPixel(32.0), y: Dimension::DisplayPixel(32.0)}))),
        ],
        widget: Box::new(image::Image{path: "assets/images/yt_favicon.png"}),
        ..Default::default()
    }));

    let mut tab_1_text = tab_text.clone();
    tab_1_text.widget = Box::new(text::Text{text: "YouTube"});
    scene_graph.add_child(tab_1, Box::new(tab_1_text));
    scene_graph.add_child(tab_1, Box::new(tab_close.clone()));

}

fn address_bar(scene_graph: &mut SceneGraph, container: rose_tree::NodeIndex) {
    let address_bar = scene_graph.add_child(container, Box::new(Node{
        appearance: Appearance{
            background: Some(Background::Color(color::hex("949898"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(70.0)},
            ..Default::default()
        },
        ..Default::default()
    }));

    // Url Box
    scene_graph.add_child(address_bar, Box::new(Node{
        appearance: Appearance{
            font: Some(Font{
                size: 14.0,
                color: color::WHITE,
                ..Default::default()
            }),
            background: Some(Background::Color(color::hex("292929"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Flex(1.0), y: Dimension::Wrap},
            margin: geometry::Spacing{top: 14.0, bottom: 14.0, left: 50.0, ..Default::default()},
            padding: geometry::Spacing{top: 6.0, left: 24.0, bottom: 6.0, right: 6.0},
            ..Default::default()
        },
        widget: Box::new(text::Text{text: "https://start.fedoraproject.org"}),
        ..Default::default()
    }));

    // Search Box
    scene_graph.add_child(address_bar, Box::new(Node{
        appearance: Appearance{
            font: Some(Font{
                size: 14.0,
                color: color::hex("999999"),
                ..Default::default()
            }),
            background: Some(Background::Color(color::hex("292929"))),
            ..Default::default()
        },
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Percent(0.18), y: Dimension::Wrap},
            margin: geometry::Spacing{top: 14.0, bottom: 14.0, left: 16.0, ..Default::default()},
            padding: geometry::Spacing{top: 6.0, left: 6.0, bottom: 6.0, right: 6.0},
            ..Default::default()
        },
        widget: Box::new(text::Text{text: "Search"}),
        ..Default::default()
    }));

    // Right Icons
    scene_graph.add_child(address_bar, Box::new(Node{
        layout: Layout{
            dimensions: Dimensions{x: Dimension::Wrap, y: Dimension::Wrap},
            margin: geometry::Spacing{left: 16.0, right: 16.0, ..Default::default()},
            ..Default::default()
        },
        widget: Box::new(image::Image{path: "assets/icons/icons_right.png"}),
        ..Default::default()
    }));

    // Back Button
    scene_graph.add_child(address_bar, Box::new(Node{
        layout: Layout{
            position: position::Position::Relative(geometry::Xyz{x: 0.0, y: 0.0}),
            dimensions: Dimensions{x: Dimension::Wrap, y: Dimension::Wrap},
            padding: geometry::Spacing{top: 8.0, left: 8.0, bottom: 8.0, right: 8.0},
            ..Default::default()
        },
        widget: Box::new(image::Image{path: "assets/icons/left_arrow.png"}),
        ..Default::default()
    }));
}
