use std::cmp;

use crate::shared::read_input;

pub fn day1a_solution() -> i32 {
	let input = read_input(1);

	let elfs = input.trim().split("\n\n");
	let mut max_calories = 0;

	for elf in elfs {
		let calories = elf.split("\n");
		let mut elf_calories = 0;

		for cal in calories {
			let n = cal.parse::<i32>().expect("Should parse int...");
			elf_calories += n;
		}

		max_calories = cmp::max(max_calories, elf_calories);
	}

	max_calories
}

pub fn day1b_solution() -> i32 {
	let input = read_input(1);

	let elfs = input.trim().split("\n\n");
	let mut all_calories = Vec::new();
	let mut total_calories = 0;

	for elf in elfs {
		let calories = elf.split("\n");
		let mut elf_calories = 0;

		for cal in calories {
			let n = cal.parse::<i32>().expect("Should parse int...");
			elf_calories += n;
		}

		all_calories.push(elf_calories);
	}

	all_calories.sort_unstable();

	for n in 1..4 {
		total_calories += all_calories[all_calories.len() - n];
	}

	total_calories
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day1a_solution(), 70_116);
		assert_eq!(super::day1b_solution(), 206_582);
	}
}
