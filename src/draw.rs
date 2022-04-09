use piston_window::rectangle;
use piston_window::polygon;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;


pub fn draw_rectange(color: Color, start_x: f64, start_y: f64, cell_size: f64, con: &Context, g: &mut G2d) {
    rectangle(color, 
        [start_x, 
         start_y, 
         cell_size, 
         cell_size], // rectangle
         con.transform, g);
}

pub fn draw_hexagon(color: Color, center_x: f64, center_y: f64, cell_size: f64, con: &Context, g: &mut G2d) {
    polygon(
        color, 
        &[[center_x - (0.5 * cell_size), center_y + (((3.0 as f64).sqrt() / 2.0) * cell_size)], 
         [center_x + (0.5 * cell_size), center_y + (((3.0 as f64).sqrt() / 2.0) * cell_size)],
         [center_x + cell_size, center_y],
         [center_x + (0.5 * cell_size), center_y - (((3.0 as f64).sqrt() / 2.0) * cell_size)],
         [center_x - (0.5 * cell_size), center_y - (((3.0 as f64).sqrt() / 2.0) * cell_size)],
         [center_x - cell_size, center_y]], 
        con.transform, g);
}