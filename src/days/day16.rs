#![allow(unused)]

use std::cmp::Ordering;
use std::hash::Hash;

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

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let r = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
			.unwrap();

		let mut map: HashMap<_, (u32, Vec<_>)> = HashMap::new();
		let mut room_indexes = HashMap::new();
		let mut reverse_room_indexes = Vec::new();
		for line in file.trim_ascii().lines() {
			// println!("{}", line.to_display_slice());
			let caps = r
				.captures(line)
				.unwrap()
				.iter()
				.map(|c| c.unwrap().as_bytes())
				.collect_vec();

			let name = caps[1].to_display_slice();
			reverse_room_indexes.push(name);
			let index = reverse_room_indexes.len() - 1;
			room_indexes.insert(name, index);

			map.insert(
				index,
				(
					caps[2].parse().unwrap(),
					caps[3]
						.split(|&b| b == b' ' || b == b',')
						.filter(|p| !p.is_empty())
						.map(|p| p.to_display_slice())
						.collect(),
				),
			);
		}

		// dbg!(map);

		let max_flow_per_minute = map.values().map(|&(flow, _)| flow).sum_self();
		let max_total_flow = max_flow_per_minute * 30;
		let map: HashMap<_, _> = map
			.into_iter()
			.map(|(room, (flow, connections))| {
				let connections = connections
					.into_iter()
					.map(|room| room_indexes[&room])
					.collect_vec();
				(room, (flow, connections))
			})
			.collect();

		// dbg!(reverse_room_indexes, map);

		// let mut states = MinHeap::new();
		// states.push();
		// loop {}

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

struct State {
	min_possible_flow: u32,
	max_possible_flow: u32,
	current_room: usize,
	opened_valves: u64,
}
