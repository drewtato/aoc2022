#![allow(unused)]

use std::num::NonZeroU8;

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
		let mut p1: u32 = 0;
		let mut p2: u32 = 0;

		let p1_lines = file.trim_ascii().lines().map(|line| {
			let len = line.len();
			let (first, second) = line.split_at(len / 2);

			let mut first_map: u64 = 0;
			let mut second_map: u64 = 0;

			for &bit in first {
				first_map |= (1 << priority(bit));
			}

			for &bit in second {
				second_map |= (1 << priority(bit));
			}

			let common = first_map & second_map;
			let priority = common.ilog2();
			p1 += priority;
			first_map | second_map
		});

		for [a, b, c] in p1_lines.array_chunks() {
			let badge = a & b & c;
			let priority = badge.ilog2();
			p2 += priority;
		}

		Self { p1, p2 }
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

fn priority(item: u8) -> u8 {
	match item {
		b'a'..=b'z' => item - b'a' + 1,
		b'A'..=b'Z' => item - b'A' + 27,
		_ => panic!(),
	}
}
