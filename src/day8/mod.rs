use std::cmp;

use crate::shared::read_input;

type Tress = Vec<Vec<String>>;

const DELTAS: [(i8, i8); 4] = [
	(-1, 0), // top
	(0, -1), // left
	(0, 1),  // right
	(1, 0),  // bottom
];

pub fn parse_tree(value: String) -> i8 {
	value.parse().unwrap()
}

pub fn is_visible(
	trees: &Tress,
	row_index: usize,
	col_index: usize,
	row_delta: i8,
	col_delta: i8,
	value: i8,
) -> bool {
	if row_index <= 0
		|| col_index <= 0
		|| row_index >= trees.len() - 1
		|| col_index >= trees[row_index].len() - 1
	{
		return true;
	}

	let next_row_index = (row_index as i8 + row_delta) as usize;
	let next_col_index = (col_index as i8 + col_delta) as usize;

	let next_value = parse_tree(trees[next_row_index][next_col_index].clone());

	return value > next_value
		&& is_visible(
			trees,
			next_row_index,
			next_col_index,
			row_delta,
			col_delta,
			value,
		);
}

pub fn get_total_visible_trees(
	trees: &Tress,
	row_index: usize,
	col_index: usize,
	row_delta: i8,
	col_delta: i8,
	value: i8,
) -> u32 {
	if row_index <= 0
		|| col_index <= 0
		|| row_index >= trees.len() - 1
		|| col_index >= trees[row_index].len() - 1
	{
		return 0;
	}

	let next_row_index = (row_index as i8 + row_delta) as usize;
	let next_col_index = (col_index as i8 + col_delta) as usize;

	let next_value = parse_tree(trees[next_row_index][next_col_index].clone());

	if next_value >= value {
		return 1;
	}

	return 1 + get_total_visible_trees(
		trees,
		next_row_index,
		next_col_index,
		row_delta,
		col_delta,
		value,
	);
}

pub fn day8a_solution() -> u32 {
	let input = read_input(8);
	let trees: Tress = input
		.lines()
		.map(|l| l.chars().map(|c| c.to_string()).collect())
		.collect();
	let mut visible_trees = 0;

	for (row_index, row) in trees.iter().enumerate() {
		for (col_index, tree) in row.iter().enumerate() {
			let value = parse_tree(tree.clone());

			for (row_delta, col_delta) in DELTAS {
				if is_visible(&trees, row_index, col_index, row_delta, col_delta, value) {
					visible_trees += 1;
					break;
				}
			}
		}
	}

	visible_trees
}

pub fn day8b_solution() -> u32 {
	let input = read_input(8);
	let trees: Tress = input
		.lines()
		.map(|l| l.chars().map(|c| c.to_string()).collect())
		.collect();
	let mut max_visible_trees = 0;

	for (row_index, row) in trees.iter().enumerate() {
		for (col_index, tree) in row.iter().enumerate() {
			let value = parse_tree(tree.clone());
			let mut visible_trees = 1;

			for (row_delta, col_delta) in DELTAS {
				visible_trees *= get_total_visible_trees(
					&trees, row_index, col_index, row_delta, col_delta, value,
				);
			}

			max_visible_trees = cmp::max(max_visible_trees, visible_trees);
		}
	}

	max_visible_trees
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day8a_solution(), 1_794);
		assert_eq!(super::day8b_solution(), 199_272);
	}
}
