mod lib;
mod draw;
mod cell;
// use crate::cell::Cell;
mod canvas;
use crate::canvas::CanvasHex;
use crate::canvas::CanvasSquare;
use crate::canvas::CanvasTriangle;

use draw::{draw_rectange};

extern crate rand;
extern crate piston_window;

// use std::io::stdin;
use std::{thread, time};

use piston_window::*;
use piston_window::types::Color;

const BLACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const GRAY_COLOR: Color = [0.2, 0.2, 0.2, 1.0];

fn main() {
    let mut status: String;
    loop {
        println!("Which status?(hexagon / square / triangle)");
        status = lib::read_one().trim().to_uppercase();

        if status == "HEXAGON".to_string() || status == "SQUARE".to_string() || status == "TRIANGLE".to_string() {
            break
        }

        println!("Invalid status.  Please try again.")
    }

    
    let mut row_num: String;
    loop {
        println!("number of rows: ");
        row_num = lib::read_one().trim().to_string();

        if lib::is_number(row_num.clone()) {
            break
        }

        println!("Invalid status.  Please try again.")
    }
    let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

    let mut col_num: String;
    loop {
        println!("number of cols: ");
        col_num = lib::read_one().trim().to_string();

        if lib::is_number(col_num.clone()) {
            break
        }

        println!("Invalid status.  Please try again.")
    }
    let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

    let mut loop_num: String;
    loop {
        println!("number of loops: ");
        loop_num = lib::read_one().trim().to_string();

        if lib::is_number(loop_num.clone()) {
            break
        }

        println!("Invalid status.  Please try again.")
    }
    let loop_num = loop_num.trim().parse::<usize>().expect("Failed to parse int");
    //----------------------------------------------------------------------------------------------
    let separation = "------------------------------------------------------------------------------------";
    
    println!("{}", separation);

    if status == "HEXAGON".to_string() {
        canvas_hexagons_display(row_num, col_num, loop_num);
    }
    if status == "SQUARE".to_string() {
        // canvas_square_display(row_num, col_num, loop_num);
        canvas_square_display_windows(row_num, col_num, 10);
    }
    if status == "TRIANGLE".to_string() {
        canvas_triangle_display(row_num, col_num, loop_num)
    }
    
}
// fn main() {
//     let separation = "------------------------------------------------------------------------------------";
//     let sleep_time = time::Duration::from_millis(1500);
//     // lib::printhello();
//     //  test for Cell
//     let mut cell_a = Cell::new();
//     println!("status: {:?}, x: {}, y: {}", cell_a.is_alive(), cell_a.get_x(), cell_a.get_y());
//     cell_a.set_x(15);
//     cell_a.set_y(20);
//     cell_a.change_status(true);
//     println!("status: {:?}, x: {}, y: {}", cell_a.is_alive(), cell_a.get_x(), cell_a.get_y());
//     //  test for canvas
//     let mut canvas_hexagons = Canvas::new(15, 15, 5.0);
//     // canvas_1.display_in_terminal();
//     // println!("{}", separation);
//     // --------------------------------------------------------------
//     for _num in 0..100 {
//         let x: u8 = rand::random::<u8>() % 15;
//         let y: u8 = rand::random::<u8>() % 15;
//         canvas_hexagons.reverse_status(x.into(), y.into());
//     }
//     canvas_hexagons.display_in_terminal();
//     println!("{}", separation);
//     thread::sleep(sleep_time);
//     // --------------------------------------------------------------
//     canvas_hexagons.next_generation();
//     canvas_hexagons.display_in_terminal();
//     println!("{}", separation);
//     for _num in 0..20 {
//         canvas_hexagons.next_generation();
//         canvas_hexagons.display_in_terminal();
//         println!("{}", separation);
//         thread::sleep(sleep_time);
//     }
//     // --------------------------------------------------------------
// }

