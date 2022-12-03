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
		let mut sum = 0;
		for line in file.trim_ascii().lines() {
			let len = line.len();
			let (first, second) = line.split_at(len / 2);
			let first: HashSet<u8> = first.iter().copied().collect();
			let second = second.iter().copied().collect();
			let &same = first.intersection(&second).next().unwrap();
			sum += priority(same);
		}

		let mut sum2 = 0;
		for [a, b, c] in file.trim_ascii().lines().array_chunks() {
			let a: HashSet<u8> = a.iter().copied().collect();
			let b: HashSet<u8> = b.iter().copied().collect();
			let c: HashSet<u8> = c.iter().copied().collect();
			let same = 'b: {
				for elem in a.intersection(&b) {
					if c.contains(elem) {
						break 'b *elem;
					}
				}
				panic!()
			};

			sum2 += priority(same);
		}

		Self { p1: sum, p2: sum2 }
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

fn priority(item: u8) -> i32 {
	(match item {
		b'a'..=b'z' => item - b'a' + 1,
		b'A'..=b'Z' => item - b'A' + 27,
		_ => panic!(),
	}) as i32
}
