extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate rose_tree;
use std::path::Path;
use std::cell::Cell;
use std::cell::RefCell;

// use piston_window::*;
use piston_window::{PistonWindow, WindowSettings, Glyphs};
use piston::input::*;

mod graph_node;
mod widget;
mod renderer;
mod appearance;
mod geometry;
mod layout;
// mod test_fixture;

use graph_node::GraphNode;
use widget::{Widget, State};
use widget::div::Div;
use widget::button::{Button as WButton, Background};

use layout::Cartographer;

use geometry::dimension::{Dimensions, Dimension};
use geometry::Geometry as GeometryUncached;

use renderer::Renderer;
use renderer::geometry::{Geometry, Xy, Xyz};

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
    let mut cursor = [0.0, 0.0];
    let mut window_size = Xy{x: WIDTH as f64, y: HEIGHT as f64};


    let types: [Box<Widget>; 3] = [Box::new(Background), Box::new(WButton), Box::new(Div)];

    let root = GraphNode{type_id: 0, dirty: Cell::new(true), ..Default::default()};
    let (mut tree, root) = rose_tree::RoseTree::<GraphNode, u32>::new(root);
    let a = tree.add_child(root, GraphNode{
        state: State{text: "A`y", ..Default::default()},
        type_id: 2,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::DisplayPixel(400.0), y: Dimension::Percent(0.3), ..Default::default()},
            ..Default::default()
        },
        geometry: RefCell::new(Geometry{
            position: Xyz{
                // x: 40.0,
                // y: 200.0,
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    });

    tree.add_child(a, GraphNode{
        state: State{text: "A`y", ..Default::default()},
        type_id: 2,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(0.50), y: Dimension::Percent(0.5), ..Default::default()},
            ..Default::default()
        },
        geometry: RefCell::new(Geometry{
            position: Xyz{
                x: 0.0,
                y: 200.0,
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    });

    tree.add_child(a, GraphNode{
        state: State{text: "A`y", ..Default::default()},
        type_id: 2,
        geometry_uncached: GeometryUncached{
            dimensions: Dimensions{x: Dimension::Grid(1.0), y: Dimension::Percent(0.5), ..Default::default()},
            ..Default::default()
        },
        geometry: RefCell::new(Geometry{
            position: Xyz{
                x: 0.0,
                y: 300.0,
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    });
    // tree.add_child(a, GraphNode{type_id: 2, geometry_uncached: GeometryUncached{dimensions: Dimensions{x: Dimension::Percent(0.5), y: Dimension::Percent(0.30), ..Default::default()}, ..Default::default()}, ..Default::default()});


    // Simulate post layout/style pass.
    // let (mut tree, root) = test_fixture::divs();

    // tree.add_child(root, GraphNode{state: State{text: "A`y", ..Default::default()}, geometry: Geometry{position: Xyz{x: 11.0, y: 43.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.0, y: 1.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.1, y: 22.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.2, y: 43.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.3, y: 64.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.4, y: 85.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.5, y: 106.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.6, y: 127.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.7, y: 148.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.8, y: 169.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 10.9, y: 190.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", ..Default::default()}, geometry: Geometry{position: Xyz{x: 11.0, y: 211.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});
    // tree.add_child(root, GraphNode{state: State{text: "A`y", ..Default::default()}, geometry: Geometry{position: Xyz{x: 11.0, y: 43.0}, ..default_layout()}, type_id: 1, dirty: Cell::new(true), ..Default::default()});


    for e in window {

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                println!("Pressed mouse button '{:?}'", button);
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                let dfs = rose_tree::petgraph::DfsIter::new(tree.graph(), root);
                for node_index in dfs {
                    let node = tree.node_weight(node_index).unwrap();
                    if node.geometry.borrow().contained(cursor) {
                        if node.type_id != 0 && !node.state.hover.get() {
                            node.state.hover.set(true);
                            node.dirty.set(true);
                        }

                        println!("Tapped {:?} {:?}", node, cursor);
                    } else {
                        if node.state.hover.get() {
                            node.state.hover.set(false);
                            node.dirty.set(true);
                        }
                    }
                }
                // println!("Released mouse button '{:?}'", button);
            }
        };

        e.mouse_cursor(|x, y| {
            cursor = [x, y];
            let dfs = rose_tree::petgraph::DfsIter::new(tree.graph(), root);
            for node_index in dfs {
                let node = tree.node_weight(node_index).unwrap();
                if node.geometry.borrow().contained(cursor) {
                    if node.type_id != 0 && !node.state.hover.get() {
                        node.state.hover.set(true);
                        node.dirty.set(true);
                    }
                } else {
                    if node.state.hover.get() {
                        node.state.hover.set(false);
                        node.dirty.set(true);
                    }
                }
            }
        });
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| {
            window_size = Xy{x: w as f64, y: h as f64};
            layout::layout(&Cartographer{window: &window_size, dpi: &Xy{x:96.0, y: 96.0}}, &tree, root);
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
            let dfs = rose_tree::petgraph::DfsIter::new(tree.graph(), root);
            for node_index in dfs {
                let node = tree.node_weight(node_index).unwrap();
                if node.dirty.get() {
                    types[node.type_id].render(Renderer{context: c, graphics: g, glyphs: &mut glyph_cache}, &node.appearance, &node.geometry.borrow(), &node.state);
                    node.dirty.set(false);
                }
            }
        });

        // Not yielding events
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        if let Some(cursor) = e.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse leaved"); }
        };

    }
}

fn default_layout() -> Geometry {Geometry{dimensions: Xyz{x: 10.0, y: 10.0}, ..Default::default()}}
