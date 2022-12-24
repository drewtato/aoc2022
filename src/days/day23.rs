#![allow(unused)]

use crate::helpers::*;

type A1 = i32;
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
		let mut elves: HashSet<[A1; 2]> = file
			.trim_ascii_end()
			.lines()
			.enumerate()
			.flat_map(|(y, line)| {
				line.iter().enumerate().filter_map(move |(x, c)| match c {
					b'#' => Some([y as A1, x as A1]),
					b'.' => None,
					_ => panic!("Unknown character"),
				})
			})
			.collect();

		// dbg_small!(input.len());

		let neighbors = [
			[[-1, 0], [-1, -1], [-1, 1]],
			[[1, 0], [1, -1], [1, 1]],
			[[0, -1], [1, -1], [-1, -1]],
			[[0, 1], [1, 1], [-1, 1]],
		];

		let mut considerations: HashMap<[A1; 2], Option<[A1; 2]>> =
			HashMap::with_capacity(elves.len());
		let mut p1 = None;
		let elf_count = elves.len();

		let p2 = 'l: {
			for i in 0.. {
				if cfg!(debug_assertions) && elves.len() != elf_count {
					print_grove(&elves);
					panic!("Elves multiplied on step {i}");
				}

				if i == 10 {
					let (min_y, max_y) = elves.iter().map(|e| e[0]).minmax().into_option().unwrap();
					let (min_x, max_x) = elves.iter().map(|e| e[1]).minmax().into_option().unwrap();
					let p1_n = (max_y - min_y + 1) * (max_x - min_x + 1) - elves.len() as A1;
					p1 = Some(p1_n);
				}

				let mut moved = false;
				for &elf in &elves {
					let count = (-1..=1)
						.flat_map(|y| (-1..=1).map(move |x| [y, x]))
						.filter(|&pos| elves.contains(&add(elf, pos)))
						.count();
					debug_assert!(count != 0);
					if count == 1 {
						continue;
					}

					moved = true;

					for n in neighbors.iter().cycle().skip(i % 4).take(4) {
						if n.iter().all(|&place| !elves.contains(&add(elf, place))) {
							let n = add(elf, n[0]);
							considerations
								.entry(n)
								.and_modify(|e| *e = None)
								.or_insert(Some(elf));
							break;
						}
					}
				}

				if !moved {
					break 'l i as A1 + 1;
				}

				for (to, from) in considerations.drain() {
					if let Some(from) = from {
						let removed = elves.remove(&from);
						debug_assert!(removed);
						elves.insert(to);
					}
				}
			}
			unreachable!()
		};

		Self {
			p1: p1.unwrap(),
			p2,
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

fn add(a: [A1; 2], b: [A1; 2]) -> [A1; 2] {
	let [a1, a2] = a;
	let [b1, b2] = b;
	[a1 + b1, a2 + b2]
}

fn print_grove(elves: &HashSet<[A1; 2]>) {
	let (min_y, max_y) = elves.iter().map(|e| e[0]).minmax().into_option().unwrap();
	let (min_x, max_x) = elves.iter().map(|e| e[1]).minmax().into_option().unwrap();
	print!("   ");
	for x in min_x..=max_x {
		print!("{}", x.rem_euclid(10));
	}
	println!();
	for y in min_y..=max_y {
		print!("{y:2} ");
		for x in min_x..=max_x {
			if elves.contains(&[y, x]) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!();
	}
}
