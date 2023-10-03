use std::collections::HashSet;

use crate::shared::read_input;

#[derive(Clone, Default)]
struct Knot {
	x: i32,
	y: i32,
}

#[derive(Default)]
struct Rope {
	head: Knot,
	tail: Knot,
	tail_visited: HashSet<String>,
}

impl Rope {
	fn new() -> Rope {
		let mut rope = Self::default();

		rope.tail_visited.insert("0,0".into());

		rope
	}

	fn move_(&mut self, dir: char, steps: i32) {
		let mut prev_head = self.head.clone();

		for _ in 0..steps {
			match dir {
				'U' => self.head.y -= 1,
				'L' => self.head.x -= 1,
				'R' => self.head.x += 1,
				'D' => self.head.y += 1,
				_ => panic!("Unexpected direction..."),
			}

			if self.is_tail_away() {
				self.tail = prev_head.clone();
				self.tail_visited
					.insert(format!("{},{}", self.tail.x, self.tail.y));
			}

			prev_head = self.head.clone();
		}
	}

	fn is_tail_away(&self) -> bool {
		let x_diff = (self.head.x - self.tail.x).abs();
		let y_diff = (self.head.y - self.tail.y).abs();

		x_diff > 1 // Same row
			|| y_diff > 1 // Same column
			|| x_diff + y_diff > 2 // Different column and row
	}
}

pub fn day9a_solution() -> usize {
	let input = read_input(9);
	let motions = input.lines();
	let mut rope = Rope::new();

	for motion in motions {
		let parts: Vec<&str> = motion.split(" ").collect();

		rope.move_(
			parts[0].chars().collect::<Vec<char>>()[0],
			parts[1].parse::<i32>().unwrap(),
		);
	}

	rope.tail_visited.len()
}

pub fn day9b_solution() -> usize {
	0
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day9a_solution(), 6_023);
	}
}
