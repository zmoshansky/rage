use rose_tree;

// Individually import items to determine what's needed. Later group so glob imports can be used.
use layout::{flow, position};
use layout::dimension::{self, Dimension};
use renderer::geometry;
use scene_graph::node::Node;
use scene_graph::SceneGraph;
use widget::{self, text, image};
use appearance::background;
use appearance::background::Background;
use appearance::color;
use appearance::font::Font;
use style::{Rule, AppearanceRule, LayoutRule};
use style::RuleType::{Appearance, Layout};
use collision;
use event;

// https://www.google.com/design/spec/style/color.html#color-color-palette
pub fn web_browser(scene_graph: &mut SceneGraph) {

    let container = scene_graph.add_child_root(Box::new(Node{
        style_rules: vec![
            Rule::new(Appearance(AppearanceRule::Background(background::Background::Color(color::hex("FAFAFA"))))),
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Viewport(1.0), y: Dimension::Viewport(1.0)}))),
            Rule::new(Layout(LayoutRule::Flow(flow::Flow{
                direction: flow::Direction::Down,
                ..Default::default()
            }))),
        ],
        ..Default::default()
    }));

    tabs(scene_graph, container);
    address_bar(scene_graph, container);

    scene_graph.add_child(container, Box::new(Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Percent(1.0), y: Dimension::Percent(1.0)}))),
        ],
        widget: Box::new(image::Image{path: "assets/images/page.png"}),
        ..Default::default()
    }));
}

