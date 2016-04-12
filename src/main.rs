extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate rose_tree;
use std::path::Path;
use std::cell::Cell;

// use piston_window::*;
use piston_window::{PistonWindow, WindowSettings, Glyphs};
use piston::input::*;

mod graph_node;
mod tree;
mod widget;
mod renderer;
mod appearance;
mod geometry;
mod layout;
mod collision;
// mod test_fixture;

use graph_node::GraphNode;
use widget::State;
use widget::div::Div;
use widget::button::{Button as WButton, Background};

use tree::Tree;
use layout::Cartographer;
use collision::CollisionArgs;

use geometry::dimension::{Dimensions, Dimension};
use geometry::Geometry as GeometryUncached;

use renderer::Renderer;
use renderer::geometry::Xy;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let window: PistonWindow =
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

    let (mut ui_tree, root) = Tree::new(GraphNode{id: Cell::new(0), type_id: 0, dirty: Cell::new(true), ..Default::default()});
    ui_tree.types.push(Box::new(Background));
    ui_tree.types.push(Box::new(WButton));
    ui_tree.types.push(Box::new(Div));

    let a = ui_tree.add_child(root, GraphNode{
        type_id: 2,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Percent(1.0), y: Dimension::DisplayPixel(100.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    ui_tree.add_child(a, GraphNode{
        state: State{text: "A`y", ..Default::default()},
        type_id: 1,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::DisplayPixel(400.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    ui_tree.add_child(a, GraphNode{
        type_id: 2,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::DisplayPixel(400.0)},
            ..Default::default()
        },
        ..Default::default()
    });

    ui_tree.add_child(a, GraphNode{
        state: State{text: "B", ..Default::default()},
        type_id: 1,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::DisplayPixel(400.0), ..Default::default()},
            ..Default::default()
        },
        ..Default::default()
    });

    ui_tree.add_child(a, GraphNode{
        type_id: 2,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::Viewport(1.0), ..Default::default()},
            ..Default::default()
        },
        ..Default::default()
    });

    for e in window {

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                println!("Pressed mouse button '{:?}'", button);
            }
        };
        // if let Some(button) = e.release_args() {
        //     if button == Button::Mouse(MouseButton::Left) {
        //         let dfs = rose_tree::petgraph::DfsIter::new(ui_tree.tree.borrow().graph(), root);
        //         for node_index in dfs {
        //             let node = ui_tree.tree.borrow().node_weight(node_index).unwrap();
        //             if node.geometry.borrow().contained(cursor) {
        //                 if node.type_id != 0 && !node.state.hover.get() {
        //                     node.state.hover.set(true);
        //                     node.dirty.set(true);
        //                 }

        //                 println!("Tapped {:?} {:?}", node, cursor);
        //             } else {
        //                 if node.state.hover.get() {
        //                     node.state.hover.set(false);
        //                     node.dirty.set(true);
        //                 }
        //             }
        //         }
        //         // println!("Released mouse button '{:?}'", button);
        //     }
        // };

        e.mouse_cursor(|x, y| {
            cursor.x = x;
            cursor.y = y;
            collision::collision(CollisionArgs{cursor: &cursor}, &ui_tree);
            // let mut tree = ui_tree.tree.borrow();
            // let dfs = rose_tree::petgraph::DfsIter::new(tree.graph(), root);
            // for node_index in dfs {
            //     let node = tree.node_weight_mut(node_index).unwrap();
            //     if node.geometry.borrow().contained(cursor) {
            //         if node.type_id != 0 && !node.state.hover.get() {
            //             node.state.hover.set(true);
            //             node.dirty.set(true);
            //         }
            //     } else {
            //         if node.state.hover.get() {
            //             node.state.hover.set(false);
            //             node.dirty.set(true);
            //         }
            //     }
            // }
        });
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| {
            window_size = Xy{x: w as f64, y: h as f64};
            layout::layout(&Cartographer{window: &window_size, dpi: &Xy{x:96.0, y: 96.0}}, &ui_tree.tree.borrow(), root);
            println!("Resized '{}, {}'", w, h)
        });
        if let Some(focused) = e.focus_args() {
            if focused { println!("Gained focus"); }
            else { println!("Lost focus"); }
        };

        // e.update(|_| {
        //     println!("Update");
        // });
        e.draw_2d(|c, g| {
            renderer::render(Renderer{context: c, graphics: g, glyphs: &mut glyph_cache}, &ui_tree);
        });

        // Not yielding events
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        if let Some(cursor) = e.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse leaved"); }
        };

    }
}
