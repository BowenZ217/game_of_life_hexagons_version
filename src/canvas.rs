// pub const NUM_SIDE_TOTAL : usize = 6;

// mod cell;
use crate::cell::Cell;

#[derive(Debug, Default)]
pub struct CanvasHex {
    height: f64,
    width: f64,
    cells_vertical: usize,
    cells_horizontal: usize,
    cell_side_length: f64,
    display: Vec<Vec<Cell>>
}

impl CanvasHex {
    pub fn new(cells_vertical_set: usize, cells_horizontal: usize, side_length: f64) -> CanvasHex {
        // or it will be different shape for even and odd rows
        let cells_vertical = cells_vertical_set * 2;
        // create 2d vector for cells, also set up the position
        let display_vector: Vec<Vec<Cell>> = CanvasHex::get_set_position(cells_vertical, cells_horizontal, side_length);
        // let display_vector: Vec<Vec<Cell>> = Vec::new();
        return CanvasHex {
            height: side_length*((3 as f64).sqrt()) * (cells_vertical as f64),
            width: side_length * (cells_horizontal as f64),
            cells_vertical: cells_vertical,
            cells_horizontal: cells_horizontal,
            cell_side_length: side_length,
            display: display_vector
        };
    }

    fn get_set_position(cells_vertical: usize, cells_horizontal: usize, cell_side_length: f64) -> Vec<Vec<Cell>> {
        // todo!();

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
        let mut display_vector: Vec<Vec<Cell>> = vec![vec![Cell::new(); cells_horizontal]; cells_vertical];
        for row in 0..cells_vertical {
            for col in 0..cells_horizontal {
                // unfinished   !!!

                let x = cell_side_length + 3*col*cell_side_length;

                // sqrt(3)/2 approx = 0.8660254
                let y =  0.8660254 * cell_side_length + 2*row*side_length;
                display_vector[row][col].set_x(x);
                display_vector[row][col].set_y(y);
            }
        }
        return display_vector;
    }

    // means cells_vertical % 2 == 0
    // get number of alive neighbors of that cell
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
        // todo!();
        //  true    means   alive
        //  false   means   dead
        let neighbor_num = CanvasHex::alive_nei_num_even(self, row, col);
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
        // todo!();
        let mut next = self.display.clone();
        for row in 0..self.cells_vertical {
            for col in 0..self.cells_horizontal {
                let next_state = CanvasHex::check(self, row, col);
                next[row][col].change_status(next_state);
            }
        }
        self.display = next;
    }

    //  let user change the cell's status in canvas
    pub fn reverse_status(&mut self, row: usize, col: usize) {
        self.display[row][col].reverse_status();
    }

    // pub fn display_canvas(&mut self) {
    //     todo!()
    //     // Ask at office hours
    // }

    // a easy way to display it out in terminal
    pub fn display_in_terminal(&mut self) {
        let space_4 = "    ";
        let space_6 = "      ";
        for row in 0..self.cells_vertical {
            if row % 2 == 0 {
                //  space_3 + display[row][col] + space_5 + ...(display[row][col] + space_5)
                print!("{}", space_4);
                for col in 0..self.cells_horizontal {
                    print!("{}{}", self.display[row][col].get_status_in_string(), space_6);
                }
                print!("\n");
            }
            else {
                //  display[row][col] + space_5 + ...(display[row][col] + space_5)
                for col in 0..self.cells_horizontal {
                    print!("{}{}", self.display[row][col].get_status_in_string(), space_6);
                }
                print!("\n");
            }
        }
    }

    pub fn do_nothing(&mut self) {
        self.display[0][0].get_x();
        self.display[0][0].get_y();
        self.height;
        self.width;
        self.cell_side_length;
    }
}


#[derive(Debug, Default)]
pub struct CanvasSquare {
    row_num: usize,
    col_num: usize,
    cell_side_length: i32,
    display: Vec<Vec<Cell>>
}

impl CanvasSquare {
    pub fn new(row_num_set: usize, col_num_set: usize, side_length: i32) -> CanvasSquare {
        // create 2d vector for cells
        let display_vector: Vec<Vec<Cell>> = vec![vec![Cell::new(); col_num_set]; row_num_set];
        return CanvasSquare {
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
                let next_state = CanvasSquare::check(self, row, col);
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
        let space_2 = "  ";
        for row in 0..self.row_num {
            for col in 0..self.col_num {
                print!("{}{}", self.display[row][col].get_status_in_string(), space_2);
            }
            print!("\n\n");
        }
    }

    pub fn do_nothing(&mut self) {
        self.cell_side_length;
    }
    // helper
    fn alive_nei_num(&mut self, row: usize, col: usize) -> i32 {
        let mut count = 0;
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
        count
    }
    // rules
    fn check(&mut self, row: usize, col: usize) -> bool {
        // todo!();
        //  true    means   alive
        //  false   means   dead
        let neighbor_num = CanvasSquare::alive_nei_num(self, row, col);
        if self.display[row][col].is_alive() {
            // 
            if neighbor_num == 2 {
                return true;
            }
            if neighbor_num == 3 {
                return true;
            }
        }
        else {
            //  reproduction
            if neighbor_num == 3 {
                return true;
            }
        }
        return false;
    }
}


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