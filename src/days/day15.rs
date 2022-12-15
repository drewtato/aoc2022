use crate::helpers::*;

type A1 = i32;
type A2 = i64;

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

		let r = Regex::new(
			r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
		)
		.unwrap();
		let sensors = file
			.trim_ascii()
			.lines()
			.map(|line| {
				let [x, y, c, d] = r
					.captures(line)
					.unwrap()
					.iter()
					.skip(1)
					.map(|n| n.unwrap().as_bytes().parse().unwrap())
					.array_chunks()
					.next()
					.unwrap();
				let radius = distance(y, x, d, c);
				(GridCircle { y, x, radius }, d, c)
			})
			.collect_vec();

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
		let &mut Self {
			ref sensors,
			search_space,
			..
		} = self;

		let mut y = 0;
		let mut ranges = Vec::new();
		let [y, x]: [A1; 2] = 'l: loop {
			let target_row = y;

			sensors
				.iter()
				.filter_map(|&(sensor, _, _)| {
					let distance_to_target = sensor.y.abs_diff(target_row) as A1;
					let width_at_target = sensor.radius - distance_to_target;
					let range = (sensor.x - width_at_target)..(sensor.x + width_at_target + 1);
					if range.is_empty() {
						None
					} else {
						Some([range.start, range.end])
					}
				})
				.collect_into(&mut ranges);

			ranges.push([-1, 0]);
			ranges.push([search_space, search_space + 1]);
			ranges.sort_unstable();

			let mut last = 0;

			// let mut seen = HashSet::new();

			for &[[_start1, end1], [start2, _end2]] in ranges.array_windows() {
				let end1 = end1.max(last);
				last = end1;
				if end1 == start2 - 1 && (0..=search_space).contains(&end1) {
					break 'l [y, end1];
				}
				// for n in start1..end1 {
				// 	seen.insert(n);
				// }
			}

			ranges.clear();

			y += 1;

			if cfg!(debug_assert) && y > search_space {
				panic!("Didn't find beacon")
			}
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
