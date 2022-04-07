use piston_window::rectangle;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

const BLOCK_SIZE: f64 = 10.0;
pub fn draw_rectange(color: Color, start_x: i32, start_y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    rectangle(color, [(start_x as f64), (start_y as f64),
            BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)], con.transform, g);
}
