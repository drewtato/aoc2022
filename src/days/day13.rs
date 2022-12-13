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
		let mut count = 0;

		let div_one: Packet = b"[[2]]".parse().unwrap();

		let div_two: Packet = b"[[6]]".parse().unwrap();

		let mut input = file
			.trim_ascii()
			.lines()
			.filter(|l| !l.is_empty())
			.map(|l| l.parse::<Packet>().unwrap())
			.array_chunks()
			.enumerate()
			.flat_map(|(i, [l1, l2])| {
				if l1 < l2 {
					count += i + 1;
					[l1, l2]
				} else {
					[l2, l1]
				}
			})
			.chain([div_one.clone(), div_two.clone()])
			.collect_vec();

		input.sort_unstable();

		let one_pos = input.iter().position(|p| div_one.eq(p)).unwrap() + 1;
		let two_pos = input.iter().position(|p| div_two.eq(p)).unwrap() + 1;

		Self {
			p1: count,
			p2: one_pos * two_pos,
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

use packet::Packet;
mod packet {
	use super::*;
	use std::fmt::Debug;

	#[derive(Clone, PartialEq, Eq, Hash)]
	pub enum Packet {
		List(Vec<Packet>),
		Int(i32),
	}

	impl Ord for Packet {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			match (self, other) {
				(Packet::List(a), Packet::List(b)) => a.cmp(b),
				(a @ Packet::List(_), b @ Packet::Int(_)) => {
					let b = vec![b.clone()];
					a.cmp(&Packet::List(b))
				}
				(a @ Packet::Int(_), b @ Packet::List(_)) => {
					let a = vec![a.clone()];
					Packet::List(a).cmp(b)
				}
				(&Packet::Int(a), &Packet::Int(b)) => a.cmp(&b),
			}
		}
	}

	impl PartialOrd for Packet {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			Some(self.cmp(other))
		}
	}

	impl Debug for Packet {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Self::List(inner) => f.debug_list().entries(inner).finish(),
				Self::Int(int) => write!(f, "{}", int),
			}
		}
	}

	fn parse_packet_list(bytes: &[u8]) -> (Vec<Packet>, &[u8]) {
		// println!("{:?}", bytes.to_display_slice());
		debug_assert!(bytes[0] == b'[');

		let mut bytes = &bytes[1..];
		let mut list = Vec::new();

		while bytes[0] != b']' {
			// println!("{:?}", bytes.to_display_slice());
			if bytes[0] == b'[' {
				// println!("recurse");
				let (inner, new_bytes) = parse_packet_list(bytes);
				list.push(Packet::List(inner));
				bytes = new_bytes;
			} else {
				let int = parse_consume_unsigned(&mut bytes);
				list.push(Packet::Int(int));
			}
			if bytes[0] == b',' {
				bytes = &bytes[1..];
			}
		}
		(list, &bytes[1..])
	}

	impl FromBytes for Packet {
		fn from_bytes(bytes: &[u8]) -> Option<Self> {
			let (list, slice) = parse_packet_list(bytes);
			debug_assert!(slice.is_empty());
			Some(Packet::List(list))
		}
	}
}
