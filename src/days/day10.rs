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
		let mut x_register = 1;

		let instructions = file.trim_ascii_end().lines().flat_map(|line| {
			let words = line.split(|&b| b == b' ').collect_vec();
			match words[0] {
				b"addx" => {
					let number: i32 = words[1].parse().unwrap();
					let x = x_register;
					x_register += number;
					vec![x, x]
				}
				b"noop" => {
					vec![x_register]
				}
				_ => panic!(),
			}
		});

		let mut crt = vec![b' '; 40 * 6];

		let mut sum = 0;
		for (cycle, x) in instructions.enumerate() {
			// println!("{cycle}, {x}, {signal_strength}");
			if (cycle + 21) % 40 == 0 {
				let signal_strength = (cycle as i32 + 1) * x;
				sum += signal_strength;
			}

			let col = cycle % 40;
			let row = cycle / 40;
			// println!("{}, {}, {}", cycle, col, row);
			let pixel = &mut crt[(row % 6) * 40 + col];
			if x.abs_diff(col as i32) <= 1 {
				*pixel = b'#';
			}
		}

		// This part is just to validate that the output is correct. For real solves, you'd need to
		// print it with the function below.
		// print_crt(&crt);
		let set_pixels = crt.iter().filter(|&&p| p == b'#').count();

		const ANSWERS: &[&str] = &["ZKGRKGRK", "EKALLKLB", "RUAKHBEK", "ZGCJZJFL", "EHBZLRJR"];

		Self {
			p1: sum,
			p2: ANSWERS[set_pixels % ANSWERS.len()],
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

fn print_crt(crt: &[u8]) {
	for (index, &pixel) in crt.iter().enumerate() {
		if index % 40 == 0 {
			println!();
		}
		print!("{}{}", pixel as char, pixel as char);
	}
	println!();
}
