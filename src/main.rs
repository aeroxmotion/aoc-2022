pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
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
}
