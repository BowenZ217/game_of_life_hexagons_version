use crate::canvas::CanvasHex;
use crate::canvas::CanvasSquare;
use piston_window::*;
use piston_window::types::Color;

use crate::draw::{draw_rectange, draw_hexagon};

extern crate rand;
extern crate piston_window;

// dead cell color
const BLACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
// background color
const GRAY_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
// alive cell color
const WHITE_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

// auto speed setup
const SPEED_INIT: i32 = 300;
const SPEED_MAX: i32 = 600;
const SPEED_CHANGE_PER_TIME: i32 = 20;

pub fn canvas_square_display_windows(row_num: usize, col_num: usize, cell_size: i32) {
    // Prepare window settings
    let mut window_settings = WindowSettings::new("Rust Game of Life - square",
        [(((col_num as i32) * cell_size + 10) as u32), (((row_num as i32) * cell_size + 10) as u32)])
        .exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // Create a canvas
    let mut canvas_squares = CanvasSquare::new(row_num, col_num, cell_size as f64);

    // init
    let mut auto = false;
    let mut cursor = [0.0, 0.0];
    let mut time = 0;
    let mut speed = SPEED_INIT;

    // Event loop
    while let Some(event) = window.next() {
        time += 1;

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // check for auto play
            if key == Key::Space {
                if auto {
                    auto = false;
                } else {
                    auto = true;
                }
            }
            // if enter -> next loop
            if key == Key::Return {
                canvas_squares.next_generation();
            }
            // increase the speed of auto play
            if key == Key::Up {
                if speed >= 0 {
                    speed -= SPEED_CHANGE_PER_TIME;
                }
            }
            // decrease the speed of auto play
            if key == Key::Down {
                if speed < SPEED_MAX {
                    speed += SPEED_CHANGE_PER_TIME;
                }
            }
            // create some random alive cells
            if key == Key::R {
                for row in 0..row_num {
                    for col in 0..col_num {
                        canvas_squares.set_state(row, col, rand::random::<bool>());
                    }
                }
            }
            // save current
            if key == Key::S {
                canvas_squares.save_file("save.txt");
            }
        };
        // check mouse click
        let mut change = false;
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                change = true;
            }
        }
        // get mouse position
        event.mouse_cursor(|pos| {
            cursor = pos;
        });
        // check if user have a mouse click -> change current cell state
        if change {
            canvas_squares.change_state(cursor[0], cursor[1]);
        }

        let mut canvas = canvas_squares.get_canvas();
        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(GRAY_COLOR, g);
            for row in 0..row_num {
                for col in 0..col_num {
                    if canvas[row][col].is_alive() {
                        draw_rectange(WHITE_COLOR, 
                            (col as i32 * cell_size) as f64, // start_x
                            (row as i32 * cell_size) as f64, // start_y
                            cell_size as f64 - 1.0, 
                            &c, 
                            g);
                    }
                    else {
                        draw_rectange(BLACK_COLOR, 
                            (col as i32 * cell_size) as f64, // start_x
                            (row as i32 * cell_size) as f64, // start_y
                            cell_size as f64 - 1.0, 
                            &c, 
                            g);
                    }
                }
            }
        });
        if auto && time > speed {
            canvas_squares.next_generation();
            time = 0;
        }
    }
}

