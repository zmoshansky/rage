use graph_node::GraphNode;
use renderer::Renderer;

pub trait Widget {
  // _: Option<Self>
  fn render(renderer: Renderer, node: &GraphNode);
}