use crate::shared::read_input;
use std::collections::HashSet;

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

		if self.prev.is_none() {
			return;
		}

		let prev = self.prev.as_mut().unwrap();

		// Same row
		if self.x == prev.x {
			let diff = self.y - prev.y;

			if diff.abs() == 2 {
				prev.move_(0, diff.clamp(-1, 1));
			}

			return;
		} else if self.y == prev.y {
			let diff = self.x - prev.x;

			if diff.abs() == 2 {
				prev.move_(diff.clamp(-1, 1), 0);
			}

			return;
		}

		let x_diff = prev.x - self.x;
		let y_diff = prev.y - self.y;

		// An L or diagonally
		if x_diff.abs() + y_diff.abs() > 2 {
			let deltas = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

			for (x_delta, y_delta) in deltas {
				let mut target = prev.clone();
				target.move_(x_delta, y_delta);

				let x_inner_diff = self.x - target.x;
				let y_inner_diff = self.y - target.y;

				if x_inner_diff.abs() + y_inner_diff.abs() == 1
					|| x_inner_diff.abs() + y_inner_diff.abs() == 2
				{
					prev.move_(x_delta, y_delta);
					break;
				}
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
