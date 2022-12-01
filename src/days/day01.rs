use crate::helpers::*;

type A1 = impl std::fmt::Display;
type A2 = impl std::fmt::Display;

#[derive(Debug)]
pub struct Solution {
	input: Vec<Vec<i32>>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		let r = Regex::new("\n\n").unwrap();
		let input = r
			.split(&file)
			.map(|chunk| chunk.lines().map(|line| line.parse().unwrap()).collect())
			.collect();

		Self { input }
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		self.input
			.iter()
			.map(|v| v.iter().copied().self_sum())
			.max()
			.unwrap()
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		let mut top_3 = [0; 4];

		for v in &self.input {
			let sum = v.iter().copied().self_sum();
			top_3[0] = sum;
			top_3.sort_unstable();
		}

		top_3[1..4].iter().copied().self_sum()
	}

	fn run_any(&mut self, part: u32) -> Res<String> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}
