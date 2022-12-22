#![allow(unused)]

use std::fmt::Display;

use crate::helpers::*;

type A1 = i64;
type A2 = A1;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut split = file.trim_ascii_end().lines();
		let mut last_line = split.next_back().unwrap();
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

		// println!("{}", TileMap(&map));
		let width = map.iter().map(|row| row.len()).max().unwrap();

		let first_last_rows = map
			.iter()
			.map(|row| {
				// println!("{}", TileMap(&vec![row.clone()]));

				let mut row = row.iter().enumerate().skip_while(|&(_, &c)| c == Empty);

				let (start, _) = row.next().unwrap();
				let (end, _) = row.take_while(|&(_, &c)| c != Empty).last().unwrap();
				[start, end]
			})
			.collect_vec();

		let first_last_cols = (0..width)
			.map(|x| {
				let mut col = map
					.iter()
					.map(move |row| *row.get(x).unwrap_or(&Empty))
					.enumerate()
					.skip_while(|&(_, c)| c == Empty);

				let (start, _) = col.next().unwrap();
				let (end, _) = col.take_while(|&(_, c)| c != Empty).last().unwrap();
				[start, end]
			})
			.collect_vec();

		// dbg_small!(&first_last_rows);
		// dbg_small!(&first_last_cols);

		let mut cur_pos = [0, 0];
		while *get_2d(&map, cur_pos).unwrap() != Open {
			cur_pos[1] += 1;
		}

		let mut dir = Right;

		loop {
			let (num, advance) = FromRadix10::from_radix_10(last_line);
			if advance == 0 {
				// println!("{}", last_line[0].to_display_byte());
				match last_line.take_first() {
					Some(b'R') => dir.right(),
					Some(b'L') => dir.left(),
					Some(_) => panic!("Unknown non-numeric"),
					None => break,
				}
			} else {
				// println!("{}", num);
				last_line = &last_line[advance..];
				let diff = dir.offset();
				for _ in 0usize..num {
					// println!("{:?}", cur_pos);
					let next_pos = add(cur_pos, diff);
					match get_2d(&map, next_pos) {
						Some(&Open) => {
							cur_pos = next_pos;
							continue;
						}
						Some(&Wall) => break,
						Some(&Empty) | None => (),
					}

					let next_pos = match dir {
						Right => [
							next_pos[0],
							first_last_rows[next_pos[0] as usize][0] as isize,
						],
						Up => [
							first_last_cols[next_pos[1] as usize][1] as isize,
							next_pos[1],
						],
						Left => [
							next_pos[0],
							first_last_rows[next_pos[0] as usize][1] as isize,
						],
						Down => [
							first_last_cols[next_pos[1] as usize][0] as isize,
							next_pos[1],
						],
					};

					match get_2d(&map, next_pos) {
						Some(&Open) => cur_pos = next_pos,
						Some(&Wall) => break,
						_ => panic!("Bad first_last position"),
					}
				}
			}
		}

		let p1 = (cur_pos[0] as A1 + 1) * 1000 + (cur_pos[1] as A1 + 1) * 4 + dir.value();

		Self { p1, p2: 0 }
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
			writeln!(f);
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
			Up => 1,
			Left => 2,
			Down => 3,
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
