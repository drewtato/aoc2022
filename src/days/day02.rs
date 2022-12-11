use crate::helpers::*;

type A1 = i32;
type A2 = i32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Shape {
	Rock = 1,
	Paper = 2,
	Scissors = 3,
}
use Shape::*;

const LOSE: Shape = Rock;
const DRAW: Shape = Paper;
const WIN: Shape = Scissors;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut total1 = 0;
		let mut total2 = 0;

		file.trim_ascii().consume_lines(|line| {
			let a = match line[0] {
				b'A' => Rock,
				b'B' => Paper,
				b'C' => Scissors,
				_ => panic!(),
			};
			let b = match line[2] {
				b'X' => Rock,
				b'Y' => Paper,
				b'Z' => Scissors,
				_ => panic!(),
			};

			total1 += match (a, b) {
				(Rock, Scissors) => 0,
				(Paper, Rock) => 0,
				(Scissors, Paper) => 0,
				(Rock, Rock) => 3,
				(Paper, Paper) => 3,
				(Scissors, Scissors) => 3,
				(Rock, Paper) => 6,
				(Paper, Scissors) => 6,
				(Scissors, Rock) => 6,
			};
			total1 += b as i32;

			total2 += match (a, b) {
				(Rock, WIN) => Paper,
				(Paper, WIN) => Scissors,
				(Scissors, WIN) => Rock,
				(Rock, DRAW) => Rock,
				(Paper, DRAW) => Paper,
				(Scissors, DRAW) => Scissors,
				(Rock, LOSE) => Scissors,
				(Paper, LOSE) => Rock,
				(Scissors, LOSE) => Paper,
			} as i32;
			total2 += (b as i32 - 1) * 3;
			Err(4)
		});

		Self {
			p1: total1,
			p2: total2,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.p1
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.p2
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
