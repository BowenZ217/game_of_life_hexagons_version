mod lib;

mod cell;
use crate::cell::Cell;
mod canvas;
use crate::canvas::Canvas;

extern crate rand;

fn main() {
    lib::printhello();
    //  test for Cell
    let mut cell_a = Cell::new();
    println!("status: {:?}, x: {}, y: {}", cell_a.is_alive(), cell_a.get_x(), cell_a.get_y());
    cell_a.set_x(15);
    cell_a.set_y(20);
    cell_a.change_status(true);
    println!("status: {:?}, x: {}, y: {}", cell_a.is_alive(), cell_a.get_x(), cell_a.get_y());
    //  test for canvas
    let separation = "------------------------------------------------------------------------------";
    let mut canvas_1 = Canvas::new(15, 15, 5.0);
    // canvas_1.display_in_terminal();
    // println!("{}", separation);
    // --------------------------------------------------------------
    for _num in 0..20 {
        let x: u8 = rand::random::<u8>() % 15;
        let y: u8 = rand::random::<u8>() % 15;
        canvas_1.reverse_status(x.into(), y.into());
    }
    // canvas_1.reverse_status(5, 7);
    // canvas_1.reverse_status(6, 7);
    // canvas_1.reverse_status(5, 8);
    // canvas_1.reverse_status(8, 7);
    canvas_1.display_in_terminal();
    println!("{}", separation);
    // --------------------------------------------------------------
    canvas_1.next_generation();
    canvas_1.display_in_terminal();
    println!("{}", separation);
    // --------------------------------------------------------------
}
