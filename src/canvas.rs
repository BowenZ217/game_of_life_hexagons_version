use crate::cell::Cell;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

use crate::lib::{rem_first_and_last};

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
        
        return CanvasHex {
            height: ((3.0 as f64).sqrt() / 2.0) * (cells_vertical as f64) * side_length + side_length,
            width: (cells_horizontal as f64) * side_length * 3.0 + 0.5 * side_length,
            cells_vertical: cells_vertical,
            cells_horizontal: cells_horizontal,
            cell_side_length: side_length,
            display: display_vector
        };
    }
    pub fn new_f(file_name: &str) -> CanvasHex {
        // uses a reader buffer
        let mut file_name_temp = file_name;
        // check if file name start & end with: ' , such as Mac
        if file_name.chars().nth(0).unwrap() == '\'' {
            file_name_temp = rem_first_and_last(file_name);
        }
        let file = File::open(file_name_temp).expect("file not found!");
        let reader = BufReader::new(file);

        // init
        let mut cells_vertical: usize = 0;
        let mut cells_horizontal: usize = 0;
        let mut side_length: f64 = 0.0;
        let mut display_vector: Vec<Vec<Cell>> = vec![vec![Cell::new(); 0]; 0];
        let mut row: usize = 0;

        // file structure:
        // X means dead cell, O means alive cell
        // col_number row_number cell_size
        // [0][0] [0][1] [0][2]
        // [1][0] [1][1] [1][2]
        // [2][0] [2][1] [2][2]

        for line in reader.lines() {
            let data_line = &line.unwrap();
            let data: Vec<&str> = data_line.split_whitespace().collect();
            // use first line init base case
            if cells_vertical == 0 && cells_horizontal == 0 && side_length == 0.0 {
                cells_horizontal = data[0].trim().parse::<usize>().expect("Failed to parse number col");
                cells_vertical = data[1].trim().parse::<usize>().expect("Failed to parse number row");
                if cells_vertical % 2 != 0 {
                    cells_vertical += 1;
                }
                side_length = data[2].trim().parse::<f64>().expect("Failed to parse number size");
                display_vector = CanvasHex::get_set_position(cells_vertical, cells_horizontal, side_length);
                continue;
                // finished base case
            }
            // after first line: init each cell state
            for col in 0..cells_horizontal {
                if data[col] == "O" {
                    display_vector[row][col].reverse_status();
                }
            }
            row += 1;
        }
        return CanvasHex {
            height: ((3.0 as f64).sqrt() / 2.0) * (cells_vertical as f64) * side_length + side_length,
            width: (cells_horizontal as f64) * side_length * 3.0 + 0.5 * side_length,
            cells_vertical: cells_vertical,
            cells_horizontal: cells_horizontal,
            cell_side_length: side_length,
            display: display_vector
        };
    }
    
    pub fn save_file(&mut self, file_name: &str) {
        let mut data: String = self.cells_horizontal.to_string() + " " + 
                               &self.cells_vertical.to_string() + " " + 
                               &self.cell_side_length.to_string() + "\n";
        for row in 0..self.cells_vertical {
            for col in 0..self.cells_horizontal {
                if self.display[row][col].is_alive() {
                    data += "O ";
                }
                else {
                    data += "X ";
                }
            }
            data += "\n";
        }
        let mut f = File::create(file_name).expect("Unable to create file");
        f.write_all(data.as_bytes()).expect("Unable to write data");
    }

    fn get_set_position(cells_vertical: usize, cells_horizontal: usize, cell_side_length: f64) -> Vec<Vec<Cell>> {
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
                // sqrt(3)/2 approx = 0.8660254

                // Position of even and odd row is diifferent
                // odd rows (start from 0)
                if row % 2 == 0 {
                    let x = (2.5 + 3.0 * (col as f64)) * cell_side_length;
                    let y = (((3.0 as f64).sqrt() / 2.0) * (1.0 + row as f64)) * cell_side_length;
                    display_vector[row][col].set_x(x);
                    display_vector[row][col].set_y(y);
                }
                // even rows
                else {
                    let x = (1.0 + (3.0 * (col as f64))) * cell_side_length;
                    let y = (row as f64 + 1.0) * ((3.0 as f64).sqrt() / 2.0) * cell_side_length;
                    display_vector[row][col].set_x(x);
                    display_vector[row][col].set_y(y);
                }
            }
        }
        return display_vector;
    }

    // means cells_vertical % 2 == 0
    // get number of alive neighbors of that cell
    fn alive_nei_num(&mut self, row: usize, col: usize) -> i32 {
        let mut count = 0;
        // odd rows
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
        // odd rows
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
        //  true    means   alive
        //  false   means   dead
        // check how many alive neighbor cells
        let neighbor_num = CanvasHex::alive_nei_num(self, row, col);
        if self.display[row][col].is_alive() {
            // 
            if neighbor_num == 3 || neighbor_num == 4{
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
        let mut next = self.display.clone();
        for row in 0..self.cells_vertical {
            for col in 0..self.cells_horizontal {
                let next_state = CanvasHex::check(self, row, col);
                next[row][col].change_status(next_state);
            }
        }
        self.display = next;
    }

    // use user mouse position (when clicked) to check state
    // version 2:
    //  not work when the mouse position is at triangle part for each hexagon ———— slanting lines
    pub fn change_state(&mut self, x: f64, y: f64) {
        let temp = (x - (self.cell_side_length / 2.0)) / (self.cell_side_length / 2.0);
        // devided to six parts
        //  part:   x 0 1 2 3 4 5
        //                 _____
        //           _____/     \
        //          /     \_____/
        //          \_____/
        // 0 && 1 means odd rows
        // 3 && 4 means even rows
        let region = temp as usize % 6;
        let col = temp as usize / 6;
        if region == 0 || region == 1 {
            // start from 0 + (√3 / 2) * cell_side_length 
            let mut row = ( (y - (((3.0 as f64).sqrt() * self.cell_side_length) / 2.0) ) * 2.0 / ((3.0 as f64).sqrt() * self.cell_side_length) ) as usize;
            // make sure row number is odd
            if row % 2 == 0 {
                row += 1;
            }
            self.display[row][col].reverse_status();
        }
        if region == 3 || region == 4 {
            // start from 0
            let mut row = ( y  / ((3.0 as f64).sqrt() * self.cell_side_length)  * 2.0) as usize;
            // make sure row number is even
            if row % 2 != 0 {
                row -= 1;
            }
            self.display[row][col].reverse_status();
        }
    }

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
    // geter
    pub fn get_height(&mut self) -> f64 {
        return self.height;
    }
    pub fn get_width(&mut self) -> f64 {
        return self.width;
    }
    pub fn get_cell_center_x(&mut self, row: usize, col: usize) -> f64 {
        return self.display[row][col].get_x();
    }
    pub fn get_cell_center_y(&mut self, row: usize, col: usize) -> f64 {
        return self.display[row][col].get_y();
    }
    pub fn is_alive(&mut self, row: usize, col: usize) -> bool {
        return self.display[row][col].is_alive();
    }
    pub fn get_row_num(&mut self) -> usize {
        self.cells_vertical
    }
    pub fn get_col_num(&mut self) -> usize {
        self.cells_horizontal
    }
    pub fn get_cell_size(&mut self) -> f64 {
        self.cell_side_length
    }

    // seter
    //  let user change the cell's status in canvas
    pub fn reverse_status(&mut self, row: usize, col: usize) {
        self.display[row][col].reverse_status();
    }
    pub fn set_state(&mut self, row: usize, col: usize, next: bool) {
        self.display[row][col].change_status(next);
    }
}


#[derive(Debug, Default)]
pub struct CanvasSquare {
    row_num: usize,
    col_num: usize,
    cell_side_length: f64,
    display: Vec<Vec<Cell>>
}

impl CanvasSquare {
    pub fn new(row_num_set: usize, col_num_set: usize, side_length: f64) -> CanvasSquare {
        // create 2d vector for cells
        let display_vector: Vec<Vec<Cell>> = vec![vec![Cell::new(); col_num_set]; row_num_set];
        return CanvasSquare {
            row_num: row_num_set,
            col_num: col_num_set,
            cell_side_length: side_length,
            display: display_vector
        };
    }
    
    pub fn new_f(file_name: &str) -> CanvasSquare {
        // uses a reader buffer
        let mut file_name_temp = file_name;
        // check if file name start & end with: ' , such as Mac
        if file_name.chars().nth(0).unwrap() == '\'' {
            file_name_temp = rem_first_and_last(file_name);
        }
        let file = File::open(file_name_temp).expect("file not found!");
        let reader = BufReader::new(file);

        // init
        let mut col_num_set: usize = 0;
        let mut row_num_set: usize = 0;
        let mut side_length: f64 = 0.0;
        let mut display_vector: Vec<Vec<Cell>> = vec![vec![Cell::new(); 0]; 0];
        let mut row: usize = 0;

        // file structure:
        // X means dead cell, O means alive cell
        // col_number row_number cell_size
        // [0][0] [0][1] [0][2]
        // [1][0] [1][1] [1][2]
        // [2][0] [2][1] [2][2]

        for line in reader.lines() {
            let data_line = &line.unwrap();
            let data: Vec<&str> = data_line.split_whitespace().collect();
            // use first line init base case
            if row_num_set == 0 && col_num_set == 0 && side_length == 0.0 {
                col_num_set = data[0].trim().parse::<usize>().expect("Failed to parse number col");
                row_num_set = data[1].trim().parse::<usize>().expect("Failed to parse number row");
                side_length = data[2].trim().parse::<f64>().expect("Failed to parse number size");
                display_vector = vec![vec![Cell::new(); col_num_set]; row_num_set];
                continue;
                // finish base case
            }
            // after first line: init cell state
            for col in 0..col_num_set {
                if data[col] == "O" {
                    display_vector[row][col].reverse_status();
                }
            }
            row += 1;
        }
        return CanvasSquare {
            row_num: row_num_set,
            col_num: col_num_set,
            cell_side_length: side_length,
            display: display_vector
        };
    }

    pub fn save_file(&mut self, file_name: &str) {
        let mut data: String = self.col_num.to_string() + " " + 
                               &self.row_num.to_string() + " " + 
                               &self.cell_side_length.to_string() + "\n";
        for row in 0..self.row_num {
            for col in 0..self.col_num {
                if self.display[row][col].is_alive() {
                    data += "O ";
                }
                else {
                    data += "X ";
                }
            }
            data += "\n";
        }
        let mut f = File::create(file_name).expect("Unable to create file");
        f.write_all(data.as_bytes()).expect("Unable to write data");
    }

    pub fn next_generation(&mut self) {
        let mut next = self.display.clone();
        for row in 0..self.row_num {
            for col in 0..self.col_num {
                let next_state = CanvasSquare::check(self, row, col);
                next[row][col].change_status(next_state);
            }
        }
        self.display = next;
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
        //  true    means   alive
        //  false   means   dead
        let neighbor_num = CanvasSquare::alive_nei_num(self, row, col);
        if self.display[row][col].is_alive() {
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
    // seter
    //  let user change the cell's status in canvas
    pub fn reverse_status(&mut self, row: usize, col: usize) {
        self.display[row][col].reverse_status();
    }
    pub fn set_state(&mut self, row: usize, col: usize, next: bool) {
        self.display[row][col].change_status(next);
    }
    pub fn change_state(&mut self, x: f64, y: f64) {
        self.display[(y / self.cell_side_length) as usize][(x / self.cell_side_length) as usize].reverse_status();
    }

    // geter
    pub fn get_canvas(&mut self) -> Vec<Vec<Cell>> {
        return self.display.clone(); 
    }
    pub fn get_row_num(&mut self) -> usize {
        self.row_num
    }
    pub fn get_col_num(&mut self) -> usize {
        self.col_num
    }
    pub fn get_cell_size(&mut self) -> f64 {
        self.cell_side_length
    }
}


