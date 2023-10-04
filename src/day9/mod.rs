use crate::shared::read_input;
use std::{cmp, collections::HashSet};

#[derive(Clone, Default)]
struct Knot {
	x: i32,
	y: i32,
	prev: Option<Box<Knot>>,
}

impl Knot {
	fn move_(&mut self, x_delta: i32, y_delta: i32) {
		self.x += x_delta;
		self.y += y_delta;

		if let Some(prev) = self.prev.as_mut() {
			let x_diff = self.x - prev.x;
			let y_diff = self.y - prev.y;
			let diff = x_diff.abs() + y_diff.abs();

			if
			// Same row or column
			(cmp::min(x_diff.abs(), y_diff.abs()) == 0 && diff == 2) ||
				// An L or diagonally
				[3, 4].contains(&diff)
			{
				prev.move_(x_diff.clamp(-1, 1), y_diff.clamp(-1, 1));
			}
		}
	}
}

#[derive(Default)]
struct Rope {
	head: Knot,
	tail_visited: HashSet<String>,
}

impl Rope {
	fn new(knots: usize) -> Rope {
		let mut rope = Self::default();
		let mut current = &mut rope.head;

		for _ in 0..(knots - 1) {
			current.prev = Some(Box::new(Knot::default()));
			current = current.prev.as_mut().unwrap();
		}

		rope.register_tail_visit();
		rope
	}

	fn move_(&mut self, dir: char, steps: i32) {
		for _ in 0..steps {
			match dir {
				'U' => self.head.move_(0, -1),
				'L' => self.head.move_(-1, 0),
				'R' => self.head.move_(1, 0),
				'D' => self.head.move_(0, 1),
				_ => panic!("Unexpected direction..."),
			}

			self.register_tail_visit();
		}
	}

	fn register_tail_visit(&mut self) {
		let mut current = &self.head;

		loop {
			match current.prev {
				Some(_) => {
					current = current.prev.as_ref().unwrap();
				}
				None => {
					self.tail_visited
						.insert(format!("{},{}", current.x, current.y));
					break;
				}
			}
		}
	}
}

pub fn day9_solution(knots: usize) -> usize {
	let input = read_input(9);
	let motions = input.lines();
	let mut rope = Rope::new(knots);

	for motion in motions {
		let parts: Vec<&str> = motion.split(" ").collect();

		rope.move_(
			parts[0].chars().collect::<Vec<char>>()[0],
			parts[1].parse::<i32>().unwrap(),
		);
	}

	rope.tail_visited.len()
}

pub fn day9a_solution() -> usize {
	day9_solution(2)
}

pub fn day9b_solution() -> usize {
	day9_solution(10)
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day9a_solution(), 6_023);
		assert_eq!(super::day9b_solution(), 2_533);
	}
}
