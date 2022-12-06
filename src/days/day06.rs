#![allow(unused)]

use crate::helpers::*;

type A1 = usize;
type A2 = usize;

#[derive(Debug)]
pub struct Solution {
	file: Vec<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		Self { file }
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		self.find_consecutive_unique::<4>() + 4
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.find_consecutive_unique::<14>() + 14
	}

	fn run_any_write<W: std::fmt::Write>(&mut self, part: u32, _writer: W) -> Res<()> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}

impl Solution {
	fn find_consecutive_unique<const N: usize>(&self) -> usize {
		self.file
			.array_windows::<N>()
			.position(|&window| HashSet::from(window).len() == N)
			.unwrap()
	}
}
