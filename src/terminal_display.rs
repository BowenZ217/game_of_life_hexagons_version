use crate::canvas::CanvasHex;
use crate::canvas::CanvasSquare;

extern crate rand;
use std::{thread, time};


pub fn canvas_hexagons_display(row_num: usize, col_num: usize, loop_num: usize) {
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
}

pub fn canvas_square_display(row_num: usize, col_num: usize, loop_num: usize) {
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
}