fn tabs<'a>(scene_graph: &mut SceneGraph<'a>, container: rose_tree::NodeIndex) {
    // Tab Bar
    let tab_bar = scene_graph.add_child(container, Box::new(Node{
        style_rules: vec![
            Rule::new(Appearance(AppearanceRule::Background(background::Background::Color(color::hex("393f3f"))))),
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(40.0)}))),
        ],
        ..Default::default()
    }));

    // Tab Template
    let tab = Node{
        style_rules: vec![
            Rule::new(Appearance(AppearanceRule::Background(background::Background::Color(color::hex("424242"))))),
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Grid(1.0), y: Dimension::Percent(1.0)}))),
            Rule::new(Layout(LayoutRule::Padding(geometry::Spacing{left: 8.0, ..Default::default()}))),
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{top: 4.0, left: 4.0, right: 4.0, bottom: 0.0}))),
        ],
        ..Default::default()
    };

    // Tabs
    let tab_0 = tab.clone();
    tab_0.appearance.borrow_mut().background = Some(Background::Color(color::hex("949898")));
    tab_0.layout.borrow_mut().margin = geometry::Spacing{top: 4.0, left: 20.0, right: 4.0, bottom: 0.0};
    let tab_0 = scene_graph.add_child(tab_bar, Box::new(tab_0));

    let tab_1 = scene_graph.add_child(tab_bar, Box::new(tab.clone()));

    // New Tab Icon
    scene_graph.add_child(tab_bar, Box::new(Node{
        style_rules: vec![
            Rule::new(Appearance(AppearanceRule::Border(color::hex("d0d1cf")))),
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{top: 4.0, bottom: 4.0, ..Default::default()}))),
            Rule::new(Layout(LayoutRule::Border(geometry::Spacing{left: 2.0, ..Default::default()}))),
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::DisplayPixel(0.0), y: Dimension::Percent(0.80)}))),
        ],
        ..Default::default()
    }));


    scene_graph.add_child(tab_bar, Box::new(Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{top: 8.0, bottom: 8.0, left: 8.0, ..Default::default()}))),
            Rule::new(Layout(LayoutRule::Padding(geometry::Spacing{top: 2.0, left: 2.0, bottom: 2.0, right: 2.0}))),
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Wrap, y: Dimension::Wrap}))),

            // For now, conditional rules must come after others
            Rule::new_with_condition(collision::CollisionState::Hover, Appearance(AppearanceRule::Background(background::Background::Color(color::hex("949898"))))),
        ],
        widget: Box::new(image::Image{path: "assets/icons/plus.png"}),
        ..Default::default()
    }));

    // Spacer for the end since we don't yet have min/max-width
    scene_graph.add_child(tab_bar, Box::new(Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Grid(3.0), y: Dimension::Percent(1.0)}))),
        ],
        ..Default::default()
    }));

    // Tab Text Template
    let tab_text = Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Flex(1.0), y: Dimension::Wrap}))),
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{top: 8.0, left: 8.0, ..Default::default()}))),
            Rule::new(Appearance(AppearanceRule::Font(Font{
                    size: 12.0,
                    color: color::WHITE,
                    ..Default::default()
                }))
            ),
        ],
        ..Default::default()
    };

    let func = tapped;
    let tab_close = Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Wrap, y: Dimension::Wrap}))),
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{top: 6.0, right: 8.0, ..Default::default()}))),
            Rule::new(Layout(LayoutRule::Padding(geometry::Spacing{top: 4.0, left: 4.0, bottom: 4.0, right: 4.0}))),

            // For now, conditional rules must come after others
            Rule::new_with_condition(collision::CollisionState::Hover, Appearance(AppearanceRule::Background(background::Background::Color([1.0, 0.0, 0.0, 1.0])))),
        ],
        widget: Box::new(image::Image{path: "assets/icons/close.png"}),
        event_handlers: vec![
            event::EventHandler{
                event: event::EventType::Pressed,
                callback: func,
            }
        ],
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
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::DisplayPixel(32.0), y: Dimension::DisplayPixel(32.0)}))),
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
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(70.0)}))),
            Rule::new(Appearance(AppearanceRule::Background(background::Background::Color(color::hex("949898"))))),
        ],
        ..Default::default()
    }));

    // Url Box
    scene_graph.add_child(address_bar, Box::new(Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Flex(1.0), y: Dimension::Wrap}))),
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{top: 14.0, bottom: 14.0, left: 50.0, ..Default::default()}))),
            Rule::new(Layout(LayoutRule::Padding(geometry::Spacing{top: 6.0, left: 24.0, bottom: 6.0, right: 6.0}))),
            Rule::new(Appearance(AppearanceRule::Background(background::Background::Color(color::hex("292929"))))),
            Rule::new(Appearance(AppearanceRule::Font(Font{
                    size: 14.0,
                    color: color::WHITE,
                    ..Default::default()
                }))
            ),

            Rule::new_with_condition(collision::CollisionState::Hover, Layout(LayoutRule::Border(geometry::Spacing{left: 2.0, top: 2.0, right: 2.0, bottom: 2.0, ..Default::default()}))),
            Rule::new_with_condition(collision::CollisionState::Hover, Appearance(AppearanceRule::Border(color::hex("2196F3")))),
        ],
        widget: Box::new(text::Text{text: "https://start.fedoraproject.org"}),
        ..Default::default()
    }));

    // Search Box
    scene_graph.add_child(address_bar, Box::new(Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Percent(0.18), y: Dimension::Wrap}))),
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{top: 14.0, bottom: 14.0, left: 16.0, ..Default::default()}))),
            Rule::new(Layout(LayoutRule::Padding(geometry::Spacing{top: 6.0, left: 6.0, bottom: 6.0, right: 6.0}))),
            Rule::new(Appearance(AppearanceRule::Background(background::Background::Color(color::hex("292929"))))),
            Rule::new(Appearance(AppearanceRule::Font(Font{
                    size: 14.0,
                    color: color::hex("999999"),
                    ..Default::default()
                }))
            ),

            Rule::new_with_condition(collision::CollisionState::Hover, Layout(LayoutRule::Border(geometry::Spacing{left: 2.0, top: 2.0, right: 2.0, bottom: 2.0, ..Default::default()}))),
            Rule::new_with_condition(collision::CollisionState::Hover, Appearance(AppearanceRule::Border(color::hex("2196F3")))),
        ],
        widget: Box::new(text::Text{text: "Search"}),
        ..Default::default()
    }));

    // Right Icons
    scene_graph.add_child(address_bar, Box::new(Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Wrap, y: Dimension::Wrap}))),
            Rule::new(Layout(LayoutRule::Margin(geometry::Spacing{left: 16.0, right: 16.0, ..Default::default()}))),
        ],
        widget: Box::new(image::Image{path: "assets/icons/icons_right.png"}),
        ..Default::default()
    }));

    // Back Button
    scene_graph.add_child(address_bar, Box::new(Node{
        style_rules: vec![
            Rule::new(Layout(LayoutRule::Dimensions(dimension::Dimensions{x: Dimension::Wrap, y: Dimension::Wrap}))),
            Rule::new(Layout(LayoutRule::Padding(geometry::Spacing{top: 8.0, left: 8.0, bottom: 8.0, right: 8.0}))),
            Rule::new(Layout(LayoutRule::Position(position::Position::Relative(geometry::Xyz{x: 0.0, y: 0.0})))),
        ],
        widget: Box::new(image::Image{path: "assets/icons/left_arrow.png"}),
        ..Default::default()
    }));
}

fn tapped(state: &widget::State) -> () {
    println!("Tapped {:?}", state);
}