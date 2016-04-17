use scene_graph::SceneGraph;
use rose_tree::{ROOT, petgraph};
use renderer::geometry::{Xy, Geometry};

pub struct CollisionArgs<'a> {
    pub cursor: &'a Xy,
}

// Keep a quick list of absolutely positioned nodes. Then DFS/BFS search can be used on the `UiTree` and exited early as a bounding box heirarchy.
// On mouse button down - All hover nodes set to down.
// On mouse release - All [down|drag] nodes emit event.
// Assumes mouse move always happens before press/release.
pub fn collision(args: &CollisionArgs, scene_graph: &SceneGraph) {
    let mut tree = scene_graph.tree.borrow_mut();

    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node = &mut tree[node_index];

        // OPTIMIZATION - Store a List of absolutely positioned nodes, then collision only need check those and do a DFS, skipping branches as soon as a `node.is_hover() == false`
        // OPTIMIZATION - `scene_graph.cursor_over: WeakRef<Nodes>` of `hover_state != HoverState::Up` for press and drag events.
        if cursor_over(&node.geometry.borrow(), args.cursor) && node.id != 0 {
            if let HoverState::Up = node.state.hover_state {
                node.state.hover_state = HoverState::Hover;
                node.dirty.set(true);
                println!("Hovered {:?}", node);
            }
        } else {
            match node.state.hover_state {
                HoverState::Hover => {
                    node.state.hover_state = HoverState::Up;
                    node.dirty.set(true);
                }
                HoverState::Down => {
                    node.state.hover_state = HoverState::Drag;
                    node.dirty.set(true);
                }
                _ => {}
            }
        }
    }
}

/// On press - All `HoverState::Hover` nodes set to down.
pub fn press(scene_graph: &SceneGraph) {
    let mut tree = scene_graph.tree.borrow_mut();

    // OPTIMIZATION - `scene_graph.cursor_over: WeakRef<Nodes>` of `hover_state != HoverState::Up` for press and drag events.
    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node = &mut tree[node_index];

        if let HoverState::Hover = node.state.hover_state {
            node.state.hover_state = HoverState::Down;
            node.dirty.set(true);
        }
    }
}

/// On press - All `HoverState::Hover` nodes set to down.
pub fn release(args: &CollisionArgs, scene_graph: &SceneGraph) {
    let mut tree = scene_graph.tree.borrow_mut();

    // OPTIMIZATION - `scene_graph.cursor_over: WeakRef<Nodes>` of `hover_state != HoverState::Up` for press and drag events.
    let mut dfs = petgraph::Dfs::new(tree.graph(), petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(tree.graph()) {
        let node = &mut tree[node_index];

        match node.state.hover_state {
            HoverState::Down => {
                node.state.hover_state = HoverState::Hover;
                node.dirty.set(true);
                println!("Tapped {:?}", node);
            }
            HoverState::Drag => {
                if cursor_over(&node.geometry.borrow(), args.cursor) {
                    node.state.hover_state = HoverState::Hover;
                }
                else {
                    node.state.hover_state = HoverState::Up;
                }
                node.dirty.set(true);
                println!("Dragged {:?}", node);
            }
            _ => {}
        }
    }
}

fn cursor_over(geometry: &Geometry, cursor: &Xy) -> bool {geometry.within_border_box(cursor)}

pub fn hovering(state: &HoverState) -> bool {
    *state == HoverState::Hover || *state == HoverState::Down
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