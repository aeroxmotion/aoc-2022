use crate::shared::read_input;

fn day6_solution(distinct_chars: usize) -> usize {
	let input = read_input(6);
	let bytes = input.as_bytes();

	'root: for i in 0..input.len() {
		let end = i + (distinct_chars - 1);
		let mut j = i;
		let mut k = end;

		while j < end {
			if bytes[j] == bytes[k] {
				continue 'root;
			}

			if k > j + 1 {
				k -= 1;
			} else {
				j += 1;
				k = end;
			}
		}

		return end + 1;
	}

	0
}

pub fn day6a_solution() -> usize {
	day6_solution(4)
}

pub fn day6b_solution() -> usize {
	day6_solution(14)
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day6a_solution(), 1_198);
		assert_eq!(super::day6b_solution(), 3_120);
	}
}
