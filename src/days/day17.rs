#![allow(unused)]

use std::convert::identity;

use crate::helpers::*;

pub type A1 = usize;
pub type A2 = u64;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let file = file.trim_ascii_end();

		let mut playfield: VecDeque<[bool; 7]> = VecDeque::new();
		let mut rocks_placed = 0;
		let mut current_pos = [3, 2];
		let mut height = 0;
		let mut height_adjustment = 0;

		let mut jet_cycle = file.iter().cycle();
		let mut jet_count = 0;

		for &c in &mut jet_cycle {
			if rocks_placed == PART_ONE_COUNT {
				break;
			}
			jet_count += 1;

			run_step(
				&mut playfield,
				&mut height,
				&mut rocks_placed,
				c,
				&mut current_pos,
				&mut height_adjustment,
			);
			// print_playfield(&playfield);
			// read_value::<String>().unwrap();
		}

		let p1 = height + height_adjustment;
		let p1_jet_count = jet_count;

		// for &c in &mut jet_cycle {
		// 	jet_count += 1;

		// 	run_step(
		// 		&mut playfield,
		// 		&mut height,
		// 		&mut rocks_placed,
		// 		c,
		// 		&mut current_pos,
		// 		&mut height_adjustment,
		// 	);

		// 	if height % 10_000_000 <= 2 {
		// 		println!("{height}");
		// 		if dbg!(jet_count % file.len() == p1_jet_count % file.len())
		// 			&& dbg!(rocks_placed as usize % ROCKS.len() == PART_ONE_COUNT as usize)
		// 		{
		// 			break;
		// 		}
		// 	} else if (jet_count % file.len() == p1_jet_count % file.len())
		// 		&& (rocks_placed as usize % ROCKS.len() == PART_ONE_COUNT as usize)
		// 	{
		// 		break;
		// 	}
		// }

		// let rocks_in_cycle = rocks_placed - PART_ONE_COUNT;
		// rocks_placed += ALL_ROCKS / rocks_in_cycle * rocks_in_cycle;
		// let height_added_in_cycle = height - p1;
		// let height_added_in_all_cycles = ALL_ROCKS / rocks_in_cycle * height_added_in_cycle as
		// u64;

		// for &c in &mut jet_cycle {
		// 	if rocks_placed == ALL_ROCKS {
		// 		break;
		// 	}
		// 	run_step(
		// 		&mut playfield,
		// 		&mut height,
		// 		&mut rocks_placed,
		// 		c,
		// 		&mut current_pos,
		// 		&mut height_adjustment,
		// 	);
		// }

		Self {
			p1,
			// p2: height as u64 + height_added_in_all_cycles,
			p2: 0,
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

fn run_step(
	playfield: &mut VecDeque<[bool; 7]>,
	height: &mut usize,
	rocks_placed: &mut u64,
	c: u8,
	current_pos: &mut [usize; 2],
	_height_adjustment: &mut usize,
) {
	playfield.resize(*height + 7, [false; 7]);
	let current_rock = ROCKS[(*rocks_placed % ROCKS.len() as u64) as usize];
	// Test if jet moves the rock
	let new_pos = match c {
		LEFT => sub(*current_pos, [0, 1]),
		RIGHT => add(*current_pos, [0, 1]),
		_ => panic!("Unknown character"),
	};
	let movable = current_rock.iter().all(|&point| {
		let new_point = add(point, new_pos);
		matches!(get_2d(playfield, new_point), Some(&false))
	});
	if movable {
		*current_pos = new_pos;
	}
	// Test if gravity moves the rock
	let new_pos = sub(*current_pos, [1, 0]);
	let movable = current_rock.iter().all(|&point| {
		let new_point = add(point, new_pos);
		matches!(get_2d(playfield, new_point), Some(&false))
	});
	if movable {
		*current_pos = new_pos;
	} else {
		for &point in current_rock {
			let new_point = add(point, *current_pos);
			*get_2d_mut(playfield, new_point).unwrap() = true;
		}

		*rocks_placed += 1;

		// let height_of_full_line = playfield
		// 	.iter()
		// 	.enumerate()
		// 	.rev()
		// 	.find(|&(_, row)| row.iter().all(|&point| point))
		// 	.map(|(i, _)| i + 1)
		// 	// .into_iter()
		// 	// .inspect(|_| print_playfield(playfield))
		// 	// .next()
		// 	.unwrap_or_default();

		// *height_adjustment += height_of_full_line;

		// playfield.drain(..=height_of_full_line);

		*height = playfield
			.iter()
			.enumerate()
			.rev()
			.find(|&(_, row)| row.iter().any(|&point| point))
			.map(|(i, _)| i + 1)
			.unwrap_or(playfield.len() - 3);

		*current_pos = [*height + 3, 2];
	}
}

fn print_playfield(playfield: &VecDeque<[bool; 7]>) {
	for (i, row) in playfield.iter().enumerate().rev() {
		print!("{i:04} |");
		for &p in row {
			if p {
				print!("[]");
			} else {
				print!("..");
			}
		}
		println!("|");
	}
	println!("     +--------------+")
}

fn add(p1: [A1; 2], p2: [A1; 2]) -> [A1; 2] {
	[p1[0].wrapping_add(p2[0]), p1[1].wrapping_add(p2[1])]
}

fn sub(p1: [A1; 2], p2: [A1; 2]) -> [A1; 2] {
	[p1[0].wrapping_sub(p2[0]), p1[1].wrapping_sub(p2[1])]
}

fn get_2d<const N: usize, T>(grid: &VecDeque<[T; N]>, point: [A1; 2]) -> Option<&T> {
	grid.get(point[0]).and_then(|row| row.get(point[1]))
}

fn get_2d_mut<const N: usize, T>(grid: &mut VecDeque<[T; N]>, point: [A1; 2]) -> Option<&mut T> {
	grid.get_mut(point[0]).and_then(|row| row.get_mut(point[1]))
}

const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';

const ALL_ROCKS: u64 = 1_000_000_000_000;

const PART_ONE_COUNT: u64 = 2022;

use rocks::*;
#[rustfmt::skip]
mod rocks {
    use super::A1;

	const ROCK_ONE: &[[A1; 2]] = &[
		[0,0],
		[0,1],
		[0,2],
		[0,3],
	];

	const ROCK_TWO: &[[A1; 2]] = &[
		[1,1],
		[1,0],
		[0,1],
		[2,1],
		[1,2],
	];

	const ROCK_THREE: &[[A1; 2]] = &[
		[0,0],
		[0,1],
		[0,2],
		[1,2],
		[2,2],
	];

	const ROCK_FOUR: &[[A1; 2]] = &[
		[0,0],
		[1,0],
		[2,0],
		[3,0],
	];

	const ROCK_FIVE: &[[A1; 2]] = &[
		[0,0],
		[1,0],
		[0,1],
		[1,1],
	];

	pub const ROCKS: &[&[[A1; 2]]] = &[
		ROCK_ONE,
		ROCK_TWO,
		ROCK_THREE,
		ROCK_FOUR,
		ROCK_FIVE
	];
}
