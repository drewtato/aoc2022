#![allow(dead_code)]

use crate::helpers::*;

type A1 = impl std::fmt::Display + std::fmt::Debug + Clone;
type A2 = impl std::fmt::Display + std::fmt::Debug + Clone;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>) -> Self {
		// These are all very similar in time, but VecSet works the best. It works best when the
		// capacities are equal.

		// let mut tail_positions = HashSet::with_capacity(7168);
		// let mut tenth_tail_positions = HashSet::with_capacity(3584);

		let mut tail_positions = VecSet::with_capacity(90);
		let mut tenth_tail_positions = VecSet::with_capacity(90);

		// let mut tail_positions = BitHashSet::with_capacity(7168 / 3);
		// let mut tenth_tail_positions = BitHashSet::with_capacity(3584);

		tail_positions.insert([0, 0]);
		tenth_tail_positions.insert([0, 0]);

		let mut knots = [[0i16, 0]; 10];
		let mut file = file.as_slice();
		let input = std::iter::from_fn(|| {
			let &direction = file.take_first()?;
			file.take_first();
			let count = parse_consume_unsigned(&mut file);
			file.take_first();
			Some((direction, count))
		});

		for (dir, count) in input {
			for _ in 0..count {
				match dir {
					b'R' => knots[0][1] += 1,
					b'L' => knots[0][1] -= 1,
					b'U' => knots[0][0] -= 1,
					b'D' => knots[0][0] += 1,
					_ => panic!("Unknown direction byte: '{}'", dir as char),
				}

				let mut unmoved_knots = knots.as_mut_slice();
				while let (Some(head), Some(tail)) =
					(unmoved_knots.take_first_mut(), unmoved_knots.first_mut())
				{
					let a = head[0] - tail[0];
					let b = head[1] - tail[1];
					if a.abs() > 1 || b.abs() > 1 {
						tail[0] += a.signum();
						tail[1] += b.signum();
					} else {
						break;
					}
				}

				tail_positions.insert(knots[1]);
				tenth_tail_positions.insert(knots[9]);
			}
		}

		// let (&min, &max) = tail_positions
		// 	.iter()
		// 	.chain(&tenth_tail_positions)
		// 	.flatten()
		// 	.minmax()
		// 	.into_option()
		// 	.unwrap();
		// println!("{max} - {min}");
		// println!(
		// 	"{}, {}",
		// 	tail_positions.capacity(),
		// 	tenth_tail_positions.capacity()
		// );

		Self {
			p1: tail_positions.len(),
			p2: tenth_tail_positions.len(),
		}
	}

	fn part_one(&mut self) -> Self::AnswerOne {
		self.p1.clone()
	}

	fn part_two(&mut self) -> Self::AnswerTwo {
		self.p2.clone()
	}

	fn run_any_write<W: std::fmt::Write>(&mut self, part: u32, _writer: W) -> Res<()> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}

#[derive(Debug, Clone)]
struct VecMap<T> {
	positive: Vec<[Vec<T>; 2]>,
	negative: Vec<[Vec<T>; 2]>,
}

impl<T> Default for VecMap<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> VecMap<T> {
	fn new() -> Self {
		Self {
			positive: Vec::new(),
			negative: Vec::new(),
		}
	}
	fn with_capacity(capacity: usize) -> Self
	where
		T: Default + Clone,
	{
		Self {
			positive: vec![[vec![T::default(); capacity], vec![T::default(); capacity]]; capacity],
			negative: vec![[vec![T::default(); capacity], vec![T::default(); capacity]]; capacity],
		}
	}
	fn get_mut(&mut self, key: [i16; 2]) -> &mut T
	where
		T: Default,
	{
		let [y, x] = key;
		let row = if y >= 0 {
			let y = y as usize;
			if y >= self.positive.len() {
				self.positive.resize_with(y + 1, Default::default);
			}
			&mut self.positive[y]
		} else {
			let y = (-y - 1) as usize;
			if y >= self.negative.len() {
				self.negative.resize_with(y + 1, Default::default);
			}
			&mut self.negative[y]
		};

		let (index, x) = if x >= 0 {
			(0, x as usize)
		} else {
			(1, (-x - 1) as usize)
		};
		let row = &mut row[index];
		if x >= row.len() {
			row.resize_with(x + 1, Default::default);
		}
		&mut row[x]
	}
}

#[derive(Debug, Default, Clone)]
struct VecSet {
	map: VecMap<u8>,
	len: u32,
}

impl VecSet {
	fn new() -> Self {
		Self::default()
	}
	fn with_capacity(capacity: usize) -> Self {
		Self {
			map: VecMap::with_capacity(capacity),
			len: 0,
		}
	}
	fn len(&self) -> u32 {
		self.len
	}
	fn insert(&mut self, item: [i16; 2]) {
		let [y, x] = item;
		let coarse_index = x.div_euclid(8);
		let fine_index = x.rem_euclid(8);
		let fine_byte = 1 << fine_index;
		let existing = self.map.get_mut([y, coarse_index]);
		if *existing & fine_byte == 0 {
			self.len += 1;
		}
		*existing |= fine_byte;
	}
}

#[derive(Debug, Default, Clone)]
struct BitHashSet {
	map: HashMap<[i16; 2], u8>,
	len: u32,
}

impl BitHashSet {
	fn new() -> Self {
		Self::default()
	}
	fn with_capacity(capacity: usize) -> Self {
		Self {
			map: HashMap::with_capacity(capacity),
			len: 0,
		}
	}
	fn len(&self) -> u32 {
		self.len
	}
	fn insert(&mut self, item: [i16; 2]) {
		let [y, x] = item;
		let coarse_index = x.div_euclid(8);
		let fine_index = x.rem_euclid(8);
		let fine_byte = 1 << fine_index;
		let existing = self.map.entry([y, coarse_index]).or_default();
		if *existing & fine_byte == 0 {
			self.len += 1;
		}
		*existing |= fine_byte;
	}
}