pub fn canvas_hexagon_display_windows(row_num: usize, col_num: usize, cell_size: i32) {
    // Create a canvas
    let mut canvas_hexagons = CanvasHex::new(row_num, col_num, cell_size as f64);

    // Prepare window settings
    let mut window_settings = WindowSettings::new("Rust Game of Life - hexagon",
    [(canvas_hexagons.get_width() as u32), 
     (canvas_hexagons.get_height() as u32)]).exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // init
    let mut auto = false;
    let mut cursor = [0.0, 0.0];
    let mut time = 0;
    let mut speed = SPEED_INIT;

    // Event loop
    while let Some(event) = window.next() {
        time += 1;

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // check for auto play
            if key == Key::Space {
                if auto {
                    auto = false;
                } else {
                    auto = true;
                }
            }
            // if enter -> next loop
            if key == Key::Return {
                canvas_hexagons.next_generation();
            }
            // increase the speed of auto play
            if key == Key::Up {
                if speed >= 0 {
                    speed -= SPEED_CHANGE_PER_TIME;
                }
            }
            // decrease the speed of auto play
            if key == Key::Down {
                if speed < SPEED_MAX {
                    speed += SPEED_CHANGE_PER_TIME;
                }
            }
            // create some random alive cells
            if key == Key::R {
                for row in 0..2*row_num {
                    for col in 0..col_num {
                        canvas_hexagons.set_state(row, col, rand::random::<bool>());
                    }
                }
            }
            // save current
            if key == Key::S {
                canvas_hexagons.save_file("save.txt");
            }
        };
        // check mouse click
        let mut change = false;
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                change = true;
            }
        }
        // get mouse position
        event.mouse_cursor(|pos| {
            cursor = pos;
        });
        // check if user have a mouse click -> change current cell state
        if change {
            canvas_hexagons.change_state(cursor[0], cursor[1]);
        }

        // let mut canvas = canvas_hexagons.get_canvas();
        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(GRAY_COLOR, g);
            for row in 0..2*row_num {
                for col in 0..col_num {
                    if canvas_hexagons.is_alive(row, col) {
                        // for testing the center
                        // draw_rectange(WHITE_COLOR, 
                        //     canvas_hexagons.get_cell_center_x(row, col), // start_x
                        //     canvas_hexagons.get_cell_center_y(row, col), // start_y
                        //     (cell_size * 2 / 3) as f64, 
                        //     &c, 
                        //     g);
                        draw_hexagon(
                            WHITE_COLOR,
                            canvas_hexagons.get_cell_center_x(row, col),
                            canvas_hexagons.get_cell_center_y(row, col),
                            cell_size as f64 - 0.5, &c, g);
                    }
                    else {
                        draw_hexagon(
                            BLACK_COLOR,
                            canvas_hexagons.get_cell_center_x(row, col),
                            canvas_hexagons.get_cell_center_y(row, col),
                            cell_size as f64 - 0.5, &c, g);
                    }
                }
            }
            
        });
        if auto && time > speed {
            canvas_hexagons.next_generation();
            time = 0;
        }
    }
}
pub fn canvas_hexagon_display_windows_file(file_name: &str) {
    // Create a canvas
    let mut canvas_hexagons = CanvasHex::new_f(file_name);

    // Prepare window settings
    let mut window_settings = WindowSettings::new("Rust Game of Life - hexagon",
    [(canvas_hexagons.get_width() as u32), 
     (canvas_hexagons.get_height() as u32)]).exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // init
    let mut auto = false;
    let mut cursor = [0.0, 0.0];
    let mut time = 0;
    let mut speed = SPEED_INIT;

    // Event loop
    while let Some(event) = window.next() {
        time += 1;

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // check for auto play
            if key == Key::Space {
                if auto {
                    auto = false;
                } else {
                    auto = true;
                }
            }
            // if enter -> next loop
            if key == Key::Return {
                canvas_hexagons.next_generation();
            }
            // increase the speed of auto play
            if key == Key::Up {
                if speed >= 0 {
                    speed -= SPEED_CHANGE_PER_TIME;
                }
            }
            // decrease the speed of auto play
            if key == Key::Down {
                if speed < SPEED_MAX {
                    speed += SPEED_CHANGE_PER_TIME;
                }
            }
            // create some random alive cells
            if key == Key::R {
                for row in 0..canvas_hexagons.get_row_num() {
                    for col in 0..canvas_hexagons.get_col_num() {
                        canvas_hexagons.set_state(row, col, rand::random::<bool>());
                    }
                }
            }
            // save current
            if key == Key::S {
                canvas_hexagons.save_file("save.txt");
            }
        };
        // check mouse click
        let mut change = false;
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                change = true;
            }
        }
        // get mouse position
        event.mouse_cursor(|pos| {
            cursor = pos;
        });
        // check if user have a mouse click -> change current cell state
        if change {
            canvas_hexagons.change_state(cursor[0], cursor[1]);
        }

        // let mut canvas = canvas_hexagons.get_canvas();
        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(GRAY_COLOR, g);
            for row in 0..canvas_hexagons.get_row_num() {
                for col in 0..canvas_hexagons.get_col_num() {
                    if canvas_hexagons.is_alive(row, col) {
                        // for testing the center
                        // draw_rectange(WHITE_COLOR, 
                        //     canvas_hexagons.get_cell_center_x(row, col), // start_x
                        //     canvas_hexagons.get_cell_center_y(row, col), // start_y
                        //     (cell_size * 2 / 3) as f64, 
                        //     &c, 
                        //     g);
                        draw_hexagon(
                            WHITE_COLOR,
                            canvas_hexagons.get_cell_center_x(row, col),
                            canvas_hexagons.get_cell_center_y(row, col),
                            canvas_hexagons.get_cell_size() - 0.5, &c, g);
                    }
                    else {
                        draw_hexagon(
                            BLACK_COLOR,
                            canvas_hexagons.get_cell_center_x(row, col),
                            canvas_hexagons.get_cell_center_y(row, col),
                            canvas_hexagons.get_cell_size() - 0.5, &c, g);
                    }
                }
            }
            
        });
        if auto && time > speed {
            canvas_hexagons.next_generation();
            time = 0;
        }
    }
}

