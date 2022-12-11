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

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let line_length = file.iter().position(|&b| b == b'\n').unwrap();
		let mut stacks = vec![Vec::with_capacity(32); (line_length + 1) / 4];
		let mut file = file.as_slice();
		loop {
			let &first = file.take_first().unwrap();
			if first == b'\n' {
				break;
			}

			let line = &file[..line_length - 1];
			for (&c, stack) in line.iter().step_by(4).zip(&mut stacks) {
				if c != b' ' {
					stack.push(c);
				}
			}
			file = &file[line_length..];
		}

		for stack in &mut stacks {
			stack.pop();
			stack.reverse();
		}

		let mut instructions_slice = file;
		let instructions_iter = std::iter::from_fn(|| {
			instructions_slice = instructions_slice.get(5..)?;
			let boxes_count = parse_consume_unsigned(&mut instructions_slice);
			instructions_slice = &instructions_slice[6..];
			let from_stack: usize = parse_consume_unsigned(&mut instructions_slice);
			instructions_slice = &instructions_slice[4..];
			let to_stack: usize = parse_consume_unsigned(&mut instructions_slice);
			instructions_slice = &instructions_slice[1..];
			// Change to zero-based indexing
			Some([boxes_count, from_stack - 1, to_stack - 1])
		});
		// let instructions_iter = line_iter.map(|line| {
		// 	let line = &line[5..];
		// 	let (boxes_count, bytes): (usize, _) = atoi::FromRadix10::from_radix_10(line);
		// 	let line = &line[(bytes + 6)..];
		// 	let (from_stack, bytes): (usize, _) = atoi::FromRadix10::from_radix_10(line);
		// 	let line = &line[(bytes + 4)..];
		// 	let (to_stack, _): (usize, _) = atoi::FromRadix10::from_radix_10(line);
		// 	// Change to zero-based indexing
		// 	[boxes_count, from_stack - 1, to_stack - 1]
		// });

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

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		std::mem::take(&mut self.p1)
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		std::mem::take(&mut self.p2)
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
