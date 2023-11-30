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
		let mut p1: u32 = 0;
		let mut p2: u32 = 0;
		let mut chunk = [0; 3];
		let mut i = 0;

		file.consume_lines(|line| {
			let len = line.iter().position(|&b| b == b'\n').ok_or(1usize)?;
			let first = &line[..len / 2];
			let second = &line[len / 2..len];

			let mut first_map: u64 = 0;
			let mut second_map: u64 = 0;

			for &bit in first {
				first_map |= 1 << priority(bit);
			}

			for &bit in second {
				second_map |= 1 << priority(bit);
			}

			let common = first_map & second_map;
			let priority = common.ilog2();
			p1 += priority;

			chunk[i % 3] = first_map | second_map;
			if i % 3 == 2 {
				let [a, b, c] = chunk;
				let badge = a & b & c;
				let priority = badge.ilog2();
				p2 += priority;
			}
			i += 1;
			Err(len + 1)
		});

		Self { p1, p2 }
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

fn priority(item: u8) -> u8 {
	match item {
		b'a'..=b'z' => item - b'a' + 1,
		b'A'..=b'Z' => item - b'A' + 27,
		_ => panic!(),
	}
}
