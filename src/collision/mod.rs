use std::rc::Rc;

use style;
use scene_graph::{SceneGraph, node};
use rose_tree::{ROOT, petgraph};
use renderer::geometry::{Xy, Geometry};
use event;

pub struct CollisionArgs<'a> {
    pub cursor: &'a Xy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HoverState {
    Up,
    Hover,
    Down,
    Drag
}

impl Default for HoverState {
    fn default() -> HoverState {HoverState::Up}
}

// Keep a quick list of absolutely positioned nodes. Then DFS/BFS search can be used on the `UiTree` and exited early as a bounding box heirarchy.
// On mouse button down - All hover nodes set to down.
// On mouse release - All [down|drag] nodes emit event.
// Assumes mouse move always happens before press/release.
pub fn collision(args: &CollisionArgs, scene_graph: &SceneGraph) {
    let tree = scene_graph.tree.borrow();

    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node: &Rc<node::Node> = &tree[node_index];

        // OPTIMIZATION - Store a List of absolutely positioned nodes, then collision only need check those and do a DFS, skipping branches as soon as a `node.is_hover() == false`
        // OPTIMIZATION - `scene_graph.cursor_over: WeakRef<Nodes>` of `hover_state != HoverState::Up` for press and drag events.

        // TODO - Is there a better fix?
        // https://github.com/rust-lang/rfcs/issues/811
        let hover_state = node.state.borrow().hover_state.clone();
        let over = cursor_over(&node.geometry.borrow(), args.cursor);
        if over && node.id != 0 {
            if let HoverState::Up = hover_state {
                set_state(node, HoverState::Hover, scene_graph);
                event::emit_events(node, event::EventType::Hovering);
            }
        } else {
            match hover_state {
                HoverState::Hover => {
                    set_state(node, HoverState::Up, scene_graph);
                    event::emit_events(node, event::EventType::Hovered);
                }
                HoverState::Down => {
                    set_state(node, HoverState::Drag, scene_graph);
                    event::emit_events(node, event::EventType::Dragging);

                }
                _ => {}
            }
        }
    }
}

/// On press - All `HoverState::Hover` nodes set to down.
pub fn press(scene_graph: &SceneGraph) {
    let tree = scene_graph.tree.borrow();

    // OPTIMIZATION - `scene_graph.cursor_over: WeakRef<Nodes>` of `hover_state != HoverState::Up` for press and drag events.
    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node = &tree[node_index];

        // TODO - Is there a better fix?
        // https://github.com/rust-lang/rfcs/issues/811
        let hover_state = node.state.borrow().hover_state.clone();
        if let HoverState::Hover = hover_state {
            set_state(node, HoverState::Down, scene_graph);
            event::emit_events(node, event::EventType::Pressing);

        }
    }
}

/// On press - All `HoverState::Hover` nodes set to down.
pub fn release(args: &CollisionArgs, scene_graph: &SceneGraph) {
    let tree = scene_graph.tree.borrow();

    // OPTIMIZATION - `scene_graph.cursor_over: WeakRef<Nodes>` of `hover_state != HoverState::Up` for press and drag events.
    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node = &tree[node_index];

        // TODO - Is there a better fix?
        // https://github.com/rust-lang/rfcs/issues/811
        let hover_state = node.state.borrow().hover_state.clone();
        match hover_state {
            HoverState::Down => {
                set_state(node, HoverState::Hover, scene_graph);
                event::emit_events(node, event::EventType::Pressed);
            }
            HoverState::Drag => {
                // https://github.com/rust-lang/rfcs/issues/811
                let over = cursor_over(&node.geometry.borrow(), args.cursor);
                if over {
                    set_state(node, HoverState::Hover, scene_graph);
                }
                else {
                    set_state(node, HoverState::Up, scene_graph);
                }
                event::emit_events(node, event::EventType::Dragged);
            }
            _ => {}
        }
    }
}
fn cursor_over(geometry: &Geometry, cursor: &Xy) -> bool {geometry.within_border_box(cursor)}

fn set_state<'a>(node: &Rc<node::Node<'a>>, state: HoverState, scene_graph: &SceneGraph<'a>) {
    node.state.borrow_mut().hover_state = state;
    style::maybe_style(node, scene_graph);
}
