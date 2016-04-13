// Individually import items to determine what's needed. Later group so glob imports can be used.
use graph_node::GraphNode;
use geometry::{Geometry as GeometryUncached};
use geometry::dimension::{Dimensions, Dimension};
use widget::State;
use scene_graph::SceneGraph;
use widget::div::Div;
use widget::button::Button;
use appearance::Appearance;
use appearance::background::Background;
use appearance::color;
use appearance::font::Font;

pub fn web_browser(scene_graph: &mut SceneGraph) {
    scene_graph.types.push(Box::new(Div));
    scene_graph.types.push(Box::new(Button));

    // Tabbar Container
    let tab = GraphNode{
        type_id: 0,
        appearance: Appearance{
            background: Some(Background::Color(color::hex("424242"))),
            ..Default::default()
        },
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::Percent(1.0)},
            ..Default::default()
        },
        ..Default::default()
    };

    // Tabs
    let tabs = scene_graph.add_child_root(&mut GraphNode{
        type_id: 0,
        appearance: Appearance{
            background: Some(Background::Color(color::hex("212121"))),
            ..Default::default()
        },
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Viewport(1.0), y: Dimension::DisplayPixel(40.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    let tab_0 = scene_graph.add_child(tabs, &mut tab.clone());
    let tab_1 = scene_graph.add_child(tabs, &mut tab.clone());
    let tab_2 = scene_graph.add_child(tabs, &mut tab.clone());

    // Spacer for the end since we don't yet have min/max-width
    scene_graph.add_child(tabs, &mut GraphNode{
        type_id: 0,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(3.0), y: Dimension::Percent(1.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    // Tab Text
    let tab_text = GraphNode{
        type_id: 1,
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
    tab_0_text.state = State{text: "Getting Started", ..Default::default()};
    scene_graph.add_child(tab_0, &mut tab_0_text);

    let mut tab_1_text = tab_text.clone();
    tab_1_text.state = State{text: "YouTube", ..Default::default()};
    scene_graph.add_child(tab_1, &mut tab_1_text);

    let mut tab_2_text = tab_text.clone();
    tab_2_text.state = State{text: "StackOverflow", ..Default::default()};
    scene_graph.add_child(tab_2, &mut tab_2_text);
}
