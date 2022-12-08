use crate::helpers::*;

type A1 = i32;
type A2 = i32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

const SEEN: u8 = 0b1111_0000;
const SEEN_NORTH: u8 = 0b1000_0000;
const SEEN_SOUTH: u8 = 0b0100_0000;
const SEEN_WEST: u8 = 0b0010_0000;
const SEEN_EAST: u8 = 0b0001_0000;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(mut file: Vec<u8>) -> Self {
		// First go through and unset the 4 MSB of each tree. This does nothing to newlines.
		for c in &mut file {
			*c &= !SEEN;
		}
		// Figure out the dimensions
		let width = file.iter().position(|&b| b == b'\n').unwrap();
		let height = file.len() / (width + 1);

		// Check NS visibility
		for x in 0..width {
			insert_seen_data_x(width, x, &mut file, 0..height, SEEN_NORTH);
			insert_seen_data_x(width, x, &mut file, (0..height).rev(), SEEN_SOUTH);
		}

		// Check EW visibility
		for y in 0..height {
			insert_seen_data_y(width, y, &mut file, 0..width, SEEN_WEST);
			insert_seen_data_y(width, y, &mut file, (0..width).rev(), SEEN_EAST);
		}

		// Count trees marked visible. We skip the first and last row since those are guaranteed
		// visible, and newlines also haven't been marked (they're still newlines since they have
		// zeros in the 4 MSBs).
		let visible_from_edge = file[width + 1..file.len() - width - 1]
			.iter()
			.fold(0, |acc, &c| acc + (c & SEEN != 0) as usize)
			+ width * 2;

		let scenic_scores = vec![0u32; width * height];

		// Accumulate NS scenic scores
		for x in 0..width {
			for y in 0..height {
				todo!()
			}
		}

		let best_scenic_score = 0;

		// let input = file.trim_ascii_end().grid(|b| b - b'0');

		// for (y, row) in input.iter().enumerate() {
		// 	for (x, &tree) in row.iter().enumerate() {
		// 		let north_trees = input.iter().take(y).rev().map(|row| row[x]).collect_vec();
		// 		let south_trees = input.iter().skip(y + 1).map(|row| row[x]).collect_vec();
		// 		let west_trees = row.iter().take(x).rev().copied().collect_vec();
		// 		let east_trees = row.iter().skip(x + 1).copied().collect_vec();
		// 		// println!("{north_trees:?}");
		// 		// println!("{south_trees:?}");
		// 		// println!("{west_trees:?}");
		// 		// println!("{east_trees:?}");
		// 		'a: for range in [&north_trees, &south_trees, &west_trees, &east_trees] {
		// 			for &obstruction in range {
		// 				if obstruction >= tree {
		// 					continue 'a;
		// 				}
		// 			}
		// 			visible_from_edge += 1;
		// 			break;
		// 		}
		// 		let mut scenic_score = 1;
		// 		for range in [&north_trees, &south_trees, &west_trees, &east_trees] {
		// 			let mut visible_from_tree = 0;
		// 			for (i, &obstruction) in range.iter().enumerate() {
		// 				if obstruction >= tree {
		// 					visible_from_tree = i as i32 + 1;
		// 					break;
		// 				}
		// 			}
		// 			if visible_from_tree == 0 {
		// 				scenic_score *= range.len() as i32;
		// 			} else {
		// 				scenic_score *= visible_from_tree;
		// 			}
		// 		}
		// 		best_scenic_score = best_scenic_score.max(scenic_score);
		// 	}
		// }

		Self {
			p1: visible_from_edge as _,
			p2: best_scenic_score,
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

fn insert_seen_data_x<I>(width: usize, x: usize, file: &mut [u8], mut iter: I, mask: u8)
where
	I: Iterator<Item = usize>,
{
	let c = index_mut(width, iter.next().unwrap(), x, file);
	let mut highest_seen = char_as_height(*c);
	*c |= mask;
	for y in iter {
		update_tree(width, y, x, file, &mut highest_seen, mask);
	}
}

fn insert_seen_data_y<I>(width: usize, y: usize, file: &mut [u8], mut iter: I, mask: u8)
where
	I: Iterator<Item = usize>,
{
	let c = index_mut(width, y, iter.next().unwrap(), file);
	let mut highest_seen = char_as_height(*c);
	*c |= mask;
	for x in iter {
		update_tree(width, y, x, file, &mut highest_seen, mask);
	}
}

fn update_tree(width: usize, y: usize, x: usize, file: &mut [u8], highest_seen: &mut u8, mask: u8) {
	debug_assert!(*highest_seen < 10);
	let tree = index_mut(width, y, x, file);
	if char_as_height(*tree) > *highest_seen {
		*highest_seen = char_as_height(*tree);
		*tree |= mask;
	}
}

fn char_as_height(c: u8) -> u8 {
	c & !SEEN
}

fn index_mut(width: usize, y: usize, x: usize, s: &mut [u8]) -> &mut u8 {
	debug_assert!(x < width);
	&mut s[(width + 1) * y + x]
}
