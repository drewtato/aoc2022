#![allow(unused)]

use crate::helpers::*;

type A1 = impl std::fmt::Display + std::fmt::Debug + Clone;
type A2 = impl std::fmt::Display + std::fmt::Debug + Clone;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut input: Vec<i32> = file
			.trim_ascii()
			.lines()
			.map(|line| line.parse().unwrap())
			.collect();

		let queue = input.clone();

		// dbg!(input.iter().min().unwrap());
		let len = input.len() - 1;

		for n in queue {
			let mut pos = input.iter().position(|&d| d == n).unwrap();
			input.remove(pos);
			let index = if n >= 0 {
				(pos + len * 10 + n as usize) % len
			} else {
				(pos + len * 10 - (-n as usize)) % len
			};
			input.insert(index, n);
			// dbg_small!(&input);
		}

		let zero_pos = input.iter().position(|&d| d == 0).unwrap();

		let &a = input.iter().cycle().nth(1000 + zero_pos).unwrap();
		let &b = input.iter().cycle().nth(2000 + zero_pos).unwrap();
		let &c = input.iter().cycle().nth(3000 + zero_pos).unwrap();

		dbg_small!(a, b, c);

		Self {
			p1: a + b + c,
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
