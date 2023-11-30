#![allow(unused)]

use crate::helpers::*;

pub type A1 = usize;
pub type A2 = A1;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut grid = file.trim_ascii_end().grid(|c| match c {
			b'#' => Wall,
			b'>' => Blizzards([true, false, false, false]),
			b'^' => Blizzards([false, true, false, false]),
			b'<' => Blizzards([false, false, true, false]),
			b'v' => Blizzards([false, false, false, true]),
			b'.' => Blizzards([false; 4]),
			c => panic!("Unknown character {c:?}"),
		});

		let mut temp_grid = grid.clone();

		let starting_pos = [0, 1];
		let ending_pos = [grid.len() as isize - 1, grid[0].len() as isize - 2];

		let mut queue = vec![starting_pos];
		let mut temp_queue = HashSet::new();

		let mut minute = 0;

		let p1 = loop {
			// print_grid(&grid, &queue);
			// read_value::<String>().unwrap();

			minute += 1;
			move_blizzards(&mut grid, &mut temp_grid);
			if move_expedition(&mut queue, &mut temp_queue, &grid, ending_pos) {
				break minute;
			}
		};

		queue.clear();
		queue.push(ending_pos);
		temp_queue.clear();

		loop {
			minute += 1;
			move_blizzards(&mut grid, &mut temp_grid);
			if move_expedition(&mut queue, &mut temp_queue, &grid, starting_pos) {
				break;
			}
		}

		queue.clear();
		queue.push(starting_pos);
		temp_queue.clear();

		let p2 = loop {
			minute += 1;
			move_blizzards(&mut grid, &mut temp_grid);
			if move_expedition(&mut queue, &mut temp_queue, &grid, ending_pos) {
				break minute;
			}
		};

		Self { p1, p2 }
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

fn move_blizzards(grid: &mut Vec<Vec<Tile>>, temp_grid: &mut Vec<Vec<Tile>>) {
	let height = grid.len();
	let width = grid[0].len();
	for row in &mut temp_grid[1..height - 1] {
		for tile in &mut row[1..width - 1] {
			*tile = Blizzards([false; 4]);
		}
	}

	for (y, row) in grid.iter().enumerate().skip(1).take(height - 2) {
		for (x, &tile) in row.iter().enumerate().skip(1).take(width - 2) {
			// println!("{tile:?}");
			match tile {
				Wall => panic!("Wall in middle area"),
				Blizzards(b) => {
					for (i, b) in b.into_iter().enumerate() {
						if !b {
							continue;
						}

						let dir: Direction = i.into();
						let offset = dir.offset();
						let [ny, nx] = add([y as isize, x as isize], offset);
						let next_pos = match &mut temp_grid[ny as usize][nx as usize] {
							Wall => match dir {
								East => temp_grid.get_mut(y).unwrap().get_mut(1),
								North => temp_grid.get_mut(height - 2).unwrap().get_mut(x),
								West => temp_grid.get_mut(y).unwrap().get_mut(width - 2),
								South => temp_grid.get_mut(1).unwrap().get_mut(x),
							}
							.unwrap()
							.as_mut()
							.unwrap(),
							Blizzards(b) => b,
						};
						debug_assert!(
							!next_pos[i],
							"Location {:?} already has blizzard pointing {dir:?}",
							[ny, nx]
						);
						next_pos[i] = true;
					}
				}
			}
		}
	}

	swap(temp_grid, grid);
}

fn move_expedition(
	queue: &mut Vec<[isize; 2]>,
	temp_queue: &mut HashSet<[isize; 2]>,
	grid: &[Vec<Tile>],
	goal: [isize; 2],
) -> bool {
	for pos in queue.drain(..) {
		match get_2d(grid, pos) {
			Some(Blizzards([false, false, false, false])) => {
				temp_queue.insert(pos);
			}
			None | Some(Wall) | Some(Blizzards(_)) => (),
		}

		for n in [East, North, South, West] {
			let point = add(pos, n.offset());
			if point == goal {
				return true;
			}
			match get_2d(grid, point) {
				Some(Blizzards([false, false, false, false])) => {
					temp_queue.insert(point);
				}
				None | Some(Wall) | Some(Blizzards(_)) => (),
			}
		}
	}
	queue.extend(temp_queue.drain());
	false
}

use std::fmt::Write;
fn print_grid(grid: &[Vec<Tile>], expeditions: &[[isize; 2]]) {
	let mut v = Vec::new();
	for (y, row) in grid.iter().enumerate() {
		let mut s = String::new();
		for (x, &tile) in row.iter().enumerate() {
			match tile {
				Wall => write!(s, "#"),
				Blizzards([false, false, false, false]) => write!(s, "."),
				Blizzards(all) => write!(s, "{}", all.into_iter().filter(|&b| b).count()),
			}
			.unwrap()
		}
		v.push(s);
	}
	for &[y, x] in expeditions {
		let [y, x] = [y as usize, x as usize];
		v[y].replace_range(x..x + 1, "E");
	}
	for line in v {
		println!("{line}");
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
	Wall,
	Blizzards([bool; 4]),
}

impl Tile {
	fn as_mut(&mut self) -> Option<&mut [bool; 4]> {
		match self {
			Wall => None,
			Blizzards(b) => Some(b),
		}
	}
}

impl Default for Tile {
	fn default() -> Self {
		Self::Blizzards([false; 4])
	}
}

use Tile::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
	East,
	North,
	West,
	South,
}
use Direction::*;

impl Direction {
	fn offset(self) -> [isize; 2] {
		match self {
			East => [0, 1],
			North => [-1, 0],
			West => [0, -1],
			South => [1, 0],
		}
	}
}

impl From<usize> for Direction {
	fn from(value: usize) -> Self {
		match value {
			0 => East,
			1 => North,
			2 => West,
			3 => South,
			_ => panic!(),
		}
	}
}
