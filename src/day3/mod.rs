use crate::shared::read_input;
use std::collections::{HashMap, HashSet};

fn get_priority(char: u8) -> u32 {
	match char as char {
		'a'..='z' => char as u32 - 96,
		_ => char as u32 - 38,
	}
}

pub fn day3a_solution() -> u32 {
	let input = read_input(3);
	let rucksacks = input.lines();

	let mut priorities_sum = 0;

	for rucksack in rucksacks {
		let middle = rucksack.len() / 2;
		let mut counts = HashSet::new();

		for (i, char) in rucksack.bytes().enumerate() {
			if i < middle {
				counts.insert(char);
			} else if counts.contains(&char) {
				priorities_sum += get_priority(char);
				break;
			}
		}
	}

	priorities_sum
}

pub fn day3b_solution() -> u32 {
	let input = read_input(3);
	let rucksacks = input.lines();

	let mut priorities_sum = 0;
	let mut found_badges = HashMap::<u8, usize>::new();

	for (i, rucksack) in rucksacks.enumerate() {
		// Our increment
		let n = 1 << (i % 3);

		for char in rucksack.bytes() {
			let badge_count = found_badges.get(&char).unwrap_or_else(|| &0);
			let next_badge_count = badge_count | n;

			if next_badge_count == 7 {
				priorities_sum += get_priority(char);
				found_badges.clear();
				break;
			}

			found_badges.insert(char, next_badge_count);
		}
	}

	priorities_sum
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day3a_solution(), 7_727);
		assert_eq!(super::day3b_solution(), 2_609);
	}
}
