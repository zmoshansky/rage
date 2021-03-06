#![feature(rc_counts)]

extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate rose_tree;
extern crate sdl2_window;

use std::path::Path;

use piston_window::{PistonWindow, WindowSettings, Glyphs};
use piston::input::*;
use sdl2_window::Sdl2Window;

mod scene_graph;
mod style;
mod event;
mod widget;
mod renderer;
mod appearance;
mod layout;
mod collision;
mod test_fixture;

use scene_graph::SceneGraph;
use layout::Cartographer;
use collision::CollisionArgs;

use renderer::{image, geometry};
use renderer::Renderer;

const WIDTH: u32 = 958;
const HEIGHT: u32 = 535;

fn main() {
    // let window: PistonWindow =
    let window: PistonWindow<(), Sdl2Window> =
        WindowSettings::new("Fedora Project - Start Page - Rage", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();

    // Load text
    // TODO - Font Loader
    // let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    // let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let font_path = Path::new("assets/fonts/NotoSans/NotoSans-Regular.ttf");
    let mut glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone()).unwrap();

    // Image Cache
    let mut image_cache = image::ImageCache::default();
    image_cache.load_image_from_path(&window, "assets/images/rust.png");
    image_cache.load_image_from_path(&window, "assets/images/yt_favicon.png");
    image_cache.load_image_from_path(&window, "assets/images/page.png");
    image_cache.load_image_from_path(&window, "assets/icons/close.png");
    image_cache.load_image_from_path(&window, "assets/icons/plus.png");
    image_cache.load_image_from_path(&window, "assets/icons/icons_right.png");
    image_cache.load_image_from_path(&window, "assets/icons/left_arrow.png");

    // Capture mouse coordinates
    let mut cursor = geometry::Xy::default();
    let mut window_size = geometry::Xy{x: WIDTH as f64, y: HEIGHT as f64};

    // Create Scene Graph
    let (mut scene_graph, _) = SceneGraph::new();
    test_fixture::web_browser(&mut scene_graph);

    for e in window {

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                collision::press(&scene_graph);
            }
            // DEBUG
            if button == Button::Mouse(MouseButton::Right) {
                debug(&scene_graph);
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

        // Only occurs if mouse button down while leaving window frame... CollisionState::Drag.
        // Only works properly in SDL2
        if let Some(cursor) = e.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else {
                println!("Mouse left");
                debug(&scene_graph);
            }
        };

        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| {
            println!("Resized '{}, {}'", w, h);
            window_size = geometry::Xy{x: w as f64, y: h as f64};
            scene_graph.layout_root();
            layout::layout_root(&mut Cartographer{window: &window_size, glyphs: &mut glyph_cache, images: &mut image_cache, dpi: &geometry::Xy{x:96.0, y: 96.0}}, &scene_graph);
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

        e.update(|_| {
            style::style(&scene_graph);
            layout::layout_root(&mut Cartographer{window: &window_size, glyphs: &mut glyph_cache, images: &mut image_cache, dpi: &geometry::Xy{x:96.0, y: 96.0}}, &scene_graph);
        });

        e.draw_2d(|c, g| {
            renderer::render(&mut Renderer{context: c, graphics: g, glyphs: &mut glyph_cache, images: &mut image_cache}, &scene_graph);
        });

        // Only works with SDL2 Window
        // e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
    }
}

fn debug(scene_graph: &SceneGraph) {
    scene_graph.debug_print_ref_counts();
    let mut focused = scene_graph.focused.borrow_mut();
    if let Some(weak) = focused.as_mut() {
        println!("{:?}", weak.upgrade());
    }
}