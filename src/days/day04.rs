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
		let input = file.trim_ascii().lines().map(|line| -> Vec<i32> {
			line.split(|&c| c == b',' || c == b'-')
				.map(|n| n.parse().unwrap())
				.collect()
		});

		let mut count = 0;
		let mut count2 = 0;
		for list in input {
			let &[a, b, c, d] = list.as_slice() else { panic!() };
			if ((a <= c) && (b >= d)) || ((a >= c) && (b <= d)) {
				count += 1;
			}
			if (a..=b).contains(&c)
				|| (a..=b).contains(&d)
				|| (c..=d).contains(&a)
				|| (c..=d).contains(&b)
			{
				count2 += 1;
			}
		}

		Self {
			p1: count,
			p2: count2,
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
