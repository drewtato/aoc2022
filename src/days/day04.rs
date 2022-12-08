use crate::helpers::*;

type A1 = i32;
type A2 = i32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
	input: Vec<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		let mut solution = Self {
			p1: 0,
			p2: 0,
			input: file,
		};

		for [a, b, c, d] in file_to_ints(&solution.input).array_chunks() {
			if ((a <= c) && (b >= d)) || ((a >= c) && (b <= d)) {
				solution.p1 += 1;
			}
			if (a..=b).contains(&c)
				|| (a..=b).contains(&d)
				|| (c..=d).contains(&a)
				|| (c..=d).contains(&b)
			{
				solution.p2 += 1;
			}
		}

		solution
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		self.p1
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.p2
	}

	fn run_any_write<W: std::fmt::Write>(&mut self, part: u32, mut writer: W) -> Res<()> {
		#[allow(clippy::match_single_binding)]
		match part {
			3 => Ok(write!(writer, "{:?}", self.part_three())?),
			_ => Err(AocError::PartNotFound),
		}
	}
}

fn file_to_ints(mut file: &[u8]) -> impl Iterator<Item = i32> + '_ {
	std::iter::from_fn(move || {
		if file.is_empty() {
			return None;
		}
		let i = parse_consume_unsigned(&mut file);
		file.take_first();
		Some(i)
	})
}

impl Solution {
	fn part_three(&mut self) -> impl std::fmt::Debug {
		let mut all_plots = vec![0; 100];
		for [a, b] in file_to_ints(&self.input).array_chunks() {
			for n in a..=b {
				all_plots[n as usize] += 1;
			}
		}
		all_plots
	}
}
