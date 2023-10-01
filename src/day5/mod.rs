use crate::shared::read_input;

// [M] [H]         [N]
// [S] [W]         [F]     [W] [V]
// [J] [J]         [B]     [S] [B] [F]
// [L] [F] [G]     [C]     [L] [N] [N]
// [V] [Z] [D]     [P] [W] [G] [F] [Z]
// [F] [D] [C] [S] [W] [M] [N] [H] [H]
// [N] [N] [R] [B] [Z] [R] [T] [T] [M]
// [R] [P] [W] [N] [M] [P] [R] [Q] [L]
//  1   2   3   4   5   6   7   8   9
pub fn get_initial_stacks() -> [Vec<char>; 9] {
	return [
		vec!['R', 'N', 'F', 'V', 'L', 'J', 'S', 'M'],
		vec!['P', 'N', 'D', 'Z', 'F', 'J', 'W', 'H'],
		vec!['W', 'R', 'C', 'D', 'G'],
		vec!['N', 'B', 'S'],
		vec!['M', 'Z', 'W', 'P', 'C', 'B', 'F', 'N'],
		vec!['P', 'R', 'M', 'W'],
		vec!['R', 'T', 'N', 'G', 'L', 'S', 'W'],
		vec!['Q', 'T', 'H', 'F', 'N', 'B', 'V'],
		vec!['L', 'M', 'H', 'Z', 'N', 'F'],
	];
}

pub fn day5_solution(reverse: bool) -> String {
	let input = read_input(5);
	let moves = input.lines();
	let mut stacks = get_initial_stacks();

	for mov in moves {
		let parts: Vec<&str> = mov.split(" ").collect();
		let num_crates = parts[1].parse::<usize>().unwrap();
		let from_index = parts[3].parse::<usize>().unwrap() - 1;
		let to_index = parts[5].parse::<usize>().unwrap() - 1;

		let mut crates_to_move: Vec<char> = stacks[from_index]
			.drain((stacks[from_index].len() - num_crates)..)
			.collect();

		if reverse {
			crates_to_move.reverse();
		}

		stacks[to_index].append(&mut crates_to_move);
	}

	stacks
		.map(|stack| stack[stack.len() - 1].to_string())
		.join("")
}

pub fn day5a_solution() -> String {
	day5_solution(true)
}

pub fn day5b_solution() -> String {
	day5_solution(false)
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day5a_solution(), "QPJPLMNNR");
		assert_eq!(super::day5b_solution(), "BQDNWJPVJ");
	}
}
