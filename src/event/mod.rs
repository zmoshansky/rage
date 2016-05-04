use widget;
use scene_graph::node;

// #[derive(Clone)]
pub struct EventHandler {
    pub event: EventType,
    pub callback: fn(&widget::State) -> (),
}

// https://stackoverflow.com/questions/33454425/how-to-clone-a-function-pointer
impl Clone for EventHandler {
    fn clone(&self) -> Self {
        EventHandler {
            event: self.event.clone(),
            callback: self.callback,
        }
    }
}

// TODO - Figure out useful event types.
#[derive(Clone, PartialEq, Debug)]
pub enum EventType {
    Hovering,
    Pressing,
    // AltPressing,
    Dragging,
    Hovered,
    Pressed,
    // AltPressed,
    Dragged,
    // Focused,
    // Custom,
}


pub fn emit_events<'a>(node: &node::Node<'a>, event_type: EventType) {
    println!("Event: {:?} - {:?}", event_type, node);

    if !node.event_handlers.is_empty() {
        for event_handler in node.event_handlers.iter() {
            if event_handler.event == event_type {
                (event_handler.callback)(&node.state.borrow());
            }
        }
    }
}
