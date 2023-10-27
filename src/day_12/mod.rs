use std::collections::HashSet;

use crate::shared::read_input;

type Pos = (usize, usize);
type Grid = Vec<Vec<char>>;

fn get_height(char: char) -> i32 {
	(match char {
		'S' => 'a',
		'E' => 'z',
		c => c,
	}) as i32
}

fn get_adjacents(grid: &Grid, (x, y): Pos, visited: &HashSet<Pos>) -> Vec<Pos> {
	let mut adjacents = vec![];
	let current_height = get_height(grid[y][x]);
	let mut push_adjacent = |pos: Pos| {
		if !visited.contains(&pos) && get_height(grid[pos.1][pos.0]) - current_height >= -1 {
			adjacents.push(pos);
		}
	};

	if x > 0 {
		push_adjacent((x - 1, y));
	}

	if x < grid[0].len() - 1 {
		push_adjacent((x + 1, y));
	}

	if y > 0 {
		push_adjacent((x, y - 1));
	}

	if y < grid.len() - 1 {
		push_adjacent((x, y + 1));
	}

	adjacents
}

fn find_end(grid: &Grid) -> Pos {
	for (y, row) in grid.iter().enumerate() {
		for (x, char) in row.iter().enumerate() {
			if *char == 'E' {
				return (x, y);
			}
		}
	}

	(0, 0)
}

fn find_shortest_path(grid: Grid, first_a: bool) -> u32 {
	let end = find_end(&grid);

	let mut steps = 0;
	let mut visitting = vec![end];
	let mut visited = HashSet::from([end]);
	let mut next_visit = vec![];

	while visitting.len() > 0 {
		for pos in visitting {
			for next_pos in get_adjacents(&grid, pos, &visited) {
				let char = grid[next_pos.1][next_pos.0];

				if char == 'S' || (first_a && char == 'a') {
					return steps + 1;
				}

				next_visit.push(next_pos);
				visited.insert(next_pos);
			}
		}

		visitting = next_visit;
		next_visit = vec![];
		steps += 1;
	}

	steps
}

pub fn day12a_solution() -> u32 {
	let input = read_input(12);
	let grid = input.lines().map(|line| line.chars().collect()).collect();

	find_shortest_path(grid, false)
}

pub fn day12b_solution() -> u32 {
	let input = read_input(12);
	let grid = input.lines().map(|line| line.chars().collect()).collect();

	find_shortest_path(grid, true)
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day12a_solution(), 449);
		assert_eq!(super::day12b_solution(), 443);
	}
}
