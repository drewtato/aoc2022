#![allow(unused)]

use crate::helpers::*;

pub type A1 = impl std::fmt::Display + std::fmt::Debug + Clone;
pub type A2 = impl std::fmt::Display + std::fmt::Debug + Clone;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let input: Vec<i32> = file
			.trim_ascii()
			.lines()
			.map(|line| line.trim_ascii().parse().unwrap())
			.collect();

		Self {
			p1: "Part 1 not implemented",
			p2: "Part 2 not implemented",
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.p1.clone()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.p2.clone()
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
