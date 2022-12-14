use crate::helpers::*;

type A1 = u32;
type A2 = u32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

const WIDTH: usize = 200;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut field = vec![[0; WIDTH]; WIDTH];

		let input = file.trim_ascii().lines().flat_map(|mut line| {
			std::iter::from_fn(move || {
				if line.is_empty() {
					None
				} else {
					let n = parse_consume_signed(&mut line);
					line = &line[1..];
					let m = parse_consume_signed(&mut line);
					if !line.is_empty() {
						line = &line[4..];
					}
					Some([n, m])
				}
			})
			.tuple_windows()
		});

		let mut lowest = 0;
		let mut leftmost = WIDTH;
		let mut rightmost = 0;
		for ([x1, y1], [x2, y2]) in input {
			let mut xs = [adjust(x1), adjust(x2)];
			let mut ys = [y1, y2];
			xs.sort();
			ys.sort();

			let [x1, x2] = xs;
			let [y1, y2] = ys;
			lowest = lowest.max(y2);
			leftmost = leftmost.min(x1);
			rightmost = rightmost.max(x2);

			if x1 == x2 {
				for n in &mut field[y1..=y2] {
					n[x1] = 1;
				}
			} else if y1 == y2 {
				for n in &mut field[y1][x1..=x2] {
					*n = 1;
				}
			} else {
				panic!("Diagonal line oh no!");
			}
		}

		leftmost -= 1;
		rightmost += 1;
		let sand_start = [adjust(500), 0];
		let mut sand_count = 0;
		let bottom_edge = lowest + 1;

		let p1 = 'l: loop {
			let mut current = sand_start;
			loop {
				if current[1] == bottom_edge {
					break 'l sand_count;
				}
				if 0 == field[current[1] + 1][current[0]] {
					current[1] += 1;
				} else if 0 == field[current[1] + 1][current[0] - 1] {
					current[0] -= 1;
					current[1] += 1;
				} else if 0 == field[current[1] + 1][current[0] + 1] {
					current[0] += 1;
					current[1] += 1;
				} else {
					// Sand has stopped
					break;
				}
			}
			sand_count += 1;
			// println!("Sand got to {:?}", current);
			// read_value::<String>().unwrap();
			field[current[1]][current[0]] = 2;
		};

		loop {
			let mut current = sand_start;
			loop {
				if current[1] == bottom_edge {
					break;
				}

				if 0 == field[current[1] + 1][current[0]] {
					current[1] += 1;
				} else if 0 == field[current[1] + 1][current[0] - 1] {
					if current[0] - 1 < leftmost {
						if 0 == field[current[1] + 1][current[0] + 1] {
							current[0] += 1;
							current[1] += 1;
						} else {
							break;
						}
					} else {
						current[0] -= 1;
						current[1] += 1;
					}
				} else if 0 == field[current[1] + 1][current[0] + 1] {
					if current[0] + 1 > rightmost {
						break;
					}
					current[0] += 1;
					current[1] += 1;
				} else {
					// Sand has stopped
					break;
				}
			}

			field[current[1]][current[0]] = 2;
			sand_count += 1;
			if current == sand_start {
				break;
			}
			// println!("Sand got to {:?}", current);
			// read_value::<String>().unwrap();

			// if sand_count % 1000 == 0 {
			// 	println!("\n\n\n{sand_count}");
			// 	print_field(&field, bottom_edge, leftmost, rightmost);
			// }
		}

		// print_field(&field, bottom_edge, leftmost, rightmost);
		// dbg_small!(leftmost, rightmost, bottom_edge);

		let right_len = bottom_edge - (rightmost - adjust(500));
		let left_len = bottom_edge - (adjust(500) - leftmost);
		sand_count += triangular_number(right_len as u32);
		sand_count += triangular_number(left_len as u32);

		Self { p1, p2: sand_count }
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
fn print_field(field: &[[u8; 1000]], bottom: usize, leftmost: usize, rightmost: usize) {
	println!();
	for line in &field[..bottom + 1] {
		for &spot in &line[(leftmost - 2)..=(rightmost + 2)] {
			match spot {
				0 => print!(" "),
				1 => print!("#"),
				2 => print!("."),
				_ => panic!(),
			}
		}
		println!();
	}
}

fn adjust(n: usize) -> usize {
	n - (1000 - WIDTH) / 2
}
