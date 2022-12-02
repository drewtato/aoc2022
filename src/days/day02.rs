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

	fn initialize(file: Vec<u8>) -> Self {
		let input: Vec<(u8, u8)> = file
			.trim_ascii()
			.lines()
			.map(|line| {
				let a = match line[0] {
					b'A' => 0,
					b'B' => 1,
					b'C' => 2,
					_ => panic!(),
				};
				let b = match line[2] {
					b'X' => 0,
					b'Y' => 1,
					b'Z' => 2,
					_ => panic!(),
				};
				(a, b)
			})
			.collect();

		let mut total = 0;
		for &(a, b) in &input {
			total +=
				(if a == b { 3 } else { 0 } + if b == (a + 1) % 3 { 6 } else { 0 } + b + 1) as i32;
		}

		let mut total2 = 0;
		for (a, x) in input {
			let b = match x {
				0 => (a + 2) % 3,
				1 => a,
				2 => (a + 1) % 3,
				_ => panic!(),
			};
			total2 +=
				(if a == b { 3 } else { 0 } + if b == (a + 1) % 3 { 6 } else { 0 } + b + 1) as i32;
			// println!("a: {} b: {} x: {}, total: {}", a, b, x, total2);
		}

		Self {
			p1: total,
			p2: total2,
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
