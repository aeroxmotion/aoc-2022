use crate::shared::read_input;

pub fn day14a_solution() {
	let input = read_input(14);
	let instructions = input.lines().map(|line| {
		line.split(" -> ").map(|coords| {
			let splitted = coords
				.split(",")
				.map(|coord| coord.parse().unwrap())
				.collect::<Vec<usize>>();

			(splitted[0], splitted[1])
		})
	});

	let mut y_max = 0;
	let mut x_range = (0, 0);

	for ins in instructions.clone() {
		for (x, y) in ins {
			y_max = y_max.max(y);
			x_range = (x.min(x_range.0), x.max(x_range.1));
		}
	}

	let mut grid = vec![vec!['.'; x_range.1 - x_range.0]; y_max];

	for ins in instructions.clone() {
		let coords = ins.collect::<Vec<(usize, usize)>>();

		for i in 0..(coords.len() - 1) {
			let coord = coords[i];
			let next_coord = coords[i + 1];
		}
	}
}
