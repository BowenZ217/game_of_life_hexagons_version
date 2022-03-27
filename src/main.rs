mod lib;

mod cell;
// use crate::cell::Cell;
mod canvas;
use crate::canvas::CanvasHex;
use crate::canvas::CanvasSquare;

extern crate rand;

// use std::io::stdin;
use std::{thread, time};


fn main() {
    let mut status: String;
    loop {
        println!("Which status?(hex / square)");
        status = lib::read_one().trim().to_uppercase();

        if status == "HEX".to_string() || status == "SQUARE".to_string() {
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

    if status == "HEX".to_string() {
        canvas_hexagons_display(row_num, col_num, loop_num);
    }
    if status == "SQUARE".to_string() {
        canvas_square_display(row_num, col_num, loop_num);
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
    let mut canvas_squares = CanvasSquare::new(row_num, col_num, 5);
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