use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_pixel;

const SNAKE_COLOR: Color = [0.0, 0.80, 0.0, 1.0];
const DEFAULT_SNAKE_SIZE: i32 = 2;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn inverse(&self) -> Direction {
		match *self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
		}
	}
}

#[derive(Clone)]
struct Block {
	x: i32,
	y: i32
}

pub struct Snake {
	direction: Direction,
	body: LinkedList<Block>,
	tail: Option<Block>,
}

impl Snake {
	pub fn new(x: i32, y: i32) -> Snake {
		let mut body: LinkedList<Block> = LinkedList::new();
		for i in 0..=DEFAULT_SNAKE_SIZE {
			body.push_back(Block{ x: x + (DEFAULT_SNAKE_SIZE - i), y: y })
		}

		Snake {
			direction: Direction::Right,
			body: body,
			tail: None
		}
	}

	pub fn draw(&self, context: &Context, graphic: &mut G2d) {
		for block in &self.body {
			draw_pixel(SNAKE_COLOR, block.x, block.y, context, graphic)
		}
	}

	pub fn head_position(&self) -> (i32, i32) {
		let head_block = self.body.front().unwrap();
		(head_block.x, head_block.y)
	}

	pub fn move_forword(&mut self, direction: Option<Direction>) {
		match direction {
			Some(d) => self.direction = d,
			None => (),
		}
		
		let (last_x, last_y): (i32, i32) = self.head_position();

		let new_block = match self.direction {
			Direction::Up => Block {
				x: last_x,
				y: last_y - 1,
			},
			Direction::Down => Block {
				x: last_x,
				y: last_y + 1,
			},
			Direction::Left => Block {
				x: last_x - 1,
				y: last_y,
			},
			Direction::Right => Block {
				x: last_x + 1,
				y: last_y,
			}
		};

		self.body.push_front(new_block);
		let removed_block = self.body.pop_back().unwrap();
		self.tail = Some(removed_block);
	}

	pub fn head_direction(&self) -> Direction {
		self.direction
	}

	pub fn next_head(&self, direction: Option<Direction>) -> (i32, i32) {
		
		let (head_x, head_y) = self.head_position();
		let mut moving_direction = self.direction;
		
		match direction {
			Some(dir) => moving_direction = dir,
			None => ()
		};

		match moving_direction {
			Direction::Up => (head_x, head_y - 1),
			Direction::Down => (head_x, head_y + 1),
			Direction::Left => (head_x - 1, head_y),
			Direction::Right => (head_x + 1, head_y),
		}
	}

	pub fn restore_tail(&mut self) {
		let tail = self.tail.clone().unwrap();
		self.body.push_back(tail);
	}

	pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
		let mut current_block = 0;
		for block in &self.body {
			if x == block.x && y == block.y {
				return true;
			}
			current_block += 1;
			if current_block == self.body.len()-1 {
				break;
			}
		}
		false
	}
}