#![allow(unused)]

use crate::helpers::*;

pub type A1 = i32;
pub type A2 = i32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
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

		let mut scan = HashSet::new();
		let mut mins = [A1::MAX; 3];
		let mut maxs = [A1::MIN; 3];

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
			for ((a, min), max) in point.into_iter().zip(&mut mins).zip(&mut maxs) {
				if a < *min {
					*min = a;
				} else if a > *max {
					*max = a;
				}
			}
		}

		for p in &mut maxs {
			*p += 1;
		}

		// let mut shrinkwrap = HashSet::new();

		// for [point_idx, line_idx, plane_idx] in [[0, 1, 2], [0, 2, 1], [1, 2, 0]] {
		// 	for plane in [mins[plane_idx], maxs[plane_idx] - 1] {
		// 		for line in mins[line_idx]..maxs[line_idx] {
		// 			for point in mins[point_idx]..maxs[point_idx] {
		// 				let mut item = [0; 3];
		// 				item[point_idx] = point;
		// 				item[line_idx] = line;
		// 				item[plane_idx] = plane;
		// 				shrinkwrap.insert(item);
		// 			}
		// 		}
		// 	}
		// }

		// let dimensions = [
		// 	(mins[0]..maxs[0]).len(),
		// 	(mins[1]..maxs[1]).len(),
		// 	(mins[2]..maxs[2]).len(),
		// ];
		// let mut exposed_faces = (dimensions[0] * dimensions[1]
		// 	+ dimensions[0] * dimensions[2]
		// 	+ dimensions[1] * dimensions[2])
		// 	* 2;

		// let mut queue: VecDeque<_> = shrinkwrap.iter().copied().collect();
		// while let Some(point) = queue.pop_front() {

		// }

		let starting_face = scan.iter().copied().find(|&[z, ..]| z == mins[0]).unwrap();
		let starting_face = (starting_face, 0);
		let mut queue: Vec<_> = [starting_face].into_iter().collect();
		let mut seen_faces = HashSet::new();

		while let Some(face) = queue.pop() {
			if !seen_faces.insert(face) {
				continue;
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
