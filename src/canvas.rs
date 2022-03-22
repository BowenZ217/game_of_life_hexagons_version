// pub const NUM_SIDE_TOTAL : usize = 6;

mod cell;
use crate::cell::Cell;

#[derive(Debug, Default)]
pub struct Canvas {
    height: f64,
    width: f64,
    cells_vertical: usize,
    cells_horizontal: usize,
    cell_side_length: f64,
    display: Vec<Vec<Cell>>
}

impl Canvas {
    pub fn new(cells_vertical: usize, cells_horizontal: usize, side_length: f64) -> Canvas {
        cells_vertical *= 2;
        let display_vector: Vec<Vec<Cell>> = vec![vec![Cell::new(); cells_horizontal]; cells_vertical];
        set_position(display_vector);
        // let display_vector: Vec<Vec<Cell>> = Vec::new();
        return Canvas {
            height: side_length*((3 as f64).sqrt()) * (cells_vertical as f64),
            width: side_length * (cells_horizontal as f64),
            cells_vertical: cells_vertical,
            cells_horizontal: cells_horizontal,
            cell_side_length: side_length,
            display: display_vector
        };
    }

    fn set_position(canvas: Vec<Vec<Cell>>, cell_side_length: f64) {
        // Calculate the position for each cell
        //                 _____       _____       _____       _____
        //           _____/ 0,0 \_____/ 0,1 \_____/ 0,2 \_____/ 0,3 \
        //          / 1,0 \_____/ 1,1 \_____/ 1,2 \_____/ 1,3 \_____/
        //          \_____/ 2,0 \_____/ 2,1 \_____/ 2,2 \_____/ 2,3 \
        //          / 3,0 \_____/ 3,1 \_____/ 3,2 \_____/ 3,3 \_____/
        //          \_____/ 4,0 \_____/ 4,1 \_____/ 4,2 \_____/ 4,3 \
        //          / 5,0 \_____/ 5,1 \_____/ 5,2 \_____/ 5,3 \_____/
        //          \_____/ 6,0 \_____/ 6,1 \_____/ 6,2 \_____/ 6,3 \
        //          / 7,0 \_____/ 7,1 \_____/ 7,2 \_____/ 7,3 \_____/
        //          \_____/     \_____/     \_____/     \_____/
        
        todo!();
    }

    // means cells_vertical % 2 == 0
    fn alive_nei_num_even(&mut self, row: usize, col: usize) -> i32 {
        let mut count = 0;
        if row % 2 == 0 {
            // Even row
            //         (-2, 0)
            // (-1, 0) /~~~~~\ (-1, 1)
            // (1, 0)  \_____/ (1, 1)
            //          (2, 0)
            if row > 0 {
                if self.display[row - 2][col].is_alive() {
                    count += 1;
                }
                if self.display[row - 1][col].is_alive() {
                    count += 1;
                }
                if col < (self.cells_horizontal - 1) && self.display[row - 1][col + 1].is_alive() {
                    count += 1;
                }
            }
            if row < (self.cells_vertical - 2) && self.display[row + 2][col].is_alive() {
                count += 1;
            }
            if self.display[row + 1][col].is_alive() {
                count += 1;
            }
            if col < (self.cells_horizontal - 1) && self.display[row + 1][col + 1].is_alive() {
                count += 1;
            }
        }
        else {
            if row > 1 && self.display[row - 2][col].is_alive() {
                count += 1;
            }
            if row < (self.cells_vertical - 1) {
                if self.display[row + 2][col].is_alive() {
                    count += 1;
                }
                if col > 0 && self.display[row + 1][col - 1].is_alive() {
                    count += 1;
                }
                if self.display[row + 1][col].is_alive() {
                    count += 1;
                }
            }
            if col > 0 && self.display[row - 1][col - 1].is_alive() {
                count += 1;
            }
            if self.display[row - 1][col].is_alive() {
                count += 1;
            }
        }
        count
    }


    // rules
    fn check(&mut self, row: usize, col: usize) -> bool {
        todo!();
        //  true    means   alive
        //  false   means   dead
        let neighbor_num = alive_nei_num_even(row, col);
        if self.display[row][col].is_alive() {
            // 
            if neighbor_num == 3 {
                return true;
            }
            if neighbor_num == 5 {
                return true;
            }
        }
        else {
            //  reproduction
            if neighbor_num == 2 {
                return true;
            }
        }
        return false;
    }
    pub fn next_generation(&mut self) {
        todo!();
        let mut next = self.display.clone();
        for row in 0..self.cells_vertical {
            for col in 0..self.cells_horizontal {
                let next_state = check(row, col);
                next[row][col].change_status(next_state);
            }
        }
        self.display = next;
    }

    pub fn display_canvas() {
        todo!()
        // Ask at office hours
    }

}