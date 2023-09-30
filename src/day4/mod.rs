use crate::shared::read_input;

pub fn parse_range(range: &str) -> (usize, usize) {
	let parts: Vec<&str> = range.split("-").collect();

	return (
		parts[0].parse::<usize>().unwrap(),
		parts[1].parse::<usize>().unwrap(),
	);
}

pub fn day4a_solution() -> u32 {
	let input = read_input(4);
	let sections = input.lines();

	let mut fully_contains = 0;

	for section in sections {
		let ranges: Vec<&str> = section.split(",").collect();
		let (first_start, first_end) = parse_range(ranges[0]);
		let (second_start, second_end) = parse_range(ranges[1]);

		if (first_start <= second_start && first_end >= second_end)
			|| (second_start <= first_start && second_end >= first_end)
		{
			fully_contains += 1;
		}
	}

	fully_contains
}

pub fn day4b_solution() -> u32 {
	let input = read_input(4);
	let sections = input.lines();

	let mut overlaps = 0;

	for section in sections {
		let ranges: Vec<&str> = section.split(",").collect();
		let (first_start, first_end) = parse_range(ranges[0]);
		let (second_start, second_end) = parse_range(ranges[1]);

		if (second_start >= first_start && second_start <= first_end)
			|| (second_end >= first_start && second_end <= first_end)
			|| (first_start >= second_start && first_start <= second_end)
			|| (first_end >= second_start && first_end <= second_end)
		{
			overlaps += 1;
		}
	}

	overlaps
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day4a_solution(), 511);
		assert_eq!(super::day4b_solution(), 821);
	}
}
