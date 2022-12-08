use crate::helpers::*;

type A1 = impl std::fmt::Display;
type A2 = impl std::fmt::Display;

#[derive(Debug)]
pub struct Solution {
	best_3: [i32; 3],
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		let mut best_3 = [0; 3];
		let mut current_num = 0;

		file.trim_ascii_end().consume_lines(|line| {
			if line[0] == b'\n' {
				if current_num > best_3[0] {
					best_3[0] = current_num;
					best_3.sort_unstable();
				}
				current_num = 0;
				Err(1)
			} else {
				let (n, size): (i32, _) = atoi::FromRadix10::from_radix_10(line);
				current_num += n;
				Err(size + 1)
			}
		});
		if current_num > best_3[0] {
			best_3[0] = current_num;
			best_3.sort_unstable();
		}

		// for chunk in file.lines() {
		// 	if chunk.is_empty() {
		// 		if current_num > best_3[0] {
		// 			best_3[0] = current_num;
		// 			best_3.sort_unstable();
		// 		}
		// 		current_num = 0;
		// 	} else {
		// 		current_num += chunk.parse::<i32>().unwrap();
		// 	}
		// }

		Self { best_3 }
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		*self.best_3.last().unwrap()
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.best_3.into_iter().sum_self()
	}

	fn run_any_write<W: std::fmt::Write>(&mut self, part: u32, _writer: W) -> Result<(), AocError> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}
