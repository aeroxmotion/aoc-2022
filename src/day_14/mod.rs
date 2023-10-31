use crate::shared::read_input;

#[repr(u8)]
#[derive(Clone, PartialEq)]
enum Unit {
	Sand = 'o' as u8,
	Air = '.' as u8,
	Rock = '#' as u8,
}

pub fn day14a_solution() -> u32 {
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
	let mut x_range = (usize::MAX, 0);

	for ins in instructions.clone() {
		for (x, y) in ins {
			y_max = y_max.max(y);
			x_range = (x.min(x_range.0), x.max(x_range.1));
		}
	}

	let mut grid = vec![vec![Unit::Air; x_range.1 - x_range.0 + 1]; y_max + 1];

	for ins in instructions.clone() {
		let coords = ins.collect::<Vec<(usize, usize)>>();

		for i in 0..(coords.len() - 1) {
			let coord = coords[i];
			let next_coord = coords[i + 1];

			if coord.0 == next_coord.0 {
				for y in coord.1.min(next_coord.1)..=next_coord.1.max(coord.1) {
					grid[y][coord.0 - x_range.0] = Unit::Rock;
				}
			} else {
				for x in coord.0.min(next_coord.0)..=next_coord.0.max(coord.0) {
					grid[coord.1][x - x_range.0] = Unit::Rock;
				}
			}
		}
	}

	let mut count = 0;
	let mut sand = (500, 0);
	let x_deltas = [0, -1i32, 1];

	'root: loop {
		for x in x_deltas {
			let target_y = sand.1 + 1;
			let target_x = (sand.0 as i32 - x_range.0 as i32) + x;

			if target_y > y_max || target_x < 0 || target_x >= grid[0].len() as i32 {
				break 'root;
			}

			if grid[target_y][target_x as usize] == Unit::Air {
				sand = (target_x as usize + x_range.0, target_y);
				continue 'root;
			}
		}

		grid[sand.1][sand.0 - x_range.0] = Unit::Sand;
		count += 1;
		sand = (500, 0);
	}

	count
}
