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
		let r = Regex::new("\n\n").unwrap();
		let split_file = r.split(&file);

		let mut best_4 = [0; 4];

		for chunk in split_file {
			let n = chunk.lines().map(|line| line.parse().unwrap()).self_sum();
			best_4[0] = n;
			best_4.sort_unstable();
		}

		Self {
			best_3: [best_4[1], best_4[2], best_4[3]],
		}
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		*self.best_3.last().unwrap()
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.best_3.into_iter().self_sum()
	}

	fn run_any(&mut self, part: u32) -> Res<String> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}
