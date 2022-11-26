#![allow(unused)]

use crate::{helpers::*, Grid, InputData, Res, Solver};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Solution {
	input: Vec<i32>,
}

impl Solver for Solution {
	type AnswerOne = u32;
	type AnswerTwo = u32;

	fn initialize(file: Vec<u8>) -> Self {
		Self {
			input: file.lines().filter_map(|l| l.parse()).collect(),
		}
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		let mut count = 0;
		for &[first, second] in self.input.array_windows() {
			if first < second {
				count += 1;
			}
		}
		count
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		let mut count = 0;
		for &[first, _, _, last] in self.input.array_windows() {
			if first < last {
				count += 1;
			}
		}
		count
	}

	fn run_any(&mut self, part: u32) -> Result<String, ()> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(()),
		}
	}
}
