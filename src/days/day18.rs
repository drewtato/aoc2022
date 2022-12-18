#![allow(unused)]

use crate::helpers::*;

type A1 = i32;
type A2 = i32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut scan = HashSet::new();
		let mut faces = 0;

		let mut file = file.as_slice();

		// let lines = gen_iter(|| loop {
		// 	let n: A1 = parse_consume_signed(&mut file);
		// 	yield n;
		// 	match file.get(1..) {
		// 		Some(f) => file = f,
		// 		None => return,
		// 	}
		// });

		let lines = from_fn_iter(|| {
			let n = parse_consume_signed(&mut file);
			file.get(1..).map(|f| {
				file = f;
				n
			})
		});

		for point in lines.array_chunks() {
			scan.insert(point);
			for neighbor in ADJACENT {
				let neighbor = add(point, neighbor);

				// Branchless one is faster, not surprisingly
				// faces += if scan.contains(&neighbor) { -1 } else { 1 };
				// println!(
				// 	"{} => {}",
				// 	scan.contains(&neighbor) as i32,
				// 	-((scan.contains(&neighbor) as i32) * 2 - 1)
				// );
				faces += -(((scan.contains(&neighbor) as i32) << 1) - 1);
			}
		}

		Self { p1: faces, p2: 0 }
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

const ADJACENT: [[A1; 3]; 6] = [
	[0, 0, 1],
	[0, 0, -1],
	[0, 1, 0],
	[0, -1, 0],
	[1, 0, 0],
	[-1, 0, 0],
];

fn add(a: [A1; 3], b: [A1; 3]) -> [A1; 3] {
	let [a1, a2, a3] = a;
	let [b1, b2, b3] = b;
	[a1 + b1, a2 + b2, a3 + b3]
}
