mod lib;
mod draw;
mod cell;
mod canvas;
mod terminal_display;
mod windows_display;

use crate::canvas::CanvasHex;
use crate::canvas::CanvasSquare;
use crate::terminal_display::{canvas_hexagons_display, canvas_square_display};
use crate::windows_display::{canvas_square_display_windows, canvas_hexagon_display_windows};

use std::{thread, time};

fn main() {
    let mut status: String;
    loop {
        println!("Which status?(hexagon / square)");
        status = lib::read_one().trim().to_uppercase();

        if status == "HEXAGON".to_string() || status == "SQUARE".to_string() {
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

    let mut cell_size: String;
    loop {
        println!("size of each cell: ");
        cell_size = lib::read_one().trim().to_string();

        if lib::is_number(cell_size.clone()) {
            break
        }

        println!("Invalid status.  Please try again.")
    }
    let cell_size = cell_size.trim().parse::<i32>().expect("Failed to parse int");

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
        // canvas_hexagons_display(row_num, col_num, loop_num);
        canvas_hexagon_display_windows(row_num, col_num, cell_size);
    }
    if status == "SQUARE".to_string() {
        // canvas_square_display(row_num, col_num, loop_num);
        canvas_square_display_windows(row_num, col_num, cell_size);
    }
    
}





