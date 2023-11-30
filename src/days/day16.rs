#![allow(unused)]

use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::Deref;

use crate::helpers::*;

pub type A1 = impl std::fmt::Display + std::fmt::Debug + Clone;
pub type A2 = impl std::fmt::Display + std::fmt::Debug + Clone;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

type Room = DisplaySlice<[u8; 2]>;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let r = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
			.unwrap();

		let mut room_indexes = HashMap::new();
		let mut index_rooms = Vec::new();

		let map: HashMap<usize, (u32, Vec<usize>)> = file
			.trim_ascii()
			.lines()
			.map(|line| {
				// println!("{}", line.to_display_slice());
				let caps = r
					.captures(line)
					.unwrap()
					.iter()
					.map(|c| c.unwrap().as_bytes())
					.collect_vec();

				let room = <[u8; 2]>::try_from(caps[1]).unwrap().to_display_slice();
				let room = *room_indexes.entry(room).or_insert_with(|| {
					index_rooms.push(room);
					index_rooms.len() - 1
				});

				let flow_rate = caps[2].parse().unwrap();
				let neighbors = caps[3]
					.split(|&b| b == b' ' || b == b',')
					.filter(|p| !p.is_empty())
					.map(|p| {
						let room = <[u8; 2]>::try_from(p).unwrap().to_display_slice();
						let room = *room_indexes.entry(room).or_insert_with(|| {
							index_rooms.push(room);
							index_rooms.len() - 1
						});
						room
					})
					.collect();

				(room, (flow_rate, neighbors))
			})
			.collect();

		let max_flow_per_minute = map.values().map(|&(flow, _)| flow).sum_self();

		let graph: HashMap<usize, Vec<(usize, u32)>> = map
			.keys()
			.map(|&room| {
				let paths = find_all_paths(room, &map).collect();
				(room, paths)
			})
			.collect();

		for item in &graph {
			dbg_small!(item);
		}

		let start = room_indexes[&b"AA".to_display_slice()];
		// dbg_small!(start);
		// dbg_small!(index_rooms);

		Self {
			p1: "Part 1 not implemented",
			p2: "Part 2 not implemented",
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.p1.clone()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.p2.clone()
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

fn find_all_paths(
	start: usize,
	map: &HashMap<usize, (u32, Vec<usize>)>,
) -> impl Iterator<Item = (usize, u32)> + '_ {
	let mut queue: VecDeque<(usize, u32)> = map[&start].1.iter().map(|&room| (room, 1)).collect();
	let mut all_rooms = HashSet::new();
	all_rooms.insert(start);

	from_fn_iter(move || {
		while let Some((room, distance)) = queue.pop_front() {
			if !all_rooms.insert(room) {
				continue;
			}

			for &next in &map[&room].1 {
				queue.push_back((next, distance + 1));
			}

			return Some((room, distance));
		}
		None
	})
}
