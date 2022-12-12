use crate::helpers::*;

type A1 = u16;
type A2 = A1;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(mut file: Vec<u8>, _: u8) -> Self {
		let width = file.iter().position(|&b| b == b'\n').unwrap();
		file.retain(|&b| b != b'\n');

		let height = file.len() / width;
		let start = file
			.iter_mut()
			.position(|b| {
				if *b == b'S' {
					*b = b'a';
					true
				} else {
					false
				}
			})
			.unwrap();

		let end = file
			.iter_mut()
			.position(|b| {
				if *b == b'E' {
					*b = b'z';
					true
				} else {
					false
				}
			})
			.unwrap();

		let start = [start / width, start % width];
		let end = [end / width, end % width];

		// println!("{width} {height} {start:?} {end:?}");

		let mut visited = vec![false; width * height];

		let mut leads = VecDeque::with_capacity(1000);
		leads.push_back((0, end));

		let mut p2_cost = None;
		let cost = loop {
			let Some((cost, [y, x])) = leads.pop_front() else { panic!("No path found") };

			let v = &mut visited[width * y + x];
			if *v {
				continue;
			} else {
				*v = true;
			}

			let current_height = file[width * y + x];
			if p2_cost.is_none() && current_height == b'a' {
				p2_cost = Some(cost);
			}

			for [dy, dx] in [[-1, 0], [1, 0], [0, 1], [0, -1]] {
				let Ok(ny) = (y as isize + dy).try_into() else { continue; };
				let Ok(nx) = (x as isize + dx).try_into() else { continue; };
				let Some(&neighbor_height) = file.get(width * ny + nx) else { continue; };
				if current_height <= neighbor_height + 1 {
					leads.push_back((cost + 1, [ny, nx]));
				}
			}

			if [y, x] == start {
				break cost;
			}
		};

		Self {
			p1: cost,
			p2: p2_cost.unwrap(),
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
