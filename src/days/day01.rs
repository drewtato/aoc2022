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

		for chunk in file.lines() {
			if chunk.is_empty() {
				if current_num > best_3[0] {
					best_3[0] = current_num;
					best_3.sort_unstable();
				}
				current_num = 0;
			} else {
				current_num += chunk.parse::<i32>().unwrap();
			}
		}

		Self { best_3 }
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		*self.best_3.last().unwrap()
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.best_3.into_iter().sum_self()
	}

	fn run_any(&mut self, part: u32) -> Res<String> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}
