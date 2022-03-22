mod lib;

mod cell;
// mod canvas;
use crate::cell::Cell;
// use crate::canvas::Canvas;

fn main() {
    lib::printhello();
    //
    let mut cell_a = Cell::new();
    println!("status: {:?}, x: {}, y: {}", cell_a.is_alive(), cell_a.get_x(), cell_a.get_y());
    //
    cell_a.set_x(15);
    cell_a.set_y(20);
    cell_a.change_status(true);
    println!("status: {:?}, x: {}, y: {}", cell_a.is_alive(), cell_a.get_x(), cell_a.get_y());
}
