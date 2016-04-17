use rose_tree::{ROOT, petgraph};

use collision;
use scene_graph::{SceneGraph, node};
use appearance::{self, background, color, font};
use layout::{self, dimension, position, flow};
use renderer::geometry;

// These are CSS-esque styles, targeted at some subset of nodes
pub struct Style {
    // pub name: Option<&str>,?
    // pub selector: &str,?
    pub rules: Vec<Rule>,
}

/// Obviously we need to group and scope rules... This isn't the wild west.
// pub struct StyleGroup {
//     pub styles: Vec<Style>,
// }

/// Some way of tracking which rules target a node...
// TODO - This needs more thought.
// Perhaps just add styles to a scope.
/// Need to be able to
// pub enum NodeRules {
//     Default(&Rule),
//     Style(Rc<Style>),
//     Direct(Rule),
// }

// Styles will use this as a basic building block, but also Nodes.
#[derive(Clone, Debug)]
pub struct Rule {
    pub condition: Option<collision::HoverState>,
    pub effect: RuleType,
}
impl Rule {
    pub fn new(condition: Option<collision::HoverState>, effect: RuleType) -> Rule {
        Rule{
            condition: condition,
            effect: effect,
        }
    }

    pub fn evaluate_condition(&self, node: &node::Node) -> bool {
        if let Some(ref condition) = self.condition {
            node.state.hover_state == *condition
        } else {true}
    }
}

// TODO - Figure out how to only override single rules... ex.) margin.left;
// Perhaps add an [auto|dont'care] option to all the underlying types...
#[derive(Clone, Debug)]
pub enum RuleType {
    // TODO - Make into a generic conditional rule ex.) :hover, :disabled, :etc.
    Appearance(AppearanceRule),
    Layout(LayoutRule),
}

#[derive(Clone, Debug)]
pub enum AppearanceRule {
    Background(background::Background),
    Border(color::Color),
    Font(font::Font),
    // Custom(.....)
}

#[derive(Clone, Debug)]
pub enum LayoutRule {
//     // Overflows(overflow::Overflows),
//     // BoxModel(box_model::BoxModel),
    // TODO - change geometry::spacing to layout::spacing
    Margin(geometry::Spacing),
    Border(geometry::Spacing),
    Padding(geometry::Spacing),
    Dimensions(dimension::Dimensions),
    Position(position::Position),
    Flow(flow::Flow),
}

// TODO - One Pass to handle changing the Node.style_rules
// pub fn style_list...

// Seperate pass to update the generated styles depending on if any conditional rules may have changed
// TODO - Optimize so only nodes with conditional styles are updated
// TODO - Optimize so only updated styles are re-computed...(in `style_list`)
pub fn style(scene_graph: &SceneGraph) {
    let mut tree = scene_graph.tree.borrow_mut();

    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node = &mut tree[node_index];

        // Temporary condition
        if !node.style_rules.is_empty() {
            // TODO, just borrow layout in same way as done with appearance.
            let mut appearance = appearance::Appearance::default();

            // TODO - Ensure a conditioned rule overrides a non-conditioned rule if the condition evaluates to true
            for rule in &node.style_rules {
                if rule.evaluate_condition(node) {
                    // println!("Evaluated Style Rules {:?}", appearance);
                    // println!("Evaluated Style Rules {:?}", layout);
                    match rule.effect {
                        RuleType::Appearance(ref appearance_rule) => {
                            match appearance_rule {
                                &AppearanceRule::Background(ref background) => {appearance.background = Some(background.clone());},
                                &AppearanceRule::Border(ref border) => {appearance.border = Some(border.clone());},
                                _ => {unimplemented!();}
                            }
                        },
                        RuleType::Layout(ref layout_rule) => {
                            match layout_rule {
                                &LayoutRule::Margin(ref margin) => {set_layout_margin(&mut node.layout, margin, scene_graph);},
                                &LayoutRule::Border(ref border) => {set_layout_border(&mut node.layout, border, scene_graph);},
                                &LayoutRule::Padding(ref padding) => {set_layout_padding(&mut node.layout, padding, scene_graph);},
                                &LayoutRule::Dimensions(ref dimensions) => {set_layout_dimensions(&mut node.layout, dimensions, scene_graph);},
                                &LayoutRule::Position(ref position) => {set_layout_position(&mut node.layout, position, scene_graph);},
                                &LayoutRule::Flow(ref flow) => {set_layout_flow(&mut node.layout, flow, scene_graph);},
                            }
                        },
                    }
                }
            }
            node.appearance = appearance;
        }
    }
}

fn set_layout_margin(layout: &mut layout::Layout, margin: &geometry::Spacing, scene_graph: &SceneGraph) {
    if layout.margin != *margin {
        layout.margin = margin.clone();
        scene_graph.needs_layout.set(true);
    }
}

fn set_layout_border(layout: &mut layout::Layout, border: &geometry::Spacing, scene_graph: &SceneGraph) {
    if layout.border != *border {
        layout.border = border.clone();
        scene_graph.needs_layout.set(true);
    }
}

fn set_layout_padding(layout: &mut layout::Layout, padding: &geometry::Spacing, scene_graph: &SceneGraph) {
    if layout.padding != *padding {
        layout.padding = padding.clone();
        scene_graph.needs_layout.set(true);
    }
}

fn set_layout_dimensions(layout: &mut layout::Layout, dimensions: &dimension::Dimensions, scene_graph: &SceneGraph) {
    if layout.dimensions != *dimensions {
        layout.dimensions = dimensions.clone();
        scene_graph.needs_layout.set(true);
    }
}

fn set_layout_position(layout: &mut layout::Layout, position: &position::Position, scene_graph: &SceneGraph) {
    if layout.position != *position {
        layout.position = position.clone();
        scene_graph.needs_layout.set(true);
    }
}

fn set_layout_flow(layout: &mut layout::Layout, flow: &flow::Flow, scene_graph: &SceneGraph) {
    if layout.flow != *flow {
        layout.flow = flow.clone();
        scene_graph.needs_layout.set(true);
    }
}