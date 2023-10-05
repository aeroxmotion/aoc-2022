use crate::shared::read_input;

struct Executor {
	reg_x: i32,
	in_addx: bool,
	addx_value: i32,
	current_cycle: usize,
	instructions: Vec<String>,
}

impl Executor {
	fn new() -> Executor {
		let input = read_input(10);
		let mut instructions: Vec<String> = vec![];

		for line in input.lines() {
			for ins in line.split(" ") {
				instructions.push(ins.to_string());
			}
		}

		Executor {
			reg_x: 1,
			instructions,
			addx_value: 0,
			in_addx: false,
			current_cycle: 1,
		}
	}
}

impl Iterator for Executor {
	type Item = (i32, usize);

	fn next(&mut self) -> Option<Self::Item> {
		let cycle = self.current_cycle;

		if cycle > self.instructions.len() {
			return None;
		}

		let ins = &self.instructions[cycle - 1];

		if self.in_addx {
			self.addx_value = ins.parse::<i32>().unwrap();
		} else if self.addx_value != 0 {
			self.reg_x += self.addx_value;
			self.addx_value = 0;
		}

		self.in_addx = ins == "addx";
		self.current_cycle += 1;

		return Some((self.reg_x, cycle));
	}
}

pub fn day10a_solution() -> i32 {
	let executor = Executor::new();

	let mut sum = 0;
	let cycles = [20, 60, 100, 140, 180, 220];

	for (reg_x, cycle) in executor {
		if cycles.contains(&cycle) {
			sum += reg_x * cycle as i32;
		}
	}

	sum
}

pub fn day10b_solution() -> String {
	let executor = Executor::new();
	let (crt_wide, crt_high) = (40, 6);

	let mut rows: Vec<String> = vec!["".into(); crt_high];

	for (reg_x, cycle) in executor {
		let row_index = ((cycle - 1) as f32 / crt_wide as f32).floor() as usize;

		if row_index == crt_high {
			break;
		}

		let row = &mut rows[row_index];
		let crt_index = (cycle - 1) % 40;
		let mut pixel = '.';

		if (crt_index as i32).clamp(reg_x - 1, reg_x + 1) == crt_index as i32 {
			pixel = '#';
		}

		row.push(pixel);
	}

	rows.join("\n")
}
