// TODO: !!!Improve this code!!!

use crate::shared::read_input;
use std::{cmp, collections::HashMap};

#[derive(Debug)]
pub enum Entry {
	Dir(Vec<String>),
	File(u32),
}

pub struct FileSystem {
	pub cwd: Vec<String>,
	pub entries: HashMap<String, Entry>,
}

impl FileSystem {
	pub fn new() -> FileSystem {
		let mut entries = HashMap::new();

		entries.insert("/".into(), Entry::Dir(vec![]));

		FileSystem {
			cwd: vec!["/".into()],
			entries,
		}
	}

	pub fn create_dir(&mut self, name: String) {
		let current_path = self.cwd.join("/");
		let dirname = format!("{}/{}", current_path, name);

		if let Entry::Dir(cwd) = self.entries.get_mut(&current_path).unwrap() {
			cwd.push(dirname.clone());
			self.entries.insert(
				format!("{}/{}", self.cwd.join("/"), name),
				Entry::Dir(vec![]),
			);
		}
	}

	pub fn create_file(&mut self, name: String, size: u32) {
		let current_path = self.cwd.join("/");
		let filepath = format!("{}/{}", current_path, name);

		if let Entry::Dir(cwd) = self.entries.get_mut(&current_path).unwrap() {
			cwd.push(filepath.clone());
			self.entries.insert(filepath, Entry::File(size));
		}
	}

	pub fn change_dir(&mut self, name: String) {
		match name.as_str() {
			"/" => self.cwd = vec!["/".into()],
			".." => {
				self.cwd.pop();
			}
			_ => {
				self.cwd.push(name.into());
			}
		}
	}

	pub fn get_entry(&self, path: &str) -> &Entry {
		self.entries.get(path).unwrap()
	}

	pub fn get_entry_size(&self, path: &str) -> u32 {
		let entry = self.get_entry(path);

		match entry {
			Entry::File(size) => *size,
			Entry::Dir(entries) => {
				let mut total_size = 0;

				for path in entries.iter() {
					total_size += self.get_entry_size(path);
				}

				total_size
			}
		}
	}

	pub fn compute_entry_size(&self, path: &str, sum: &mut u32) -> u32 {
		let entry = self.get_entry(path);

		match entry {
			Entry::File(size) => *size,
			Entry::Dir(entries) => {
				let mut total_size = 0;

				for path in entries.iter() {
					total_size += self.compute_entry_size(path, sum);
				}

				if total_size <= 100_000 {
					*sum += total_size
				}

				total_size
			}
		}
	}

	pub fn find_target_space(&self, path: &str, target_free_space: u32, target: &mut u32) -> u32 {
		let entry = self.get_entry(path);

		match entry {
			Entry::File(size) => *size,
			Entry::Dir(entries) => {
				let mut total_size = 0;

				for path in entries.iter() {
					total_size += self.find_target_space(path, target_free_space, target);
				}

				if total_size >= target_free_space {
					*target = cmp::min(*target, total_size);
				}

				total_size
			}
		}
	}
}

const TOTAL_DISK_SPACE: u32 = 70_000_000;
const TARGET_FREE_SPACE: u32 = 30_000_000;

pub fn get_initial_fs() -> FileSystem {
	let input = read_input(7);
	let instructions = input.lines();
	let mut fs = FileSystem::new();

	for ins in instructions {
		let parts: Vec<&str> = ins.split(" ").collect();

		match parts[0] {
			"$" => {
				if parts[1] == "cd" {
					fs.change_dir(parts[2].into());
				}

				// Ignore ls ...
			}
			"dir" => fs.create_dir(parts[1].into()),
			size => fs.create_file(parts[1].into(), size.parse::<u32>().unwrap()),
		}
	}

	fs
}

pub fn day7a_solution() -> u32 {
	let fs = get_initial_fs();
	let mut sum = 0;

	fs.compute_entry_size("/", &mut sum);

	sum
}

pub fn day7b_solution() -> u32 {
	let fs = get_initial_fs();
	let used_space = fs.get_entry_size("/");
	let free_space = TOTAL_DISK_SPACE - used_space;
	let missing_free_space = TARGET_FREE_SPACE - free_space;
	let mut target = TOTAL_DISK_SPACE;

	fs.find_target_space("/", missing_free_space, &mut target);

	target
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_should_find_the_solution() {
		assert_eq!(super::day7a_solution(), 1_743_217);
		assert_eq!(super::day7b_solution(), 8_319_096);
	}
}
