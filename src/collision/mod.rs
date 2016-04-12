use tree::Tree;
use rose_tree::{ROOT, petgraph};
use renderer::geometry::{Xy, Geometry};

pub struct CollisionArgs<'a> {
    pub cursor: &'a Xy,
}

// Keep a quick list of absolutely positioned nodes. Then DFS/BFS search can be used on the `UiTree` and exited early as a bounding box heirarchy.
// On mouse button down - All hover nodes set to down.
// On mouse release - All [down|drag] nodes emit event.
// Assumes mouse move always happens before press/release.
pub fn collision(args: CollisionArgs, ui_tree: &Tree) {
    let mut tree = ui_tree.tree.borrow_mut();
    let graph = tree.graph_mut();

    // OPTIMIZATION - Store a List of absolutely positioned nodes, then collision only need check those and do a DFS, skipping branches as soon as a `node.is_hover() == false`
    // OPTIMIZATION - Maintain a list `WeakRef<Nodes>` of `hover_state != HoverState::Up` for press and drag events.
    let mut dfs = petgraph::Dfs::new(graph, petgraph::graph::NodeIndex::new(ROOT));
    while let Some(node_index) = dfs.next(graph) {
        let node = graph.node_weight_mut(node_index).unwrap();
        if hovering(&node.geometry.borrow(), &args.cursor) {
            if let HoverState::Up = node.state.hover_state {
                node.state.hover_state = HoverState::Hover;
                node.dirty.set(true);
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


/// Uses W3C Content-Box by default
fn hovering(geometry: &Geometry, cursor: &Xy) -> bool {
    // TODO - Account for alternate bounding models - http://www.binvisions.com/articles/box-sizing-property-difference-content-border/
    // TODO - Use bounding models to determine whether in or out.
    geometry.position.x <= cursor.x && geometry.position.x + geometry.dimensions.x  >= cursor.x &&
    geometry.position.y <= cursor.y && geometry.position.y + geometry.dimensions.y >= cursor.y
}

#[derive(Debug, PartialEq)]
pub enum HoverState {
    Up,
    Hover,
    Down,
    Drag
}
impl Default for HoverState {
    fn default() -> HoverState {HoverState::Up}
}