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
    pub fn new(effect: RuleType) -> Rule {
        Rule{
            condition: None,
            effect: effect,
        }
    }

    pub fn new_with_condition(condition: collision::HoverState, effect: RuleType) -> Rule {
        Rule{
            condition: Some(condition),
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

            // We create and re-evaluate so that defaults can be observed with conditional rules.
            let mut appearance = appearance::Appearance::default();
            let mut layout = layout::Layout::default();

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
                                &AppearanceRule::Font(ref font) => {appearance.font = Some(font.clone());},
                            }
                        },
                        RuleType::Layout(ref layout_rule) => {
                            match layout_rule {
                                &LayoutRule::Margin(ref margin) => {layout.margin = margin.clone();},
                                &LayoutRule::Border(ref border) => {layout.border = border.clone();},
                                &LayoutRule::Padding(ref padding) => {layout.padding = padding.clone();},
                                &LayoutRule::Dimensions(ref dimensions) => {layout.dimensions = dimensions.clone();},
                                &LayoutRule::Position(ref position) => {layout.position = position.clone();},
                                &LayoutRule::Flow(ref flow) => {layout.flow = flow.clone();},
                            }
                        },
                    }
                }
            }
            if node.layout != layout {
                node.layout = layout;
                scene_graph.needs_layout.set(true);
            }
            if node.appearance != appearance {
                node.appearance = appearance;
                node.dirty.set(true);
            }
        }
    }
}
