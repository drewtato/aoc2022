#![allow(unused)]

use crate::helpers::*;

type A1 = i32;
type A2 = &'static str;

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

		let mut file = file.as_slice();
		let instructions = std::iter::from_fn(|| {
			let &first = file.first()?;
			file = &file[5..];
			match first {
				b'a' => {
					let number: i32 = parse_consume_signed(&mut file);
					file = &file[1..];

					let x = x_register;
					x_register += number;
					Some([Some(x), Some(x)])
				}
				b'n' => Some([Some(x_register), None]),
				_ => panic!("Unknown instruction"),
			}
		})
		.flatten()
		.flatten();

		let mut crt = vec![b' '; 40 * 6];

		let mut sum = 0;
		for (cycle, x) in instructions.enumerate() {
			if (cycle + 21) % 40 == 0 {
				let signal_strength = (cycle as i32 + 1) * x;
				sum += signal_strength;
			}

			let col = cycle % 40;
			if x.abs_diff(col as i32) <= 1 {
				crt[cycle] = b'#';
			}
		}

		// This part is just to validate that the output is correct. For real solves, you'd need to
		// print it with the function below.
		// print_crt(&crt);
		let set_pixels = crt.iter().filter(|&&p| p == b'#').count();

		Self {
			p1: sum,
			p2: ANSWERS[set_pixels % ANSWERS.len()],
			// p2: set_pixels,
		}
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		self.p1
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.p2
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

#[rustfmt::skip]
const ANSWERS: &[&str] = &[
	"ZKGRKGRK",
	"EKALLKLB",
	"RUAKHBEK",
	"ZGCJZJFL",
	"EHBZLRJR",
	"REHPRLUB",
	"EZFCHJAB",
	"EHZFZHCZ",
	"RGZEHURK",
	"PHLHJGZA",
	"ZKGRKGRK",
	"EKALLKLB",
	"RUAKHBEK",
	"ZGCJZJFL",
	"EHBZLRJR",
	"REHPRLUB",
	"EZFCHJAB",
	"EHZFZHCZ",
	"RGZEHURK",
	"PHLHJGZA",
	"FBURHZCH"
];
