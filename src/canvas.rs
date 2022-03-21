// pub const NUM_SIDE_TOTAL : usize = 6;

mod cell;
use crate::cell::Cell;

#[derive(Debug, Default)]
pub struct Canvas {
    height: f64,
    width: f64,
    cells_vertical: i32,
    cells_horizontal: i32,
    cell_side_length: f64,
    display: Vec<Vec<Cell>>
}

impl Canvas {
    pub fn new(cells_vertical: i32, cells_horizontal: i32, side_length: f64) {
        let display_vector: Vec<Vec<Cells>> = vec![vec![Cell::new(); cells_horizontal]; cells_vertical];
        return Canvas {
            height: side_length*(3.sqrt())*cells_vertical,
            width: side_length*cells_horizontal,
            cells_vertical: cells_vertical,
            cells_horizontal: cells_horizontal,
            cell_side_length: cell_side_length,
            display: display_vector
        };
    }

    pub fn next_generation() {
        todo!();
    }

    pub fn display_canvas() {
        todo!()
        // Ask at office hours
    }

}