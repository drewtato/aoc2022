#![allow(unused)]

use std::ops::Deref;

use crate::helpers::*;

type A1 = i64;
type A2 = i64;

#[derive(Debug)]
pub struct Solution {
	input: Vec<u64>,
}

// const OFFSET: i32 = 1_000_000;
const DECRYPTION_KEY: i64 = 811_589_153;
const MASK: u64 = 0x00_00_00_00_ff_ff_ff_ff;
const SHIFT: u32 = 32;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut input: Vec<u64> = file
			.trim_ascii()
			.lines()
			.enumerate()
			.map(|(i, line)| {
				let n = line.parse::<i32>().unwrap() as u32 as u64;
				n | ((i as u64) << SHIFT)
			})
			.collect();

		Self { input }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut input = self.input.clone();

		mix_once::<false>(&mut input);

		let zero_pos = input.iter().position(|&d| d & MASK == 0).unwrap();

		let a = input[(1000 + zero_pos) % input.len()] as u32 as i32 as i64;
		let b = input[(2000 + zero_pos) % input.len()] as u32 as i32 as i64;
		let c = input[(3000 + zero_pos) % input.len()] as u32 as i32 as i64;

		a + b + c
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut input = self.input.clone();

		for _ in 0..10 {
			mix_once::<true>(&mut input);
		}

		let zero_pos = input.iter().position(|&d| d & MASK == 0).unwrap();

		let a = input[(1000 + zero_pos) % input.len()] as u32 as i32 as i64 * DECRYPTION_KEY;
		let b = input[(2000 + zero_pos) % input.len()] as u32 as i32 as i64 * DECRYPTION_KEY;
		let c = input[(3000 + zero_pos) % input.len()] as u32 as i32 as i64 * DECRYPTION_KEY;

		a + b + c
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

fn mix_once<const PART2: bool>(input: &mut Vec<u64>) {
	let len = input.len() - 1;

	for i in 0..(input.len() as u64) {
		let mut pos = input
			.iter()
			.position(|&d| {
				let index = d >> SHIFT;
				index == i
			})
			.unwrap();

		let real_n = (input[pos] & MASK) as u32 as i32 as i64;
		let n = real_n * if PART2 { DECRYPTION_KEY } else { 1 };
		let old = input.remove(pos);

		let index = (pos as i64 + n).rem_euclid(len as i64) as usize;
		input.insert(index, old);
	}
}
