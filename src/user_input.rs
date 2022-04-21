use crate::lib::{is_number, read_one};
use crate::terminal_display::{canvas_hexagons_display, canvas_square_display};
use crate::windows_display::{canvas_square_display_windows, canvas_hexagon_display_windows, canvas_hexagon_display_windows_file, canvas_square_display_windows_file};

pub fn hexagons() {
    let mut is_default: String;
    println!("\nHow to set up?\n1) Use default \n2) Use file settings \n3) Customize");
    loop {
        is_default = read_one().trim().to_uppercase();

        if is_default == "1" || is_default == "2" || is_default == "T" || is_default == "3" {
            break
        }
        println!("Invalid enter. Please try again. (1 / 2 / 3)")
    }


    if is_default == "1" {
        canvas_hexagon_display_windows(20, 20, 20);
    }

    if is_default == "2" {
        println!("\nPlease enter the file path: ");
        let file_name = read_one().trim().to_string();
        canvas_hexagon_display_windows_file(&file_name);
    }

    // terminal version
    if is_default == "T" {
        let mut row_num: String;
        println!("Number of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if !row_num.is_empty() && is_number(row_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("\nnumber of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if !col_num.is_empty() && is_number(col_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut loop_num: String;
        loop {
            println!("\nnumber of loops: ");
            loop_num = read_one().trim().to_string();

            if !loop_num.is_empty() && is_number(loop_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let loop_num = loop_num.trim().parse::<usize>().expect("Failed to parse int");
        canvas_hexagons_display(row_num, col_num, loop_num);
    }

    // use user input to create windows
    if is_default == "3" {
        let mut row_num: String;
        println!("\nnumber of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if !row_num.is_empty() && is_number(row_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("\nnumber of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if !col_num.is_empty() && is_number(col_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut cell_size: String;
        println!("\nsize of each cell: ");
        loop {
            cell_size = read_one().trim().to_string();

            if !cell_size.is_empty() && is_number(cell_size.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let cell_size = cell_size.trim().parse::<i32>().expect("Failed to parse int");
        canvas_hexagon_display_windows(row_num, col_num, cell_size);
    }
}

pub fn square() {
    let mut is_default: String;
    println!("\nHow to set up?\n1) Use default \n2) Use file settings \n3) Customize");
    loop {
        is_default = read_one().trim().to_uppercase();

        if is_default == "1" || is_default == "2" || is_default == "T" || is_default == "3" {
            break
        }
        println!("Invalid enter. Please try again. (1 / 2 / 3)")
    }

    if is_default == "1" {
        canvas_square_display_windows(30, 30, 20);
    }

    if is_default == "2" {
        println!("\nPlease enter the file path: ");
        let file_name = read_one().trim().to_string();
        canvas_square_display_windows_file(&file_name);
    }

    // terminal version
    if is_default == "T" {
        let mut row_num: String;
        println!("\nnumber of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if !row_num.is_empty() && is_number(row_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("\nnumber of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if !col_num.is_empty() && is_number(col_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut loop_num: String;
        loop {
            println!("\nnumber of loops: ");
            loop_num = read_one().trim().to_string();

            if !loop_num.is_empty() && is_number(loop_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let loop_num = loop_num.trim().parse::<usize>().expect("Failed to parse int");
        canvas_square_display(row_num, col_num, loop_num);
    }

    // use user input to create windows
    if is_default == "3" {
        let mut row_num: String;
        println!("\nnumber of rows: ");
        loop {
            row_num = read_one().trim().to_string();

            if !row_num.is_empty() && is_number(row_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let row_num = row_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut col_num: String;
        println!("\nnumber of cols: ");
        loop {
            col_num = read_one().trim().to_string();

            if !col_num.is_empty() && is_number(col_num.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let col_num = col_num.trim().parse::<usize>().expect("Failed to parse int");

        let mut cell_size: String;
        println!("\nsize of each cell: ");
        loop {
            cell_size = read_one().trim().to_string();

            if !cell_size.is_empty() && is_number(cell_size.clone()) {
                break
            }

            println!("Invalid enter.  Please try again. (Please enter a number)")
        }
        let cell_size = cell_size.trim().parse::<i32>().expect("Failed to parse int");
        canvas_square_display_windows(row_num, col_num, cell_size);
    }
}