pub fn canvas_square_display_windows_file(file_name: &str) {
    // Create a canvas
    let mut canvas_squares = CanvasSquare::new_f(&file_name);

    // Prepare window settings
    let mut window_settings = WindowSettings::new("Rust Game of Life - square",
        [(((canvas_squares.get_col_num() as f64) * canvas_squares.get_cell_size() + 10.0) as u32), 
        (((canvas_squares.get_row_num() as f64) * canvas_squares.get_cell_size() + 10.0) as u32)])
        .exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // init
    let mut auto = false;
    let mut cursor = [0.0, 0.0];
    let mut time = 0;
    let mut speed = SPEED_INIT;

    // Event loop
    while let Some(event) = window.next() {
        time += 1;

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // check for auto play
            if key == Key::Space {
                if auto {
                    auto = false;
                } else {
                    auto = true;
                }
            }
            // if enter -> next loop
            if key == Key::Return {
                canvas_squares.next_generation();
            }
            // increase the speed of auto play
            if key == Key::Up {
                if speed >= 0 {
                    speed -= SPEED_CHANGE_PER_TIME;
                }
            }
            // decrease the speed of auto play
            if key == Key::Down {
                if speed < SPEED_MAX {
                    speed += SPEED_CHANGE_PER_TIME;
                }
            }
            // create some random alive cells
            if key == Key::R {
                for row in 0..canvas_squares.get_row_num() {
                    for col in 0..canvas_squares.get_col_num() {
                        canvas_squares.set_state(row, col, rand::random::<bool>());
                    }
                }
            }
            // save current
            if key == Key::S {
                canvas_squares.save_file("save.txt");
            }
        };
        // check mouse click
        let mut change = false;
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                change = true;
            }
        }
        // get mouse position
        event.mouse_cursor(|pos| {
            cursor = pos;
        });
        // check if user have a mouse click -> change current cell state
        if change {
            canvas_squares.change_state(cursor[0], cursor[1]);
        }

        let mut canvas = canvas_squares.get_canvas();
        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(GRAY_COLOR, g);
            for row in 0..canvas_squares.get_row_num() {
                for col in 0..canvas_squares.get_col_num() {
                    if canvas[row][col].is_alive() {
                        draw_rectange(WHITE_COLOR, 
                            col as f64 * canvas_squares.get_cell_size(), // start_x
                            row as f64 * canvas_squares.get_cell_size(), // start_y
                            canvas_squares.get_cell_size() - 1.0, 
                            &c, 
                            g);
                    }
                    else {
                        draw_rectange(BLACK_COLOR, 
                            col as f64 * canvas_squares.get_cell_size(), // start_x
                            row as f64 * canvas_squares.get_cell_size(), // start_y
                            canvas_squares.get_cell_size() - 1.0, 
                            &c, 
                            g);
                    }
                }
            }
        });
        if auto && time > speed {
            canvas_squares.next_generation();
            time = 0;
        }
    }
}