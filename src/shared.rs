use std::fs;

pub fn read_input(day: u32) -> String {
	let path = format!("src/day{}/input.txt", day);
	let input = fs::read_to_string(path).expect("Should read input...");

	input.trim().into()
}
