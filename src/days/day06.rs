use crate::helpers::*;

type A1 = usize;
type A2 = usize;

#[derive(Debug)]
pub struct Solution {
	file: Vec<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		// self.find_consecutive_unique::<4>() + 4
		self.find_consecutive_unique_compare_4() + 4
		// self.find_consecutive_unique_compare::<4>() + 4
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.find_consecutive_unique::<14>() + 14
		// self.find_consecutive_unique_compare_14() + 14
		// self.find_consecutive_unique_compare::<14>() + 14
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

impl Solution {
	pub fn find_consecutive_unique_compare<const N: usize>(&self) -> usize {
		self.file
			.array_windows::<N>()
			.position(|a| a.iter().copied().tuple_combinations().all(|(a, b)| a != b))
			.unwrap()
	}

	pub fn find_consecutive_unique_compare_4(&self) -> usize {
		self.file
			.array_windows::<4>()
			.position(|a| {
				a[0] != a[1]
					&& a[0] != a[2] && a[0] != a[3]
					&& a[1] != a[2] && a[1] != a[3]
					&& a[2] != a[3]
			})
			.unwrap()
	}
	pub fn find_consecutive_unique_compare_14(&self) -> usize {
		self.file
			.array_windows::<14>()
			.position(|a| {
				a[0] != a[1]
					&& a[0] != a[2] && a[0] != a[3]
					&& a[0] != a[4] && a[0] != a[5]
					&& a[0] != a[6] && a[0] != a[7]
					&& a[0] != a[8] && a[0] != a[9]
					&& a[0] != a[10] && a[0] != a[11]
					&& a[0] != a[12] && a[0] != a[13]
					&& a[1] != a[2] && a[1] != a[3]
					&& a[1] != a[4] && a[1] != a[5]
					&& a[1] != a[6] && a[1] != a[7]
					&& a[1] != a[8] && a[1] != a[9]
					&& a[1] != a[10] && a[1] != a[11]
					&& a[1] != a[12] && a[1] != a[13]
					&& a[2] != a[3] && a[2] != a[4]
					&& a[2] != a[5] && a[2] != a[6]
					&& a[2] != a[7] && a[2] != a[8]
					&& a[2] != a[9] && a[2] != a[10]
					&& a[2] != a[11] && a[2] != a[12]
					&& a[2] != a[13] && a[3] != a[4]
					&& a[3] != a[5] && a[3] != a[6]
					&& a[3] != a[7] && a[3] != a[8]
					&& a[3] != a[9] && a[3] != a[10]
					&& a[3] != a[11] && a[3] != a[12]
					&& a[3] != a[13] && a[4] != a[5]
					&& a[4] != a[6] && a[4] != a[7]
					&& a[4] != a[8] && a[4] != a[9]
					&& a[4] != a[10] && a[4] != a[11]
					&& a[4] != a[12] && a[4] != a[13]
					&& a[5] != a[6] && a[5] != a[7]
					&& a[5] != a[8] && a[5] != a[9]
					&& a[5] != a[10] && a[5] != a[11]
					&& a[5] != a[12] && a[5] != a[13]
					&& a[6] != a[7] && a[6] != a[8]
					&& a[6] != a[9] && a[6] != a[10]
					&& a[6] != a[11] && a[6] != a[12]
					&& a[6] != a[13] && a[7] != a[8]
					&& a[7] != a[9] && a[7] != a[10]
					&& a[7] != a[11] && a[7] != a[12]
					&& a[7] != a[13] && a[8] != a[9]
					&& a[8] != a[10] && a[8] != a[11]
					&& a[8] != a[12] && a[8] != a[13]
					&& a[9] != a[10] && a[9] != a[11]
					&& a[9] != a[12] && a[9] != a[13]
					&& a[10] != a[11] && a[10] != a[12]
					&& a[10] != a[13] && a[11] != a[12]
					&& a[11] != a[13] && a[12] != a[13]
			})
			.unwrap()
	}

	pub fn find_consecutive_unique<const N: usize>(&self) -> usize {
		let mut chars_with_match = 0;
		let mut chars_count = [0; 26];

		for &c in self.file.iter().take(N - 1) {
			let indexed = index_char_mut(&mut chars_count, c);
			if *indexed > 0 {
				chars_with_match += 1;
			}
			*indexed += 1;
		}

		self.file
			.array_windows::<N>()
			.position(|window| {
				let indexed = index_char_mut(&mut chars_count, window[N - 1]);
				if *indexed > 0 {
					chars_with_match += 1;
				}
				*indexed += 1;

				if chars_with_match != 0 {
					let indexed = index_char_mut(&mut chars_count, window[0]);
					if *indexed > 1 {
						chars_with_match -= 1;
					}
					*indexed -= 1;

					false
				} else {
					true
				}
			})
			.unwrap()
	}
}

fn index_char_mut(s: &mut [u8; 26], index: u8) -> &mut u8 {
	&mut s[(index - b'a') as usize]
}
