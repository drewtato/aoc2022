use crate::helpers::*;

pub type A1 = i32;
pub type A2 = i64;

#[derive(Debug)]
pub struct Solution {
	sensors: Vec<(GridCircle, i32, i32)>,
	search_space: i32,
	target_row: i32,
}

const MULTIPLIER: i64 = 4000000;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, dbg: u8) -> Self {
		let (target_row, search_space) = match dbg {
			1 => (10, 20),
			_ => (2_000_000, 4_000_000),
		};

		let mut sensors = Vec::with_capacity(32);

		file.trim_ascii_end().consume_lines(|mut line| {
			let original_len = line.len();

			line = &line[12..];
			let x1 = parse_consume_signed(&mut line);
			line = &line[4..];
			let y1 = parse_consume_signed(&mut line);
			line = &line[25..];
			let x2 = parse_consume_signed(&mut line);
			line = &line[4..];
			let y2 = parse_consume_signed(&mut line);

			let radius = distance(y1, x1, y2, x2);

			sensors.push((
				GridCircle {
					y: y1,
					x: x1,
					radius,
				},
				y2,
				x2,
			));

			let skip = original_len - line.len();
			Err(skip + 1)
		});

		Self {
			sensors,
			search_space,
			target_row,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let &mut Self {
			ref sensors,
			target_row,
			..
		} = self;

		let mut beacons = HashSet::new();

		let mut ranges = sensors
			.iter()
			.filter_map(|&(sensor, by, bx)| {
				if by == target_row {
					beacons.insert(bx);
				}
				let distance_to_target = sensor.y.abs_diff(target_row) as A1;
				let width_at_target = sensor.radius - distance_to_target;
				let range = (sensor.x - width_at_target)..(sensor.x + width_at_target + 1);
				if range.is_empty() {
					None
				} else {
					Some([range.start, range.end])
				}
			})
			.collect_vec();

		ranges.sort_unstable();

		let beacons_in_target_row = beacons.len() as A1;

		let mut last = A1::MIN;
		let mut count = 0;

		for range in ranges {
			let range = (range[0].max(last))..(range[1].max(last));
			last = range.end;
			count += range.len() as A1;
		}

		count - beacons_in_target_row
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut sensors = self.sensors.as_slice();

		let mut diagonal_lines_nw_se: Vec<[[i32; 2]; 2]> = Vec::new();
		let mut diagonal_lines_ne_sw = Vec::new();

		let [y, x] = 'b: {
			while let Some((a, _, _)) = sensors.take_first() {
				for (b, _, _) in sensors {
					if a.distance(*b) != a.radius + b.radius + 2 {
						continue;
					}

					let a_corners = [
						[a.y, a.x + a.radius],
						[a.y - a.radius, a.x],
						[a.y, a.x - a.radius],
						[a.y + a.radius, a.x],
					];

					let b_corners = [
						[b.y, b.x + b.radius],
						[b.y - b.radius, b.x],
						[b.y, b.x - b.radius],
						[b.y + b.radius, b.x],
					];

					let (mut line, ne_sw) = match (a.y < b.y, a.x < b.x) {
						(true, true) => (
							[
								[a_corners[0][0] + 1, a_corners[0][1]],
								[a_corners[3][0], a_corners[3][1] + 1],
								[b_corners[1][0], b_corners[1][1] - 1],
								[b_corners[2][0] - 1, b_corners[2][1]],
							],
							true,
						),
						(true, false) => (
							[
								[a_corners[2][0] + 1, a_corners[2][1]],
								[a_corners[3][0], a_corners[3][1] - 1],
								[b_corners[0][0] - 1, b_corners[0][1]],
								[b_corners[1][0], b_corners[1][1] + 1],
							],
							false,
						),
						(false, true) => (
							[
								[a_corners[0][0] - 1, a_corners[0][1]],
								[a_corners[1][0], a_corners[1][1] + 1],
								[b_corners[2][0] + 1, b_corners[2][1]],
								[b_corners[3][0], b_corners[3][1] - 1],
							],
							false,
						),
						(false, false) => (
							[
								[a_corners[1][0], a_corners[1][1] - 1],
								[a_corners[2][0] - 1, a_corners[2][1]],
								[b_corners[0][0] + 1, b_corners[0][1]],
								[b_corners[3][0], b_corners[3][1] + 1],
							],
							true,
						),
					};

					line.sort();

					if ne_sw {
						let [ne, sw] = [line[1], line[2]];

						for &[nw, se] in &diagonal_lines_nw_se {
							if let Some(ans) = self.check(nw, se, ne, sw) {
								break 'b ans;
							}
						}

						diagonal_lines_ne_sw.push([ne, sw]);
					} else {
						let [nw, se] = [line[1], line[2]];

						for &[ne, sw] in &diagonal_lines_ne_sw {
							if let Some(ans) = self.check(nw, se, ne, sw) {
								break 'b ans;
							}
						}

						diagonal_lines_nw_se.push([nw, se]);
					}
				}
			}

			for [nw, se] in diagonal_lines_nw_se {
				for &[ne, sw] in &diagonal_lines_ne_sw {
					let b1 = nw[0] - nw[1];
					let b2 = ne[0] + ne[1];

					let y = (b2 + b1) / 2;
					let x = (b2 - b1) / 2;

					if (b2 + b1) % 2 != 0 {
						continue;
					}
					if (b2 - b1) % 2 != 0 {
						continue;
					}

					if (nw[0]..=se[0]).contains(&y)
						&& (nw[1]..=se[1]).contains(&x)
						&& (ne[0]..=sw[0]).contains(&y)
						&& (sw[1]..=ne[1]).contains(&x)
						&& (0..=self.search_space).contains(&y)
						&& (0..=self.search_space).contains(&x)
					{
						break 'b [y, x];
					}
				}
			}
			panic!("No crossover found")
		};

		y as i64 + x as i64 * MULTIPLIER
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

impl Solution {
	fn check(&self, nw: [i32; 2], se: [i32; 2], ne: [i32; 2], sw: [i32; 2]) -> Option<[A1; 2]> {
		let b1 = nw[0] - nw[1];
		let b2 = ne[0] + ne[1];

		let y = (b2 + b1) / 2;
		let x = (b2 - b1) / 2;

		((b2 + b1) % 2 == 0
			&& (b2 - b1) % 2 == 0
			&& (nw[0]..=se[0]).contains(&y)
			&& (nw[1]..=se[1]).contains(&x)
			&& (ne[0]..=sw[0]).contains(&y)
			&& (sw[1]..=ne[1]).contains(&x)
			&& (0..=self.search_space).contains(&y)
			&& (0..=self.search_space).contains(&x))
		.then_some([y, x])
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GridCircle {
	y: A1,
	x: A1,
	radius: A1,
}

fn distance(y1: A1, x1: A1, y2: A1, x2: A1) -> A1 {
	(y1.abs_diff(y2) + x1.abs_diff(x2)) as _
}

#[allow(dead_code)]
impl GridCircle {
	fn distance(self, other: Self) -> A1 {
		distance(self.y, self.x, other.y, other.x)
	}

	fn area(self) -> A1 {
		2 * self.radius * (self.radius + 1) + 1
	}

	fn overlap_area(self, other: Self) -> A1 {
		let distance = self.distance(other);
		let total_radius = self.radius + other.radius;

		if distance <= total_radius {
			self.area() - other.area()
		} else if distance > total_radius {
			0
		} else {
			// We want to work with squares on the 45 degree "dual" grid

			// This finds the coordinate of self on the dual grid
			let day = self.y - self.x;
			let dax = self.y + self.x;

			// Same for other
			let dby = other.y - other.x;
			let dbx = other.y + other.x;

			// Get dual overlap's top-right corner
			let low_y = (day - self.radius).max(dby - other.radius);
			let low_x = (dax - self.radius).max(dbx - other.radius);

			// Get dual overlap's bottom-left corner
			let high_y = (day - self.radius).min(dby - other.radius);
			let high_x = (dax - self.radius).min(dbx - other.radius);

			// The area on the dual grid is the area of the overlap. The area we want is half that,
			// rounded down.
			(high_y - low_y) * (high_x - low_x) / 2
		}
	}
}
