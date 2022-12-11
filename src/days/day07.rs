use atoi::FromRadix10;

use crate::helpers::*;

type A1 = u32;
type A2 = A1;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut dir_sizes = Vec::with_capacity(250);
		let mut working_sizes = Vec::with_capacity(32);

		consume_method(file, &mut working_sizes, &mut dir_sizes);
		// manual_method(file, &mut working_sizes, &mut dir_sizes);

		let mut total = 0;
		for leftover in working_sizes.into_iter().rev() {
			total += leftover;
			dir_sizes.push(total);
		}

		let mut total_low_size = 0;

		let total_size = 70000000;
		let used_size = dir_sizes.pop().unwrap();
		if used_size < 100000 {
			total_low_size += used_size;
		}
		let free_size = total_size - used_size;
		let needed_size = 30000000 - free_size;
		let mut size_of_best_dir = A2::MAX;

		for size in dir_sizes {
			if size >= needed_size {
				size_of_best_dir = size_of_best_dir.min(size);
			}
			if size <= 100000 {
				total_low_size += size;
			}
		}

		Self {
			p1: total_low_size,
			p2: size_of_best_dir,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.p1
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.p2
	}

	fn run_any<W: std::fmt::Write>(
		&mut self,
		part: u32,
		_writer: W,
		_: u8,
	) -> Res<std::time::Duration> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}

#[allow(dead_code)]
fn manual_method(file: Vec<u8>, working_sizes: &mut Vec<u32>, dir_sizes: &mut Vec<u32>) {
	let mut file = file.as_slice();
	loop {
		let Some(&first) = file.first() else { break; };
		let skip = match first {
			b'$' => match file[2] {
				b'c' => match file[5] {
					b'.' => {
						let size = working_sizes.pop().unwrap();
						*working_sizes.last_mut().unwrap() += size;
						dir_sizes.push(size);
						7
					}
					_dirname => {
						working_sizes.push(0);
						6
					}
				},
				b'l' => 4,
				_ => panic!("Unknown command"),
			},
			b'd' => 5,
			_digit => {
				let (file_size, skip): (A1, _) = FromRadix10::from_radix_10(file);
				*working_sizes.last_mut().unwrap() += file_size;
				skip
			}
		};
		file = &file[skip..];
		while file[0] != b'\n' {
			file = &file[1..];
		}
		file = &file[1..];
	}
}

#[allow(dead_code)]
fn consume_method(file: Vec<u8>, working_sizes: &mut Vec<u32>, dir_sizes: &mut Vec<u32>) {
	file.trim_ascii_end().consume_lines(|line| match line[0] {
		b'$' => match line[2] {
			b'c' => match line[5] {
				b'.' => {
					let size = working_sizes.pop().unwrap();
					*working_sizes.last_mut().unwrap() += size;
					dir_sizes.push(size);
					Err(8)
				}
				b'/' => {
					working_sizes.push(0);
					Err(7)
				}
				_dirname => {
					working_sizes.push(0);
					Ok(6)
				}
			},
			b'l' => Err(5),
			_ => panic!("Unknown command"),
		},
		b'd' => Ok(5),
		_digit => {
			let (file_size, skip): (A1, _) = FromRadix10::from_radix_10(line);
			*working_sizes.last_mut().unwrap() += file_size;
			Ok(skip)
		}
	});
}
