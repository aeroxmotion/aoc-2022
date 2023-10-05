use std::collections::HashMap;

use crate::shared::read_input;

#[derive(Clone, Debug)]
enum Operation {
	Add(u128),
	Mul(u128),
}

impl Default for Operation {
	fn default() -> Self {
		Self::Add(0)
	}
}

#[derive(Debug, Clone, Default)]
struct Test {
	condition: u128,
	true_monkey: usize,
	false_monkey: usize,
}

#[derive(Debug, Clone, Default)]
struct Monkey {
	items: Vec<u128>,
	operation: Operation,
	test: Test,
	inspected: u128,
}

fn parse_monkeys() -> Vec<Monkey> {
	let input = read_input(11);
	let lines = input.lines();

	let mut monkeys = vec![];
	let mut monkey = Monkey::default();

	for (i, line) in lines.enumerate() {
		let mut parts: Vec<&str> = line.split(":").collect();

		parts.push("");

		let inner_parts: Vec<&str> = parts[1].trim().split(" ").collect();

		match i % 7 {
			0 => {
				monkey = Monkey::default();
			}
			1 => {
				monkey.items = parts[1]
					.trim()
					.split(",")
					.map(|n| n.trim().parse().unwrap())
					.collect();
			}
			2 => {
				let value = match inner_parts[4] {
					"old" => 0,
					n => n.parse().unwrap(),
				};

				monkey.operation = match inner_parts[3] {
					"*" => Operation::Mul(value),
					"+" => Operation::Add(value),
					_ => panic!("Unknown operator..."),
				};
			}
			3 => {
				monkey.test.condition = inner_parts[2].parse().unwrap();
			}
			4 => {
				monkey.test.true_monkey = inner_parts[3].parse().unwrap();
			}
			5 => {
				monkey.test.false_monkey = inner_parts[3].parse().unwrap();
				monkeys.push(monkey.clone());
			}
			_ => {
				// Do nothing
			}
		}
	}

	monkeys
}

fn day11_solution(rounds: u32, divide: bool) -> u128 {
	let mut monkeys = parse_monkeys();
	let mut modulus = 1;
	let mut next_worry_levels: HashMap<usize, Vec<u128>> = HashMap::new();

	for monkey in monkeys.iter() {
		modulus *= monkey.test.condition;
	}

	for _ in 0..rounds {
		for (i, monkey) in monkeys.iter_mut().enumerate() {
			match next_worry_levels.get_mut(&i) {
				Some(items) => {
					monkey.items.append(items);
					next_worry_levels.remove(&i);
				}
				None => {}
			}

			while monkey.items.len() > 0 {
				let item = monkey.items.remove(0);
				let get_n = |n: u128| if n == 0 { item } else { n };

				let mut next_worry_level = match monkey.operation {
					Operation::Add(n) => item + get_n(n),
					Operation::Mul(n) => item * get_n(n),
				} % modulus;

				if divide {
					next_worry_level /= 3;
				}

				let target_monkey = if next_worry_level % monkey.test.condition == 0 {
					monkey.test.true_monkey
				} else {
					monkey.test.false_monkey
				};

				match next_worry_levels.get_mut(&target_monkey) {
					Some(items) => {
						items.push(next_worry_level);
					}
					None => {
						next_worry_levels.insert(target_monkey, vec![next_worry_level]);
					}
				}

				monkey.inspected += 1;
			}
		}
	}

	monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

	monkeys[0].inspected * monkeys[1].inspected
}

pub fn day11a_solution() -> u128 {
	day11_solution(20, true)
}

pub fn day11b_solution() -> u128 {
	day11_solution(10_000, false)
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day11a_solution(), 99_852);
	}
}