fn canvas_hexagons_display(row_num: usize, col_num: usize, loop_num: usize) {
    let separation = "------------------------------------------------------------------------------------";
    let sleep_time = time::Duration::from_millis(1500);
    let mut canvas_hexagons = CanvasHex::new(row_num, col_num, 5.0);
    for _num in 0..150 {
        let x: usize = rand::random::<usize>() % col_num;
        let y: usize = rand::random::<usize>() % (row_num * 2);
        canvas_hexagons.reverse_status(y.into(), x.into());
    }
    canvas_hexagons.display_in_terminal();
    println!("{}", separation);
    thread::sleep(sleep_time);
    for _num in 0..loop_num {
        canvas_hexagons.next_generation();
        canvas_hexagons.display_in_terminal();
        println!("{}", separation);
        thread::sleep(sleep_time);
    }
    canvas_hexagons.do_nothing();
}

fn canvas_square_display(row_num: usize, col_num: usize, loop_num: usize) {
    let separation = "------------------------------------------------------------------------------------";
    let sleep_time = time::Duration::from_millis(1500);
    let mut canvas_squares = CanvasSquare::new(row_num, col_num, 5.0);
    for _num in 0..150 {
        let x: usize = rand::random::<usize>() % col_num;
        let y: usize = rand::random::<usize>() % row_num;
        canvas_squares.reverse_status(y.into(), x.into());
    }
    canvas_squares.display_in_terminal();
    println!("{}", separation);
    thread::sleep(sleep_time);
    for _num in 0..loop_num {
        canvas_squares.next_generation();
        canvas_squares.display_in_terminal();
        println!("{}", separation);
        thread::sleep(sleep_time);
    }
    canvas_squares.do_nothing();
}

fn canvas_triangle_display(row_num: usize, col_num: usize, loop_num: usize) {
    let separation = "------------------------------------------------------------------------------------";
    let sleep_time = time::Duration::from_millis(1500);
    let mut canvas_triangles = CanvasTriangle::new(row_num, col_num, 5);
    for _num in 0..150 {
        let x: usize = rand::random::<usize>() % col_num;
        let y: usize = rand::random::<usize>() % row_num;
        canvas_triangles.reverse_status(y.into(), x.into());
    }
    canvas_triangles.display_in_terminal();
    println!("{}", separation);
    thread::sleep(sleep_time);
    for _num in 0..loop_num {
        canvas_triangles.next_generation();
        canvas_triangles.display_in_terminal();
        println!("{}", separation);
        thread::sleep(sleep_time);
    }
    canvas_triangles.do_nothing();
}

fn canvas_square_display_windows(row_num: usize, col_num: usize, cell_size: i32) {
    // Prepare window settings
    let mut window_settings = WindowSettings::new("Rust Game of Life - square",
    [(((col_num as i32) * cell_size + 10) as u32), (((row_num as i32) * cell_size + 10) as u32)]).exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // Create a canvas
    let mut canvas_squares = CanvasSquare::new(row_num, col_num, cell_size as f64);
    for _num in 0..150 {
        let x: usize = rand::random::<usize>() % col_num;
        let y: usize = rand::random::<usize>() % row_num;
        canvas_squares.reverse_status(y.into(), x.into());
    }
    let mut auto = false;

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::Space {
                if auto {
                    auto = false;
                } else {
                    auto = true;
                }
            }
            if key == Key::Return {
                canvas_squares.next_generation();
            }
        };
        let mut change = false;
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                change = true;
            }
        }
        let mut x = 0.0;
        let mut y = 0.0;
        event.mouse_cursor(|pos| {
            x = pos[0];
            y = pos[1];
        });
        if change {
            println!("clicked - 1");
            canvas_squares.change_state(x, y);
        }

        let mut canvas = canvas_squares.get_canvas();
        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(BLACK_COLOR, g);
            for row in 0..row_num {
                for col in 0..col_num {
                    if canvas[row][col].is_alive() {
                        rectangle(WHITE_COLOR, // red
                            [(col as i32 * cell_size) as f64, 
                             (row as i32 * cell_size) as f64, 
                             cell_size as f64, 
                             cell_size as f64], // rectangle
                            c.transform, g);
                    }
                }
            }
            // rectangle(WHITE_COLOR, // red
            //     [0.0, 0.0, 100.0, 50.0], // rectangle
            //     c.transform, g);
        });
        if auto {
            canvas_squares.next_generation();
        }
    }
}