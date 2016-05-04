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
pub enum CollisionState {
    Up,
    Hover,
    Down,
    Drag
}

impl Default for CollisionState {
    fn default() -> CollisionState {CollisionState::Up}
}

// Keep a quick list of absolutely positioned nodes. Then DFS/BFS search can be used on the `UiTree` and exited early as a bounding box heirarchy.
pub fn collision(args: &CollisionArgs, scene_graph: &SceneGraph) {
    // OPTIMIZATION - Store a List of absolutely positioned nodes, then collision only need check those and do a DFS, skipping branches as soon as a `node.is_hover() == false`
    let tree = scene_graph.tree.borrow();

    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node: &Rc<node::Node> = &tree[node_index];

        // TODO - Is there a better fix?
        // https://github.com/rust-lang/rfcs/issues/811
        let hover_state = node.state.borrow().hover_state.clone();
        let over = cursor_over(&node.geometry.borrow(), args.cursor);
        if over && node.id != 0 {
            if let CollisionState::Up = hover_state {
                set_state(node, CollisionState::Hover, scene_graph);
                event::emit_events(node, event::EventType::Hovering);
            }
        }
    }

    // https://github.com/rust-lang/rfcs/issues/811
    let hovering = &mut scene_graph.hover_nodes().borrow_mut();
    hovering.retain(|weak|
        if let Some(ref node) = weak.upgrade() {
            let over = cursor_over(&node.geometry.borrow(), args.cursor);
            if over {
                true
            } else {
                set_state(node, CollisionState::Up, scene_graph);
                event::emit_events(node, event::EventType::Hovered);
                false
            }
        } else {
            false
        }
    );

    // https://github.com/rust-lang/rfcs/issues/811
    let down = &mut scene_graph.down_nodes().borrow_mut();
    down.retain(|weak|
        if let Some(ref node) = weak.upgrade() {
            let over = cursor_over(&node.geometry.borrow(), args.cursor);
            if over {
                true
            } else {
                set_state(node, CollisionState::Drag, scene_graph);
                event::emit_events(node, event::EventType::Dragging);
                false
            }
        } else {
            false
        }
    );
}

/// On press - All `CollisionState::Hover` nodes set to down.
pub fn press(scene_graph: &SceneGraph) {
    // https://github.com/rust-lang/rfcs/issues/811
    let hovering = &mut scene_graph.hover_nodes().borrow_mut();
    let iter = hovering.drain(..);
    for weak in iter {
        if let Some(ref node) = weak.upgrade() {
            set_state(node, CollisionState::Down, scene_graph);
            event::emit_events(node, event::EventType::Pressing);
        }
    }
}

/// On press - All `CollisionState::Hover` nodes set to down.
pub fn release(args: &CollisionArgs, scene_graph: &SceneGraph) {
    // https://github.com/rust-lang/rfcs/issues/811
    let drag = &mut scene_graph.drag_nodes().borrow_mut();
    let iter = drag.drain(..);
    for weak in iter {
        if let Some(ref node) = weak.upgrade() {
            // https://github.com/rust-lang/rfcs/issues/811
            let over = cursor_over(&node.geometry.borrow(), args.cursor);
            if over {
                set_state(node, CollisionState::Hover, scene_graph);
                event::emit_events(node, event::EventType::Hovering);
            }
            else {
                set_state(node, CollisionState::Up, scene_graph);
            }
            event::emit_events(node, event::EventType::Dragged);
        }
    }

    // https://github.com/rust-lang/rfcs/issues/811
    let down = &mut scene_graph.down_nodes().borrow_mut();
    let iter = down.drain(..);
    for weak in iter {
        if let Some(ref node) = weak.upgrade() {
            set_state(node, CollisionState::Hover, scene_graph);
            event::emit_events(node, event::EventType::Pressed);

            // TODO - Improve this, limit to only input nodes, otherwise none.
            scene_graph.focus(node);
        }
    }
}

fn cursor_over(geometry: &Geometry, cursor: &Xy) -> bool {geometry.within_border_box(cursor)}

fn set_state<'a>(node: &Rc<node::Node<'a>>, state: CollisionState, scene_graph: &SceneGraph<'a>) {
    match state {
        CollisionState::Hover => scene_graph.hover(node),
        CollisionState::Down => scene_graph.down(node),
        CollisionState::Drag => scene_graph.drag(node),
        _ => {},
    }
    node.state.borrow_mut().hover_state = state;
    style::maybe_style(node, scene_graph);
}
