use piston_window::{ rectangle, Context, G2d };
use piston_window::types::Color;

const PIXEL_SIZE: f64 = 10.0;

pub fn to_coord(game_coord: i32) -> f64 {
	(game_coord as f64) * PIXEL_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
	to_coord(game_coord) as u32
}

pub fn draw_pixel(color: Color, x: i32, y: i32, context: &Context, graphic: &mut G2d) {
	let _x = to_coord(x);
	let _y = to_coord(y);

	rectangle(
		color,
		[ _x, _y, PIXEL_SIZE, PIXEL_SIZE ],
		context.transform,
		graphic
	);
}

pub fn draw_rect(color: Color, x: i32, y: i32, width: i32, height: i32,context: &Context, graphic: &mut G2d) {
	let _x = to_coord(x);
	let _y = to_coord(y);

	rectangle(
		color,
		[ _x, _y, PIXEL_SIZE * (width as f64), PIXEL_SIZE * (height as f64) ],
		context.transform,
		graphic
	);
}