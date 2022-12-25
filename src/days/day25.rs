use crate::helpers::*;

type A1 = DisplaySlice<Vec<u8>>;
type A2 = &'static str;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut total_fuel: i64 = file
			.trim_ascii()
			.lines()
			.map(|line| {
				let mut total = 0;
				for c in line {
					total *= 5;
					total += match c {
						b'2' => 2,
						b'1' => 1,
						b'0' => 0,
						b'-' => -1,
						b'=' => -2,
						_ => panic!(),
					}
				}
				total
			})
			.sum_self();

		let mut p1 = Vec::new();
		while total_fuel != 0 {
			let frac = ((total_fuel + 2) % 5) - 2;
			let c = match frac {
				2 => b'2',
				1 => b'1',
				0 => b'0',
				-1 => b'-',
				-2 => b'=',
				_ => panic!(),
			};
			total_fuel -= frac;
			total_fuel /= 5;
			p1.push(c);
		}

		p1.reverse();

		Self {
			p1: p1.to_display_slice(),
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.p1.clone()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		"🎄🌟🎈🌋🐒🐘"
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
