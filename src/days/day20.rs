#![allow(unused)]

use std::ops::Deref;

use crate::helpers::*;

type A1 = i64;
type A2 = i64;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

// const OFFSET: i32 = 1_000_000;
const OFFSET: i64 = 811_589_153;
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

		mix_once(&mut input);

		let zero_pos = input.iter().position(|&d| d & MASK == 0).unwrap();

		let a = input[(1000 + zero_pos) % input.len()] as u32 as i32 as i64;
		let b = input[(2000 + zero_pos) % input.len()] as u32 as i32 as i64;
		let c = input[(3000 + zero_pos) % input.len()] as u32 as i32 as i64;
		dbg_small!(a, b, c);
		for &n in &input {
			print!("{}, ", (n & MASK) as u32 as i32);
		}
		println!();

		for _ in 0..9 {
			mix_once(&mut input);
		}

		let zero_pos = input.iter().position(|&d| d & MASK == 0).unwrap();

		let a2 = input[(1000 + zero_pos) % input.len()] as u32 as i32 as i64 * OFFSET;
		let b2 = input[(2000 + zero_pos) % input.len()] as u32 as i32 as i64 * OFFSET;
		let c2 = input[(3000 + zero_pos) % input.len()] as u32 as i32 as i64 * OFFSET;

		// dbg_small!(a, b, c);

		Self {
			p1: a + b + c,
			p2: a2 + b2 + c2,
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

fn mix_once(input: &mut Vec<u64>) {
	let len = input.len() - 1;

	for i in 0..(input.len() as u64) {
		for &n in &*input {
			print!("{}, ", (n & MASK) as u32 as i32);
		}
		println!();
		// dbg_small!(&input, i);
		let mut pos = input
			.iter()
			.position(|&d| {
				let index = d >> SHIFT;
				index == i
			})
			.unwrap();
		dbg!(pos);
		let real_n = (input[pos] & MASK) as u32 as i32 as i64;
		let n = real_n * OFFSET;
		let old = input.remove(pos);

		let index = (pos as i64 + n).rem_euclid(len as i64) as usize;
		input.insert(index, old);
	}
}
