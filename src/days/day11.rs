#![allow(unused)]

use crate::helpers::*;

type A1 = u64;
type A2 = A1;

#[derive(Debug)]
pub struct Solution {
	monkeys: Vec<Monkey>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut monkeys = file
			.trim_ascii()
			.lines()
			.map(|line| {
				line.trim_ascii()
					.split(|&b| matches!(b, b' ' | b','))
					.filter_map(|word| {
						if word.is_empty() {
							return None;
						}
						Some(
							word.wrap(std::str::from_utf8)
								.unwrap()
								.parse::<A1>()
								.map_err(|_| word),
						)
					})
					.collect_vec()
			})
			.chunks(7)
			.into_iter()
			.map(|chunk| {
				let [_, l1, l2, l3, l4, l5] = chunk.array_chunks().next().unwrap();
				let l1 = l1
					.into_iter()
					.skip(2)
					.map(|word| word.unwrap())
					.collect_vec();
				let l2 = (l2[4].unwrap_err().into(), l2[5].into());
				let l3 = l3.last().unwrap().unwrap();
				let l4 = l4.last().unwrap().unwrap() as usize;
				let l5 = l5.last().unwrap().unwrap() as usize;
				Monkey {
					items: l1,
					operation: l2,
					test: l3,
					if_true: l4,
					if_false: l5,
					inspected_count: 0,
				}
			})
			.collect_vec();

		// dbg!(&monkeys);

		Self { monkeys }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut monkeys = self.monkeys.clone();
		let mut current_monkey = 0;
		let mut round = 0;
		let len = monkeys.len();

		loop {
			let &mut Monkey {
				ref mut items,
				operation,
				test,
				if_true,
				if_false,
				ref mut inspected_count,
			} = &mut monkeys[current_monkey % len];

			*inspected_count += items.len();

			match operation.0 {
				Operator::Multiply => {
					for item in std::mem::take(items).into_iter() {
						let op1 = item;
						let op2 = match operation.1 {
							Operand::Number(n) => n,
							Operand::Old => item,
						};
						let acc = (op1 * op2) / 3;

						let dest = if acc % test == 0 { if_true } else { if_false };
						monkeys[dest].items.push(acc);
					}
				}
				Operator::Add => {
					for item in std::mem::take(items).into_iter() {
						let op1 = item;
						let op2 = match operation.1 {
							Operand::Number(n) => n,
							Operand::Old => item,
						};
						let acc = (op1 + op2) / 3;

						let dest = if acc % test == 0 { if_true } else { if_false };
						monkeys[dest].items.push(acc);
					}
				}
			}

			current_monkey += 1;
			if current_monkey == len * 20 {
				break;
			}
		}

		monkeys
			.iter()
			.map(|monkey| monkey.inspected_count as A1)
			.sorted_unstable()
			.rev()
			.take(2)
			.product()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut monkeys = self.monkeys.clone();
		let mut current_monkey = 0;
		let mut round = 0;
		let len = monkeys.len();

		let modulo: A1 = monkeys.iter().map(|monkey| monkey.test).product();

		// For testing cycles. It's not viable since it seems like my input is unique for repeating
		// before 10,000 rounds.
		// let mut set = HashSet::with_capacity(100);
		// let mut duplicate = 0;

		loop {
			let &mut Monkey {
				ref mut items,
				operation,
				test,
				if_true,
				if_false,
				ref mut inspected_count,
			} = &mut monkeys[current_monkey % len];

			*inspected_count += items.len();
			let mut temp_items = std::mem::take(items);
			match operation.0 {
				Operator::Multiply => {
					for item in temp_items.drain(..) {
						let op2 = match operation.1 {
							Operand::Number(n) => n,
							Operand::Old => item,
						};
						let acc = (item * op2) % modulo;

						let dest = if acc % test == 0 { if_true } else { if_false };
						monkeys[dest].items.push(acc);
					}
				}
				Operator::Add => {
					for item in temp_items.drain(..) {
						let op2 = match operation.1 {
							Operand::Number(n) => n,
							Operand::Old => item,
						};
						let acc = (item + op2) % modulo;

						let dest = if acc % test == 0 { if_true } else { if_false };
						monkeys[dest].items.push(acc);
					}
				}
			}

			monkeys[current_monkey % len].items = temp_items;

			// More cycle testing code
			// if current_monkey % len == 0
			// 	&& !set.insert(monkeys.iter().map(|m| m.items.clone()).collect_vec())
			// {
			// 	println!(
			// 		"Got duplicate after monkey {current_monkey} on round {}",
			// 		current_monkey / len
			// 	);
			// 	duplicate += 1;
			// 	if duplicate == 5 {
			// 		return 0;
			// 	}
			// }

			current_monkey += 1;
			if current_monkey == len * 10_000 {
				break;
			}
		}

		monkeys
			.iter()
			.map(|monkey| monkey.inspected_count as A1)
			.sorted_unstable()
			.rev()
			.take(2)
			.product()
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

#[derive(Debug, Clone)]
struct Monkey {
	items: Vec<A1>,
	operation: (Operator, Operand),
	test: A1,
	if_true: usize,
	if_false: usize,
	inspected_count: usize,
}

#[derive(Debug, Clone, Copy)]
enum Operand {
	Number(A1),
	Old,
}

impl From<Result<A1, &[u8]>> for Operand {
	fn from(value: Result<A1, &[u8]>) -> Self {
		match value {
			Ok(n) => Self::Number(n),
			Err(b"old") => Self::Old,
			_ => panic!(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Operator {
	Multiply,
	Add,
}

impl From<&[u8]> for Operator {
	fn from(value: &[u8]) -> Self {
		match value {
			b"+" => Self::Add,
			b"*" => Self::Multiply,
			_ => panic!(),
		}
	}
}
