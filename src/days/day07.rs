use std::fmt::Debug;

use crate::helpers::*;

type A1 = impl std::fmt::Display + std::fmt::Debug + Clone;
type A2 = impl std::fmt::Display + std::fmt::Debug + Clone;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

type Name = Vec<u8>;

#[derive(Clone)]
enum Entry {
	Dir(Name, usize),
	File(Name, usize),
}
use Entry::*;

impl Debug for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Dir(ref a, b) => write!(f, "Dir({:?}, {})", std::str::from_utf8(a).unwrap(), b),
			File(ref a, b) => write!(f, "File({:?}, {})", std::str::from_utf8(a).unwrap(), b),
		}
	}
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		let mut entries = HashMap::new();
		entries.insert(b"/".to_vec(), Dir(b"".to_vec(), 0));
		let mut parent_dir: Vec<u8> = Vec::new();
		for line in file.trim_ascii_end().lines() {
			// println!("{}", std::str::from_utf8(line).unwrap());
			let words = line.split(is(&b' ')).collect_vec();
			let (name, entry) = match words[0] {
				b"$" => {
					match words[1] {
						b"cd" => match words[2] {
							b"/" => {
								parent_dir.clear();
								parent_dir.extend_from_slice(b"/")
							}
							b".." => {
								while parent_dir.pop().is_some() {
									if *parent_dir.last().unwrap() == b'/' {
										break;
									}
								}
							}
							_ => {
								parent_dir.extend_from_slice(words[2]);
								parent_dir.push(b'/');
							}
						},
						b"ls" => (),
						_ => panic!("Invalid command"),
					}
					continue;
				}
				b"dir" => {
					let mut name = parent_dir.clone();
					name.extend_from_slice(words[1]);
					name.push(b'/');
					(name, Dir(parent_dir.clone(), 0))
				}
				size => {
					let mut filename = parent_dir.clone();
					filename.extend_from_slice(words[1]);
					(filename, File(parent_dir.clone(), size.parse().unwrap()))
				}
			};
			// println!("{:?} {:?}", std::str::from_utf8(&name).unwrap(), entry);
			entries.insert(name, entry);
		}

		let entries_copy = entries.clone();
		// println!("Adding sizes");
		for (_name, file) in entries_copy {
			// println!("{}: {:?}", std::str::from_utf8(&name).unwrap(), file);
			if let File(mut parent, size) = file {
				while !parent.is_empty() {
					// println!("parent is {:?}", entries[&parent]);
					if let Some(Dir(p, s)) = entries.get_mut(&parent) {
						*s += size;
						p.clone_into(&mut parent);
					// break;
					} else {
						panic!("Parent is not a dir");
					}
				}
			}
		}

		let mut total = 0;
		let used_bytes = if let Dir(_, s) = entries[&b"/".to_vec()] {
			s
		} else {
			panic!("No root dir")
		};
		let total_bytes = 70000000;
		let needed_bytes = 30000000;
		let delete_at_least_this_much = needed_bytes - (total_bytes - used_bytes);
		let mut size_of_best_dir = total_bytes;
		for (_, entry) in &entries {
			if let &Dir(_, s) = entry {
				if s < 100000 {
					total += s;
				}
				if s > delete_at_least_this_much {
					size_of_best_dir = size_of_best_dir.min(s);
				}
			}
		}

		Self {
			p1: total,
			p2: size_of_best_dir,
		}
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		self.p1.clone()
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.p2.clone()
	}

	fn run_any_write<W: std::fmt::Write>(&mut self, part: u32, _writer: W) -> Res<()> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}
