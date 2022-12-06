use crate::helpers::*;

type A1 = String;
type A2 = String;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		let mut line_iter = file.trim_ascii_end().lines();
		let mut stacks: Vec<Vec<u8>> = line_iter
			// Smallvec ended up being nearly the same speed. This might be a better optimization if
			// there is a hard limit on how large each vec can be.
			// let mut stacks: SmallVec<[SmallVec<[u8; 64]>; 10]> = line_iter
			.by_ref()
			.take_while(|line| !line.is_empty())
			.fold(Default::default(), |mut stacks, line| {
				let box_iter = line
					.iter()
					.copied()
					.chain([b' '])
					.array_chunks()
					.enumerate();
				for (i, [_, letter, _, _]) in box_iter {
					if stacks.len() == i {
						stacks.push(Default::default());
					}
					if letter != b' ' {
						stacks[i].push(letter);
					}
				}
				stacks
			});

		for stack in &mut stacks {
			stack.pop();
			stack.reverse();
			// print!("[");
			// for &mut item in stack {
			// 	print!("{}", item as char);
			// }
			// println!("]");
		}

		let instructions_iter = line_iter.map(|line| {
			let line = &line[5..];
			let (boxes_count, bytes): (usize, _) = atoi::FromRadix10::from_radix_10(line);
			let line = &line[(bytes + 6)..];
			let (from_stack, bytes): (usize, _) = atoi::FromRadix10::from_radix_10(line);
			let line = &line[(bytes + 4)..];
			let (to_stack, _): (usize, _) = atoi::FromRadix10::from_radix_10(line);
			// Change to zero-based indexing
			[boxes_count, from_stack - 1, to_stack - 1]
		});

		let mut stacks_part_2 = stacks.clone();

		for [boxes_count, from_stack, to_stack] in instructions_iter {
			// Part 1
			for _ in 0..boxes_count {
				let b = stacks[from_stack].pop().unwrap();
				stacks[to_stack].push(b);
			}

			// Part 2
			let [from_stack, to_stack] =
				stacks_part_2.get_many_mut([from_stack, to_stack]).unwrap();
			let from_len = from_stack.len();
			to_stack.extend_from_slice(&from_stack[(from_len - boxes_count)..]);
			from_stack.truncate(from_len - boxes_count);
		}

		let tops: String = stacks
			.iter()
			.filter_map(|stack| stack.last())
			.map(|&b| b as char)
			.collect();

		let tops_part_2: String = stacks_part_2
			.iter()
			.filter_map(|stack| stack.last())
			.map(|&b| b as char)
			.collect();

		Self {
			p1: tops,
			p2: tops_part_2,
		}
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		std::mem::take(&mut self.p1)
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		std::mem::take(&mut self.p2)
	}

	fn run_any_write<W: std::fmt::Write>(&mut self, part: u32, _writer: W) -> Res<()> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}
