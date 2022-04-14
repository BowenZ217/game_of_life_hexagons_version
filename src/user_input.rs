use crate::lib::{is_number, read_one};
use crate::terminal_display::{canvas_hexagons_display, canvas_square_display};
use crate::windows_display::{canvas_square_display_windows, canvas_hexagon_display_windows};

pub fn hexagons() {
    let mut is_default: String;
    println!("use default? (Y / N)");
    loop {
        is_default = read_one().trim().to_uppercase();

        if is_default == "Y" || is_default == "N" || is_default == "T" {
            break
        }
        println!("Invalid status. Please try again. (Y / N)")
    }


    if is_default == "Y" {
        canvas_hexagon_display_windows(20, 20, 20);
    }

    // terminal version
    else if is_default == "T" {
        let mut row_num: String;
        println!("number of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if is_number(row_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("number of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if is_number(col_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut loop_num: String;
        loop {
            println!("number of loops: ");
            loop_num = read_one().trim().to_string();

            if is_number(loop_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let loop_num = loop_num.trim().parse::<usize>().expect("Failed to parse int");
        canvas_hexagons_display(row_num, col_num, loop_num);
    }

    // use user input to create windows
    else {
        let mut row_num: String;
        println!("number of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if is_number(row_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("number of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if is_number(col_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut cell_size: String;
        println!("size of each cell: ");
        loop {
            cell_size = read_one().trim().to_string();

            if is_number(cell_size.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let cell_size = cell_size.trim().parse::<i32>().expect("Failed to parse int");
        canvas_hexagon_display_windows(row_num, col_num, cell_size);
    }
}

pub fn square() {
    let mut is_default: String;
    println!("use default? (Y / N)");
    loop {
        is_default = read_one().trim().to_uppercase();

        if is_default == "Y" || is_default == "N" || is_default == "T" {
            break
        }
        println!("Invalid status. Please try again. (Y / N)")
    }


    if is_default == "Y" {
        canvas_square_display_windows(30, 30, 20);
    }

    // terminal version
    else if is_default == "T" {
        let mut row_num: String;
        println!("number of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if is_number(row_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("number of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if is_number(col_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut loop_num: String;
        loop {
            println!("number of loops: ");
            loop_num = read_one().trim().to_string();

            if is_number(loop_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let loop_num = loop_num.trim().parse::<usize>().expect("Failed to parse int");
        canvas_square_display(row_num, col_num, loop_num);
    }

    // use user input to create windows
    else {
        let mut row_num: String;
        println!("number of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if is_number(row_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("number of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if is_number(col_num.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut cell_size: String;
        println!("size of each cell: ");
        loop {
            cell_size = read_one().trim().to_string();

            if is_number(cell_size.clone()) {
                break
            }

            println!("Invalid status.  Please try again. (Please enter a number)")
        }
        let cell_size = cell_size.trim().parse::<i32>().expect("Failed to parse int");
        canvas_square_display_windows(row_num, col_num, cell_size);
    }
}