use crate::helpers::*;

type A1 = u32;
type A2 = u32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut input: HashSet<[u16; 2]> = file
			.trim_ascii()
			.lines()
			.flat_map(|line| {
				let mut v = Vec::new();
				let windows = std::str::from_utf8(line)
					.unwrap()
					.split(" -> ")
					.map(|part| -> [u16; 2] {
						part.split(',')
							.map(|n| n.parse().unwrap())
							.array_chunks()
							.next()
							.unwrap()
					})
					.tuple_windows();

				for ([y1, x1], [y2, x2]) in windows {
					if y1 == y2 {
						let [&x1, &x2] = [x1, x2].iter().sorted().array_chunks().next().unwrap();
						for n in x1..=x2 {
							v.push([y1, n]);
						}
					} else if x1 == x2 {
						let [&y1, &y2] = [y1, y2].iter().sorted().array_chunks().next().unwrap();
						for n in y1..=y2 {
							v.push([n, x1]);
						}
					}
				}
				v
			})
			.collect();

		let sand_start = [500, 0];
		let mut sand_count = 0;
		let bottom_edge = input.iter().map(|&[_, y]| y).max().unwrap() + 1;
		let mut p1 = None;

		loop {
			let mut current = sand_start;
			loop {
				if current[1] == bottom_edge {
					if p1.is_none() {
						p1 = Some(sand_count);
					}
					break;
				}
				if !input.contains(&[current[0], current[1] + 1]) {
					current[1] += 1;
				} else if !input.contains(&[current[0] - 1, current[1] + 1]) {
					current[0] -= 1;
					current[1] += 1;
				} else if !input.contains(&[current[0] + 1, current[1] + 1]) {
					current[0] += 1;
					current[1] += 1;
				} else {
					// Sand has stopped
					break;
				}
			}
			sand_count += 1;
			if current == sand_start {
				break;
			}
			input.insert(current);
		}

		Self {
			p1: p1.unwrap(),
			p2: sand_count,
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
