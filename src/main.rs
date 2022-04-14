mod lib;
mod draw;
mod cell;
mod canvas;
mod terminal_display;
mod windows_display;
mod user_input;

use crate::user_input::{square, hexagons};

fn main() {
    let mut status: String;
    println!("Which status?");
    println!("1) hexagon");
    println!("2) square");
    loop {
        status = lib::read_one().trim().to_uppercase();

        if status == "HEXAGON".to_string() || status == "SQUARE".to_string() ||
           status == "1".to_string() || status == "2".to_string() {
            break
        }
        println!("Invalid status.  Please try again.")
    }
    if status == "HEXAGON".to_string() || status == "1".to_string() {
        hexagons();
    }
    if status == "SQUARE".to_string() || status == "2".to_string() {
        square();
    }
    
}





