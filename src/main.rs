pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod shared;

fn main() {
	println!(
		"Day 1: {}, {}",
		day1::day1a_solution(),
		day1::day1b_solution()
	);

	println!(
		"Day 2: {}, {}",
		day2::day2a_solution(),
		day2::day2b_solution()
	);

	println!(
		"Day 3: {}, {}",
		day3::day3a_solution(),
		day3::day3b_solution()
	);

	println!(
		"Day 4: {}, {}",
		day4::day4a_solution(),
		day4::day4b_solution()
	);

	println!(
		"Day 5: {}, {}",
		day5::day5a_solution(),
		day5::day5b_solution()
	);

	println!(
		"Day 6: {}, {}",
		day6::day6a_solution(),
		day6::day6b_solution()
	);

	println!(
		"Day 7: {}, {}",
		day7::day7a_solution(),
		day7::day7b_solution()
	);

	println!(
		"Day 8: {}, {}",
		day8::day8a_solution(),
		day8::day8b_solution()
	);

	println!(
		"Day 9: {}, {}",
		day9::day9a_solution(),
		day9::day9b_solution()
	);

	println!(
		"Day 10: {}, \n{}",
		day_10::day10a_solution(),
		day_10::day10b_solution()
	);

	println!(
		"Day 11: {}, {}",
		day_11::day11a_solution(),
		day_11::day11b_solution()
	);

	println!(
		"Day 12: {}, {}",
		day_12::day12a_solution(),
		day_12::day12b_solution()
	);

	println!(
		"Day 13: {}, {}",
		day_13::day13a_solution(),
		day_13::day13b_solution()
	);

	println!("Day 14: {}, {}", 0, 0);
}
