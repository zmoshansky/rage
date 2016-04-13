extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate rose_tree;
extern crate sdl2_window;

use std::path::Path;

use piston_window::{PistonWindow, WindowSettings, Glyphs};
use piston::input::*;
use sdl2_window::Sdl2Window;

mod graph_node;
mod scene_graph;
mod widget;
mod renderer;
mod appearance;
mod geometry;
mod layout;
mod collision;
mod test_fixture;

use scene_graph::SceneGraph;
use layout::Cartographer;
use collision::CollisionArgs;

use renderer::Renderer;
use renderer::geometry::Xy;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    // let window: PistonWindow =
    let window: PistonWindow<(), Sdl2Window> =
        WindowSettings::new("Rage", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    // Load text
    // TODO - Font Loader
    // let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let font_path = Path::new("assets/fonts/NotoSans/NotoSans-Regular.ttf");
    let mut glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone()).unwrap();

    // Capture mouse coordinates
    let mut cursor = Xy::default();
    let mut window_size = Xy{x: WIDTH as f64, y: HEIGHT as f64};

    let (mut scene_graph, _) = SceneGraph::new();
    test_fixture::web_browser(&mut scene_graph);

    // SDL2 Window doesn't resize on start, but this event does happen
    layout::layout_root(&Cartographer{window: &window_size, dpi: &Xy{x:96.0, y: 96.0}}, &scene_graph);

    for e in window {

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                collision::press(&scene_graph);
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                collision::release(&CollisionArgs{cursor: &cursor}, &scene_graph);
            }
        };

        // Stops getting updates when mouse leaves window frame, unless mouse button down
        e.mouse_cursor(|x, y| {
            cursor.x = x;
            cursor.y = y;
            collision::collision(&CollisionArgs{cursor: &cursor}, &scene_graph);
        });

        // Only occurs if mouse button down while leaving window frame... HoverState::Drag.
        // Only works properly in SDL2
        if let Some(cursor) = e.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse left"); }
        };

        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| {
            window_size = Xy{x: w as f64, y: h as f64};
            layout::layout_root(&Cartographer{window: &window_size, dpi: &Xy{x:96.0, y: 96.0}}, &scene_graph);
            println!("Resized '{}, {}'", w, h)
        });

        // Focus is gained/lost consecutively if alt-tab is used.
        if let Some(focused) = e.focus_args() {
            if focused {
                println!("Gained focus");
            }
            else {
                // TODO - Set all hover states to HOVER::Up
                println!("Lost focus");
            }
        };

        // e.update(|_| {
        //     println!("Update");
        // });
        e.draw_2d(|c, g| {
            renderer::render(Renderer{context: c, graphics: g, glyphs: &mut glyph_cache}, &scene_graph);
        });

        // Only works with SDL2 Window
        // e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
    }
}
