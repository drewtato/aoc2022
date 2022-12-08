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

	fn initialize(file: Vec<u8>) -> Self {
		let input = file.trim_ascii_end().grid(|b| b - b'0');
		let mut visible_from_edge = 0;
		let mut best_scenic_score = 0;

		for (y, row) in input.iter().enumerate() {
			for (x, &tree) in row.iter().enumerate() {
				let north_trees = input.iter().take(y).rev().map(|row| row[x]).collect_vec();
				let south_trees = input.iter().skip(y + 1).map(|row| row[x]).collect_vec();
				let west_trees = row.iter().take(x).rev().copied().collect_vec();
				let east_trees = row.iter().skip(x + 1).copied().collect_vec();
				// println!("{north_trees:?}");
				// println!("{south_trees:?}");
				// println!("{west_trees:?}");
				// println!("{east_trees:?}");
				'a: for range in [&north_trees, &south_trees, &west_trees, &east_trees] {
					for &obstruction in range {
						if obstruction >= tree {
							continue 'a;
						}
					}
					visible_from_edge += 1;
					break;
				}
				let mut scenic_score = 1;
				for range in [&north_trees, &south_trees, &west_trees, &east_trees] {
					let mut visible_from_tree = 0;
					for (i, &obstruction) in range.iter().enumerate() {
						if obstruction >= tree {
							visible_from_tree = i as i32 + 1;
							break;
						}
					}
					if visible_from_tree == 0 {
						scenic_score *= range.len() as i32;
					} else {
						scenic_score *= visible_from_tree;
					}
				}
				best_scenic_score = best_scenic_score.max(scenic_score);
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
