use std::cmp::Ordering;

use itertools::Either;

use crate::helpers::*;

pub type A1 = i64;
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
		let (mut finished, mut in_progress): (HashMap<_, _>, HashMap<_, _>) =
			file.trim_ascii().lines().partition_map(|line| {
				let begin = &line[0..4];
				let symbol = match line.get(11) {
					Some(&s) => s,
					None => return Either::Left((begin, line[6..].parse::<A1>().unwrap())),
				};
				Either::Right((begin, (&line[6..10], symbol, &line[13..17])))
			});

		let p1 = find_root(&finished, &in_progress).unwrap();

		finished.remove(&b"humn".as_slice()).unwrap();
		in_progress.get_mut(b"root".as_slice()).unwrap().1 = b'-';

		for _ in 0..50 {
			iterate(&mut in_progress, &mut finished);
		}

		// We're assuming the number we need is > 1 and the function is either increasing or
		// decreasing.
		finished.insert(b"humn", 1);
		let first = find_root(&finished, &in_progress).unwrap();
		let mut current = 1;

		let p2 = if first > 0 {
			let (mut min, mut max) = loop {
				finished.insert(b"humn", current);

				let n = find_root(&finished, &in_progress).unwrap();
				if n > 0 {
					current *= 2;
				} else {
					break (current / 2, current);
				}
			};

			loop {
				let middle = (min + max) / 2;

				finished.insert(b"humn", middle);
				let n = find_root(&finished, &in_progress).unwrap();

				match n.cmp(&0) {
					Ordering::Less => max = middle,
					Ordering::Equal => break middle,
					Ordering::Greater => min = middle,
				}
			}
		} else {
			let (mut min, mut max) = loop {
				finished.insert(b"humn", current);

				let n = find_root(&finished, &in_progress).unwrap();
				if n < 0 {
					current *= 2;
				} else {
					// This -1 is to satisfy test 1
					break (current / 2, current - 1);
				}
			};

			loop {
				let middle = (min + max) / 2;

				finished.insert(b"humn", middle);
				let n = find_root(&finished, &in_progress).unwrap();

				match n.cmp(&0).reverse() {
					Ordering::Less => max = middle,
					Ordering::Equal => break middle,
					Ordering::Greater => min = middle,
				}
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

fn find_root(
	finished: &HashMap<&[u8], A1>,
	in_progress: &HashMap<&[u8], Operation<'_>>,
) -> Option<i64> {
	let mut finished = finished.clone();
	let mut in_progress = in_progress.clone();
	loop {
		if let Some(&root) = finished.get(&b"root".as_slice()) {
			break Some(root);
		}
		if iterate(&mut in_progress, &mut finished) {
			return None;
		}
	}
}

/// Returns true when an operation overflowed.
fn iterate<'a>(
	in_progress: &mut HashMap<&'a [u8], Operation<'a>>,
	finished: &mut HashMap<&'a [u8], i64>,
) -> bool {
	let mut overflowed = false;
	in_progress.retain(|&name, &mut (a, op, b)| {
		if let (Some(&a), Some(&b)) = (finished.get(&a), finished.get(&b)) {
			let res = match op {
				b'+' => a.checked_add(b),
				b'-' => a.checked_sub(b),
				b'*' => a.checked_mul(b),
				b'/' => a.checked_div(b),
				_ => panic!("Unknown symbol"),
			};
			let res = res.unwrap_or_else(|| {
				overflowed = true;
				0
			});
			finished.insert(name, res);
			false
		} else {
			true
		}
	});
	overflowed
}

type Operation<'a> = (&'a [u8], u8, &'a [u8]);
