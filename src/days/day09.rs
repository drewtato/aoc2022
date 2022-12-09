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
			.map(|line| (line[0], line[2..].parse().unwrap()))
			.collect();

		let mut tail_positions = HashSet::from([[0, 0]]);
		let mut tenth_tail_positions = HashSet::from([[0, 0]]);
		let mut knots = vec![[0i32, 0]; 10];

		for &(dir, count) in &input {
			for _ in 0..count {
				match dir {
					b'R' => knots[0][1] += 1,
					b'L' => knots[0][1] -= 1,
					b'U' => knots[0][0] -= 1,
					b'D' => knots[0][0] += 1,
					_ => panic!(),
				}
				let mut unmoved_knots = knots.as_mut_slice();
				loop {
					let head = unmoved_knots.take_first_mut().unwrap();
					let Some(tail) = unmoved_knots.first_mut() else { break; };
					let a = head[0] - tail[0];
					let b = head[1] - tail[1];
					if a.abs() > 1 || b.abs() > 1 {
						tail[0] += a.signum();
						tail[1] += b.signum();
					}
				}
				tail_positions.insert(knots[1]);
				tenth_tail_positions.insert(knots[9]);
			}
		}

		Self {
			p1: tail_positions.len(),
			p2: tenth_tail_positions.len(),
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
