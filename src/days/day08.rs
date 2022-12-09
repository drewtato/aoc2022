use crate::helpers::*;

type A1 = i32;
type A2 = i32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

const SEEN: u8 = 0b1000_0000;
const MASK: u8 = 0b0000_1111;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(mut file: Vec<u8>) -> Self {
		// Figure out the dimensions
		let width = file.iter().position(|&b| b == b'\n').unwrap();
		let height = file.len() / (width + 1);

		let mut visible_from_edge = 0;
		let mut best_scenic_score = 0;

		let mut scenic_scores = vec![1i32; width * height];

		for y in 0..height {
			// Index of last seen tree at each height, starting at the first tree
			let mut seen_trees = [0; 10];
			// From left to right
			for x in 0..width {
				// Distance to edge
				let distance = x;
				process_cell(
					&mut file,
					&mut seen_trees,
					&mut scenic_scores,
					width,
					y,
					x,
					distance as i32,
				);
			}

			// Index of last seen tree at each height, starting at the first tree
			let mut seen_trees = [0; 10];
			// From left to right
			for x in (0..width).rev() {
				// Distance to edge
				let distance = width - x - 1;
				process_cell(
					&mut file,
					&mut seen_trees,
					&mut scenic_scores,
					width,
					y,
					x,
					distance as i32,
				);
			}
		}

		for x in 0..width {
			// Index of last seen tree at each height, starting at the first tree
			let mut seen_trees = [0; 10];
			// From top to bottom
			for y in 0..height {
				// Distance to edge
				let distance = y;
				process_cell(
					&mut file,
					&mut seen_trees,
					&mut scenic_scores,
					width,
					y,
					x,
					distance as i32,
				);
			}

			// Index of last seen tree at each height, starting at the first tree
			let mut seen_trees = [0; 10];
			// From left to right
			for y in (0..height).rev() {
				// Distance to edge
				let distance = (height - y - 1) as i32;
				// Get cell at coordinates
				let cell = index_mut(&mut file, width, y, x);
				// Get tree height in cell
				let tree = cell_to_height(*cell);
				// Get index of last seen tree
				let last_seen = seen_trees[tree as usize];
				// Check if this tree is visible from the edge
				let direction_score = if last_seen == 0 {
					// Add SEEN bit
					*cell |= SEEN;
					// Multiply scenic score by distance
					distance
				} else {
					// Multiply scenic score by distance from last tree, including that tree
					distance - last_seen + 1
				};
				// No need to store it since this is the last run
				let total_score = scenic_scores[width * y + x] * direction_score;
				// Check if this is the best score
				best_scenic_score = best_scenic_score.max(total_score);

				// If the SEEN bit is set, add to visible counter
				if *cell & SEEN != 0 {
					visible_from_edge += 1;
				}
				// Update this and all shorter last seens to the distance index plus one
				seen_trees[..=tree as usize].fill(distance + 1);
			}
		}

		Self {
			p1: visible_from_edge,
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

fn process_cell(
	file: &mut [u8],
	seen_trees: &mut [i32; 10],
	scenic_scores: &mut [i32],
	width: usize,
	y: usize,
	x: usize,
	distance: i32,
) {
	// Get cell at coordinates
	let cell = index_mut(file, width, y, x);
	// Get tree height in cell
	let tree = cell_to_height(*cell);
	// Get index of last seen tree
	let last_seen = seen_trees[tree as usize];
	// Check if this tree is visible from the edge
	if last_seen == 0 {
		// Add SEEN bit
		*cell |= SEEN;
		// Multiply scenic score by distance
		scenic_scores[width * y + x] *= distance;
	} else {
		// Multiply scenic score by distance from last tree, including that tree
		scenic_scores[width * y + x] *= distance - last_seen + 1;
	}
	// Update this and all shorter last seens to the distance index plus one
	seen_trees[..=tree as usize].fill(distance + 1);
}

fn cell_to_height(c: u8) -> u8 {
	c & MASK
}

fn index_mut(s: &mut [u8], width: usize, y: usize, x: usize) -> &mut u8 {
	debug_assert!(x < width);
	&mut s[(width + 1) * y + x]
}
