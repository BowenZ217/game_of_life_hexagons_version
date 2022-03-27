mod lib;

mod cell;
// use crate::cell::Cell;
mod canvas;
use crate::canvas::Canvas;

extern crate rand;

use std::io::stdin;
use std::{thread, time};


fn main() {
    let separation = "------------------------------------------------------------------------------------";
    let sleep_time = time::Duration::from_millis(1500);

    println!("number of rows: ");
    let mut row_num = String::new();
    stdin().read_line(&mut row_num).expect("Failed to get console input");
    let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

    println!("number of cols: ");
    let mut col_num = String::new();
    stdin().read_line(&mut col_num).expect("Failed to get console input");
    let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

    println!("number of loops: ");
    let mut loop_num = String::new();
    stdin().read_line(&mut loop_num).expect("Failed to get console input");
    let loop_num = loop_num.trim().parse::<usize>().expect("Failed to parse int");
    //----------------------------------------------------------------------------------------------
    println!("{}", separation);

    let mut canvas_hexagons = Canvas::new(row_num, col_num, 5.0);
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
