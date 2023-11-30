use std::ops::Range;

use crate::helpers::*;

pub type A1 = usize;
pub type A2 = usize;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut field: Vec<BTreeSet<[usize; 2]>> = Vec::with_capacity(128);
		let mut set_field: HashSet<[usize; 2]> = HashSet::with_capacity(1024);

		let input = file.trim_ascii().lines().flat_map(|mut line| {
			std::iter::from_fn(move || {
				if line.is_empty() {
					None
				} else {
					let n: usize = parse_consume_signed(&mut line);
					line = &line[1..];
					let m: usize = parse_consume_signed(&mut line);
					if !line.is_empty() {
						line = &line[4..];
					}
					Some([n, m])
				}
			})
			.tuple_windows()
		});

		for ([x1, y1], [x2, y2]) in input {
			if x1 == x2 {
				let mut ys = [y1, y2];
				// vertical line
				ys.sort_unstable();
				let [y1, y2] = ys;
				if y2 >= field.len() {
					field.resize_with(y2 + 1, Default::default);
				}
				for (y, row) in field.iter_mut().enumerate().skip(y1).take(y2 - y1 + 1) {
					if set_field.insert([y, x1]) {
						row.insert([x1, x1 + 1]);
					}
				}
			} else {
				let mut xs = [x1, x2];
				// horizontal line
				xs.sort_unstable();
				let [x1, x2] = xs;
				if y2 >= field.len() {
					field.resize_with(y2 + 1, Default::default);
				}

				field[y2].remove(&[x1, x1 + 1]);
				field[y2].remove(&[x2, x2 + 1]);
				field[y2].insert([x1, x2 + 1]);

				for x in x1..=x2 {
					set_field.insert([y2, x]);
				}
			}
		}

		let mut p1 = 0;
		let mut pos_stack = Vec::with_capacity(512);
		pos_stack.push([0, 500]);
		let bottom = field.len() + 1;
		let mut y = 0;
		let mut x = 500;

		loop {
			y += 1;
			if y == bottom {
				break;
			}

			if !set_field.contains(&[y, x]) {
			} else if !set_field.contains(&[y, x - 1]) {
				x -= 1;
			} else if !set_field.contains(&[y, x + 1]) {
				x += 1;
			} else {
				y -= 1;
				set_field.insert([y, x]);
				// if field[y].contains(&(x - 1)) {

				// }

				p1 += 1;
				pos_stack.pop();
				[y, x] = *pos_stack.last().unwrap();
				continue;
			}
			pos_stack.push([y, x]);
		}

		// print_set_field(&set_field, 0, field.len(), 430, 530);

		#[allow(clippy::single_range_in_vec_init)]
		let mut sand_ranges = vec![500..501];
		let mut current_sand_ranges = Vec::new();
		let mut p2 = 1;

		for row in field.drain(1..).chain([Default::default()]) {
			std::mem::swap(&mut sand_ranges, &mut current_sand_ranges);

			let mut row_iter = row.iter();
			let mut row_range = row_iter.next();

			for mut sand in current_sand_ranges.drain(..) {
				sand.start -= 1;
				sand.end += 1;

				while let Some(r) = row_range {
					let r = r[0]..r[1];
					// println!("{sand:?} {r:?}");
					if r.end > sand.start {
						if r.start >= sand.end {
							break;
						}
						if r.contains(&sand.start) {
							sand.start = r.end;
						}
						if r.contains(&(sand.end - 1)) {
							sand.end = r.start;
							break;
						}
						if sand.start < r.start && r.end < sand.end {
							let split = sand.start..r.start;
							sand.start = r.end;
							// println!("split into {split:?} and {sand:?}");
							extend_or_push(&mut sand_ranges, split);
						}
					}
					row_range = row_iter.next();
				}

				if !sand.is_empty() {
					// println!("Adding {sand:?}");
					extend_or_push(&mut sand_ranges, sand);
				} else {
					// println!("Empty {sand:?}");
				}
			}

			for range in &sand_ranges {
				p2 += range.len();
			}
			// print_sand_row(&sand_ranges, &row, 300, 700);
			// read_value::<String>().unwrap();
		}

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

#[allow(dead_code)]
fn print_set_field(
	set_field: &HashSet<[usize; 2]>,
	min_y: usize,
	max_y: usize,
	min_x: usize,
	max_x: usize,
) {
	for y in min_y..=max_y {
		for x in min_x..=max_x {
			if set_field.contains(&[y, x]) {
				print!("#");
			} else {
				print!(" ");
			}
		}
		println!();
	}
}

fn extend_or_push(sand_ranges: &mut Vec<Range<usize>>, sand: Range<usize>) {
	match sand_ranges.last_mut() {
		Some(prev) if prev.end >= sand.start => prev.end = sand.end,
		_ => sand_ranges.push(sand),
	}
}

#[allow(dead_code)]
fn print_sand_row(sand: &[Range<usize>], rock: &BTreeSet<[usize; 2]>, min: usize, max: usize) {
	let mut all = Vec::new();
	for s in sand.iter().cloned().flatten() {
		if !(min..max).contains(&s) {
			continue;
		}
		let s = s - min;
		if s >= all.len() {
			all.resize(s + 1, b' ');
		}
		all[s] = match all[s] {
			b' ' => b'.',
			b'.' => b'a',
			a @ b'a'..=b'y' => a + 1,
			b'z' => b'z',
			a => panic!("Unknown character {a}"),
		}
	}
	for s in rock.iter().flat_map(|&[a, b]| a..b) {
		if !(min..max).contains(&s) {
			continue;
		}
		let s = s - min;
		if s >= all.len() {
			all.resize(s + 1, b' ');
		}
		all[s] = match all[s] {
			b' ' => b'#',
			b'.' => b'M',
			b'#' => b'1',
			a @ b'1'..=b'9' => a + 1,
			b'0' => b'0',
			a => panic!("Unknown character {a}"),
		}
	}
	println!("{}", all.to_display_slice());
}
