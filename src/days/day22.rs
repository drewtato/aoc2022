use std::fmt::Display;

use crate::helpers::*;

type A1 = i64;
type A2 = A1;

#[derive(Debug)]
pub struct Solution {
	map: Vec<Vec<Tile>>,
	instructions: Vec<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut split = file.trim_ascii_end().lines();
		let instructions = split.next_back().unwrap().to_vec();
		split.next_back().unwrap();

		let map = split
			.map(|line| {
				line.iter()
					.map(|c| match c {
						b' ' => Empty,
						b'.' => Open,
						b'#' => Wall,
						_ => panic!("Unknown character"),
					})
					.collect_vec()
			})
			.collect_vec();

		Self { map, instructions }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		// println!("{}", TileMap(&map));
		let width = self.map.iter().map(|row| row.len()).max().unwrap();

		let wrap_rows = self
			.map
			.iter()
			.enumerate()
			.map(|(y, row)| {
				// println!("{}", TileMap(&vec![row.clone()]));

				let mut row = row.iter().enumerate().skip_while(|&(_, &c)| c == Empty);

				let (start, _) = row.next().unwrap();
				let (end, _) = row.take_while(|&(_, &c)| c != Empty).last().unwrap();
				let start = [y as isize, start as isize];
				let end = [y as isize, end as isize];
				[(end, Left), (start, Right)]
			})
			.collect_vec();

		let wrap_cols = (0..width)
			.map(|x| {
				let mut col = self
					.map
					.iter()
					.map(move |row| *row.get(x).unwrap_or(&Empty))
					.enumerate()
					.skip_while(|&(_, c)| c == Empty);

				let (start, _) = col.next().unwrap();
				let (end, _) = col.take_while(|&(_, c)| c != Empty).last().unwrap();
				let start = [start as isize, x as isize];
				let end = [end as isize, x as isize];
				[(end, Up), (start, Down)]
			})
			.collect_vec();

		// dbg_small!(&first_last_rows);
		// dbg_small!(&first_last_cols);

