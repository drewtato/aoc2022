#![allow(unused)]

use crate::helpers::*;

type A1 = impl std::fmt::Display;
type A2 = impl std::fmt::Display;

#[derive(Debug)]
pub struct Solution {
	input: Vec<i32>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		Self {
			input: file.trim_ascii().lines().multi_parse().unwrap(),
		}
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		"Part 1 not implemented"
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		"Part 2 not implemented"
	}

	fn run_any(&mut self, part: u32) -> Result<String, AocError> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}
