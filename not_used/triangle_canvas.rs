use std::{thread, time};
// use crate::canvas::CanvasTriangle;

#[derive(Debug, Default)]
pub struct CanvasTriangle {
    row_num: usize,
    col_num: usize,
    cell_side_length: i32,
    display: Vec<Vec<Cell>>
}

impl CanvasTriangle {
    pub fn new(row_num_set: usize, col_num_set: usize, side_length: i32) -> CanvasTriangle {
        // create 2d vector for cells
        let display_vector: Vec<Vec<Cell>> = vec![vec![Cell::new(); col_num_set]; row_num_set];
        return CanvasTriangle {
            row_num: row_num_set,
            col_num: col_num_set,
            cell_side_length: side_length,
            display: display_vector
        };
    }
    pub fn next_generation(&mut self) {
        // todo!();
        let mut next = self.display.clone();
        for row in 0..self.row_num {
            for col in 0..self.col_num {
                let next_state = CanvasTriangle::check(self, row, col);
                next[row][col].change_status(next_state);
            }
        }
        self.display = next;
    }

    //  let user change the cell's status in canvas
    pub fn reverse_status(&mut self, row: usize, col: usize) {
        self.display[row][col].reverse_status();
    }
    // a easy way to display it out in terminal
    pub fn display_in_terminal(&mut self) {
        for row in 0..self.row_num {
            for col in 0..self.col_num {
                print!("{} ", self.get_status_string(row, col));
            }
            print!("\n");
        }
    }

    pub fn do_nothing(&mut self) {
        self.cell_side_length;
    }
    // helper
    fn get_status_string(&mut self, row: usize, col: usize) -> &str {
        let odd_alive = "▲";
        let even_alive = "▼";
        // let odd_dead = "∆";
        // let even_dead = "∇";
        let odd_dead = " ";
        let even_dead = " ";
        if row % 2 == 0 && col % 2 != 0 || row % 2 != 0 && col % 2 == 0 {
            if self.display[row][col].is_alive() {
                return even_alive;
            }
            else {
                return even_dead;
            }
        }
        else {
            if self.display[row][col].is_alive() {
                return odd_alive;
            }
            else {
                return odd_dead;
            }
        }
    }

    fn alive_nei_num(&mut self, row: usize, col: usize) -> i32 {
        let mut count = 0;
        if row % 2 == 0 {
            // odd rows
            if col % 2 == 0 {
                // odd cols
                if row > 0 && self.display[row - 1][col].is_alive() {
                    count += 1;
                }
                if col > 0 && self.display[row][col - 1].is_alive() {
                    count += 1;
                }
                if col < self.col_num - 1 && self.display[row][col + 1].is_alive() {
                    count += 1;
                }
                if row < self.row_num - 1 {
                    if self.display[row + 1][col].is_alive() {
                        count += 1;
                    }
                    if col > 0 && self.display[row + 1][col - 1].is_alive() {
                        count += 1;
                    }
                    if col < self.col_num - 1 && self.display[row + 1][col + 1].is_alive() {
                        count += 1;
                    }
                }
            }
            else {
                // even cols
                if row > 0 {
                    if self.display[row - 1][col].is_alive() {
                        count += 1;
                    }
                    if col > 0 && self.display[row - 1][col - 1].is_alive() {
                        count += 1;
                    }
                    if col < self.col_num - 1 && self.display[row - 1][col + 1].is_alive() {
                        count += 1;
                    }
                }
                if row < self.row_num - 1 && self.display[row + 1][col].is_alive() {
                    count += 1;
                }
                if col > 0 && self.display[row][col - 1].is_alive() {
                    count += 1;
                }
                if col < self.col_num - 1 && self.display[row][col + 1].is_alive() {
                    count += 1;
                }
            }
        }
        else {
            // even rows
            if col % 2 == 0 {
                // odd cols
                if row > 0 {
                    if self.display[row - 1][col].is_alive() {
                        count += 1;
                    }
                    if col > 0 && self.display[row - 1][col - 1].is_alive() {
                        count += 1;
                    }
                    if col < self.col_num - 1 && self.display[row - 1][col + 1].is_alive() {
                        count += 1;
                    }
                }
                if row < self.row_num - 1 && self.display[row + 1][col].is_alive() {
                    count += 1;
                }
                if col > 0 && self.display[row][col - 1].is_alive() {
                    count += 1;
                }
                if col < self.col_num - 1 && self.display[row][col + 1].is_alive() {
                    count += 1;
                }
            }
            else {
                // even cols
                if row > 0 && self.display[row - 1][col].is_alive() {
                    count += 1;
                }
                if col > 0 && self.display[row][col - 1].is_alive() {
                    count += 1;
                }
                if col < self.col_num - 1 && self.display[row][col + 1].is_alive() {
                    count += 1;
                }
                if row < self.row_num - 1 {
                    if self.display[row + 1][col].is_alive() {
                        count += 1;
                    }
                    if col > 0 && self.display[row + 1][col - 1].is_alive() {
                        count += 1;
                    }
                    if col < self.col_num - 1 && self.display[row + 1][col + 1].is_alive() {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    // rules
    fn check(&mut self, row: usize, col: usize) -> bool {
        // todo!();
        //  true    means   alive
        //  false   means   dead
        let neighbor_num = CanvasTriangle::alive_nei_num(self, row, col);
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
}

pub fn canvas_triangle_display(row_num: usize, col_num: usize, loop_num: usize) {
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