		traverse_map(&self.instructions, &self.map, &wrap_rows, &wrap_cols)
	}

	fn part_two(&mut self, dbg: u8) -> Self::AnswerTwo {
		let (wrap_rows, wrap_cols) = match dbg {
			0 => generate_wraps_main(),
			1 => generate_wraps_test(),
			_ => panic!("No debug impl >1"),
		};

		traverse_map(&self.instructions, &self.map, &wrap_rows, &wrap_cols)
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

type WrapItem = [([isize; 2], Direction); 2];

fn generate_wraps_main() -> (Vec<WrapItem>, Vec<WrapItem>) {
	let side = 50;
	let rows = (0..side)
		.map(|n| {
			let start = ([side * 3 - 1 - n, 0], Right);
			let end = ([side * 3 - 1 - n, side * 2 - 1], Left);
			[start, end]
		})
		.chain((0..side).map(|n| {
			let start = ([side * 2, n], Down);
			let end = ([side - 1, side * 2 + n], Up);
			[start, end]
		}))
		.chain((0..side).map(|n| {
			let start = ([side - 1 - n, side], Right);
			let end = ([side - 1 - n, side * 3 - 1], Left);
			[start, end]
		}))
		.chain((0..side).map(|n| {
			let start = ([0, side + n], Down);
			let end = ([side * 3 - 1, side + n], Up);
			[start, end]
		}))
		.collect_vec();
	let cols = (0..side)
		.map(|n| {
			let start = ([side + n, side], Right);
			let end = ([0, side * 2 + n], Down);
			[start, end]
		})
		.chain((0..side).map(|n| {
			let start = ([side * 3 + n, 0], Right);
			let end = ([side * 3 + n, side - 1], Left);
			[start, end]
		}))
		.chain((0..side).map(|n| {
			let start = ([side * 4 - 1, n], Up);
			let end = ([side + n, side * 2 - 1], Left);
			[start, end]
		}))
		.collect_vec();
	(rows, cols)
}

fn generate_wraps_test() -> (Vec<WrapItem>, Vec<WrapItem>) {
	let side = 4;
	let rows = (0..side)
		.map(|n| {
			let start = ([side, n + side], Down);
			let end = ([side * 3 - n - 1, side * 4 - 1], Left);
			[start, end]
		})
		.chain((0..side).map(|n| {
			let start = ([side * 3 - 1, side * 4 - 1 - n], Up);
			let end = ([side * 2, side * 4 - 1 - n], Down);
			[start, end]
		}))
		.chain((0..side).map(|n| {
			let start = ([side * 2 - 1, side * 2 - 1 - n], Up);
			let end = ([n, side * 3 - 1], Left);
			[start, end]
		}))
		.collect_vec();
	let cols = (0..side)
		.map(|n| {
			let start = ([0, side * 3 - 1 - n], Down);
			let end = ([side * 3 - 1, side * 3 - 1 - n], Up);
			[start, end]
		})
		.chain((0..side).map(|n| {
			let start = ([n, side * 2], Right);
			let end = ([side * 3 - 1 - n, side * 2], Right);
			[start, end]
		}))
		.chain((0..side).map(|n| {
			let start = ([side, side - 1 - n], Down);
			let end = ([side * 2 - 1, side - 1 - n], Up);
			[start, end]
		}))
		.chain((0..side).map(|n| {
			let start = ([side * 2 - 1 - n, side * 3 - 1], Left);
			let end = ([side * 2 - 1 - n, 0], Right);
			[start, end]
		}))
		.collect_vec();
	(rows, cols)
}

fn traverse_map(
	mut instructions: &[u8],
	map: &[Vec<Tile>],
	wrap_rows: &[WrapItem],
	wrap_cols: &[WrapItem],
) -> A1 {
	let mut cur_pos = [0, 0];
	while *get_2d(map, cur_pos).unwrap() != Open {
		cur_pos[1] += 1;
	}
	let mut dir = Right;

	loop {
		let (num, advance) = FromRadix10::from_radix_10(instructions);
		if advance == 0 {
			// println!("{}", last_line[0].to_display_byte());
			match instructions.take_first() {
				Some(b'R') => dir.right(),
				Some(b'L') => dir.left(),
				Some(_) => panic!("Unknown non-numeric"),
				None => break,
			}
		} else {
			// println!("{}", num);
			instructions = &instructions[advance..];

			for _ in 0usize..num {
				// println!("{:?} {:?}", cur_pos, dir);
				let next_pos = add(cur_pos, dir.offset());
				match get_2d(map, next_pos) {
					Some(&Open) => {
						cur_pos = next_pos;
						continue;
					}
					Some(&Wall) => break,
					Some(&Empty) | None => (),
				}

				let (next_pos, rotation) = match dir {
					Right => wrap_rows[next_pos[0] as usize][1],
					Up => wrap_cols[next_pos[1] as usize][0],
					Left => wrap_rows[next_pos[0] as usize][0],
					Down => wrap_cols[next_pos[1] as usize][1],
				};

				match get_2d(map, next_pos) {
					Some(&Open) => cur_pos = next_pos,
					Some(&Wall) => break,
					_ => panic!("Bad first_last position: {:?}", next_pos),
				}

				dir = rotation;
			}
		}
	}
	(cur_pos[0] as A1 + 1) * 1000 + (cur_pos[1] as A1 + 1) * 4 + dir.value()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TileMap<'a>(&'a Vec<Vec<Tile>>);

impl<'a> Display for TileMap<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in self.0 {
			for &c in row {
				let c = match c {
					Open => '.',
					Wall => '#',
					Empty => '_',
				};
				write!(f, "{c}")?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
	Open,
	Wall,
	Empty,
}
use Tile::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
	Right,
	Up,
	Left,
	Down,
}
use atoi::FromRadix10;
use Direction::*;

impl Direction {
	fn left(&mut self) {
		*self = match self {
			Right => Up,
			Up => Left,
			Left => Down,
			Down => Right,
		}
	}

	fn right(&mut self) {
		*self = match self {
			Right => Down,
			Up => Right,
			Left => Up,
			Down => Left,
		}
	}

	fn offset(self) -> [isize; 2] {
		match self {
			Right => [0, 1],
			Up => [-1, 0],
			Left => [0, -1],
			Down => [1, 0],
		}
	}

	fn value(self) -> A1 {
		match self {
			Right => 0,
			Down => 1,
			Left => 2,
			Up => 3,
		}
	}
}

fn add(a: [isize; 2], b: [isize; 2]) -> [isize; 2] {
	let [a1, a2] = a;
	let [b1, b2] = b;
	[a1 + b1, a2 + b2]
}

fn get_2d<T>(map: &[Vec<T>], point: [isize; 2]) -> Option<&T> {
	map.get(point[0] as usize)
		.and_then(|row| row.get(point[1] as usize))
}
