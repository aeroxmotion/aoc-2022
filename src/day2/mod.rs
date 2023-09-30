use crate::shared::read_input;

// Opponent moves:
// A = Rock
// B = Paper
// C = Scissors

// Own moves:
// X = Rock
// Y = Paper
// Z = Scissors

const ALPHA_OWN_ASCII_OFFSET: u8 = 87;
const ALPHA_OPPONENT_ASCII_OFFSET: u8 = 64;

fn get_opponent_move_score(char: u8) -> u32 {
	(char - ALPHA_OPPONENT_ASCII_OFFSET) as u32
}

pub fn day2a_solution() -> u32 {
	let input = read_input(2);
	let rounds = input.split("\n");
	let mut own_score: u32 = 0;

	for round in rounds {
		let moves: Vec<&str> = round.split(" ").collect();

		let opponent_move = moves[0].as_bytes()[0];
		let own_move = moves[1].as_bytes()[0];

		// By using ASCII diff
		let diff = own_move - opponent_move;

		own_score += match diff {
			// Draw cases (diff always =23):
			// A X
			// B Y
			// C Z
			23 => 3,

			// Win cases:
			// A Y (diff 24)
			// B Z (diff 24)
			// C X (diff 21)
			24 | 21 => 6,

			// Loss cases:
			// A Z (diff 25)
			// B X (diff 22)
			// C Y (diff 22)
			_ => 0,
		};

		own_score += (own_move - ALPHA_OWN_ASCII_OFFSET) as u32;
	}

	own_score
}

// So, now:
// X = Loss
// Y = Draw
// Z = Win

pub fn day2b_solution() -> u32 {
	let input = read_input(2);
	let rounds = input.split("\n");
	let mut own_score: u32 = 0;

	for round in rounds {
		let moves: Vec<&str> = round.split(" ").collect();

		let opponent_move = moves[0].as_bytes()[0];
		let own_move = moves[1].as_bytes()[0];

		own_score += match own_move as char {
			// Lose
			'X' => {
				let result = get_opponent_move_score(opponent_move) as i32 - 1;

				if result <= 0 {
					3
				} else {
					result as u32
				}
			}

			// Draw
			'Y' => get_opponent_move_score(opponent_move) + 3,

			// Win
			_ => (get_opponent_move_score(opponent_move) % 3) + 7,
		};
	}

	own_score
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day2a_solution(), 13_005);
		assert_eq!(super::day2b_solution(), 11_373);
	}
}
