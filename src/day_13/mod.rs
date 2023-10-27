use crate::shared::read_input;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, PartialEq)]
enum Value {
	Num(u8),
	Arr(Vec<Value>),
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Value::Arr(arr) => write!(
				f,
				"[{}]",
				arr.iter()
					.map(|value| format!("{}", value))
					.collect::<Vec<String>>()
					.join(", ")
			),
			Value::Num(num) => write!(f, "{}", num),
		}
	}
}

impl Into<Value> for [u8; 1] {
	fn into(self) -> Value {
		vec![self[0].into()].into()
	}
}

impl Into<Value> for Vec<Value> {
	fn into(self) -> Value {
		Value::Arr(self)
	}
}

impl Into<Value> for u8 {
	fn into(self) -> Value {
		Value::Num(self)
	}
}

fn parse(input: &str) -> Value {
	let mut parsed = vec![];
	let mut num_to_parse = String::new();

	for char in input.as_bytes() {
		match *char as char {
			' ' => {}
			'[' => {
				parsed.push(vec![]);
			}
			',' => {
				if !num_to_parse.is_empty() {
					parsed
						.last_mut()
						.unwrap()
						.push(num_to_parse.parse::<u8>().unwrap().into());

					num_to_parse = String::new();
				}
			}
			']' => {
				let mut target = parsed.pop().unwrap();

				if !num_to_parse.is_empty() {
					target.push(num_to_parse.parse::<u8>().unwrap().into());
					num_to_parse = String::new();
				}

				if parsed.is_empty() {
					return target.into();
				}

				parsed.last_mut().unwrap().push(target.into());
			}
			n => num_to_parse.push(n),
		}
	}

	vec![].into()
}

fn cmp<A, B>(a: A, b: B) -> Ordering
where
	A: Into<Value>,
	B: Into<Value>,
{
	match (a.into(), b.into()) {
		(Value::Num(a_num), Value::Num(b_num)) => a_num.cmp(&b_num),
		(Value::Num(a_num), Value::Arr(b_arr)) => cmp([a_num], b_arr),
		(Value::Arr(a_arr), Value::Num(b_num)) => cmp(a_arr, [b_num]),
		(Value::Arr(mut a_arr), Value::Arr(mut b_arr)) => {
			match a_arr.is_empty() || b_arr.is_empty() {
				true => a_arr.len().cmp(&b_arr.len()),
				_ => match cmp(a_arr.remove(0), b_arr.remove(0)) {
					Ordering::Equal => cmp(a_arr, b_arr),
					result => result,
				},
			}
		}
	}
}

pub fn day13a_solution() -> usize {
	let input = read_input(13);
	let mut lines = input.lines();
	let mut sum = 0;
	let mut i = 1;

	loop {
		let a = parse(lines.next().unwrap());
		let b = parse(lines.next().unwrap());

		if cmp(a, b).is_lt() {
			sum += i;
		}

		match lines.next() {
			None => break,
			_ => {}
		}

		i += 1;
	}

	sum
}

pub fn day13b_solution() -> usize {
	let input = read_input(13);
	let lines = input.lines();
	let sentinels = [parse("[[2]]"), parse("[[6]]")];

	let mut result = 1;
	let mut arrs: Vec<Value> = lines.filter(|l| !l.is_empty()).map(parse).collect();

	arrs.append(&mut sentinels.to_vec());
	arrs.sort_by(|a, b| cmp(a.clone(), b.clone()));

	for (i, arr) in arrs.iter().enumerate() {
		if sentinels.contains(arr) {
			result *= i + 1;

			if result != i + 1 {
				break;
			}
		}
	}

	result
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day13a_solution(), 5_529);
		assert_eq!(super::day13b_solution(), 27_690);
	}
}
