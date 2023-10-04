use crate::shared::read_input;
use std::{cmp, collections::HashSet};

const DELTAS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

#[derive(Clone, Default)]
struct Pos(i32 /* x */, i32 /* y */);

impl Pos {
	fn inc(&mut self, x_delta: i32, y_delta: i32) {
		self.0 += x_delta;
		self.1 += y_delta;
	}

	fn dec(&mut self, x_delta: i32, y_delta: i32) {
		self.0 -= x_delta;
		self.1 -= y_delta;
	}

	fn diff(&self, pos: &Pos) -> (i32, i32, i32) {
		let x_diff = self.0 - pos.0;
		let y_diff = self.1 - pos.1;
		let diff = x_diff.abs() + y_diff.abs();

		return (diff, x_diff, y_diff);
	}
}

#[derive(Clone, Default)]
struct Knot {
	pos: Pos,
	prev: Option<Box<Knot>>,
}

impl Knot {
	fn move_(&mut self, x_delta: i32, y_delta: i32) {
		self.pos.0 += x_delta;
		self.pos.1 += y_delta;

		if self.prev.is_none() {
			return;
		}

		let prev = self.prev.as_mut().unwrap();
		let (diff, x_diff, y_diff) = self.pos.diff(&prev.pos);

		// Same row (move horizontally) or column (move vertically)
		if cmp::min(x_diff.abs(), y_diff.abs()) == 0 && diff == 2 {
			prev.move_(x_diff.clamp(-1, 1), y_diff.clamp(-1, 1));
		}
		// An L
		else if diff > 2 {
			let mut target = prev.pos.clone();

			for (x_delta, y_delta) in DELTAS {
				target.inc(x_delta, y_delta);

				let (inner_diff, _, _) = self.pos.diff(&target);

				if [1, 2].contains(&inner_diff) {
					return prev.move_(x_delta, y_delta);
				}

				target.dec(x_delta, y_delta);
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
						.insert(format!("{},{}", current.pos.0, current.pos.1));
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
