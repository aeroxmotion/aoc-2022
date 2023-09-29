pub mod day1;
pub mod day2;
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
}
