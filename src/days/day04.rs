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
		let mut file_slice = file.as_slice();
		let input = std::iter::from_fn(|| -> Option<i32> {
			if file_slice.is_empty() {
				return None;
			}
			let (i, consumed) = atoi::FromRadix10::from_radix_10(file_slice);
			file_slice = file_slice.split_at(consumed + 1).1;
			Some(i)
		})
		.array_chunks::<4>();

		let mut count = 0;
		let mut count2 = 0;
		for [a, b, c, d] in input {
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
