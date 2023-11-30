use crate::helpers::*;

pub type A1 = usize;
pub type A2 = u32;

#[derive(Debug)]
pub struct Solution {
	p1: A1,
	p2: A2,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _dbg: u8) -> Self {
		let mut count = 0;
		let mut one_pos = 1;
		let mut two_pos = 2;

		let mut file = file.as_slice();
		let packets = std::iter::from_fn(|| {
			if file.is_empty() {
				return None;
			}

			let start = file;
			while *file.take_first().unwrap() != b'\n' {}
			let line = &start[..(start.len() - file.len() - 1)];

			Some(line)
		})
		.filter(|l| !l.is_empty())
		.array_chunks()
		.enumerate();

		for (i, [a, b]) in packets {
			let FourOrder {
				a_smaller_than_b,
				c_bigger_than,
				d_bigger_than,
			} = order_four(a, b);

			// if dbg > 0 {
			// 	let a = a.to_display_slice();
			// 	let b = b.to_display_slice();
			// 	dbg_small!(a, b, result);
			// 	eprintln!();
			// }

			if a_smaller_than_b {
				count += 1 + i;
			}
			one_pos += c_bigger_than;
			two_pos += d_bigger_than;
		}

		Self {
			p1: count,
			p2: one_pos * two_pos,
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

use packet::*;
mod packet {
	use atoi::FromRadix10;

	use super::*;
	use std::cmp::Ordering;
	use std::fmt::Debug;

	const OPEN: u8 = b'[';
	const CLOSE: u8 = b']';
	const COMMA: u8 = b',';

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct FourOrder {
		pub a_smaller_than_b: bool,
		pub c_bigger_than: A2,
		pub d_bigger_than: A2,
	}

	pub fn order_four(mut a: &[u8], mut b: &[u8]) -> FourOrder {
		let original_a = a;
		let original_b = b;

		advance(&mut a, &mut b);
		let mut depth = 1;

		// This loop starts with a and b inside a list
		let a_smaller_than_b = loop {
			if cfg!(debug_assertions) && depth == 0 {
				panic!(
					"Found two equal packets:\n{}\n{}",
					original_a.to_display_slice(),
					original_b.to_display_slice()
				);
			}

			match order_one_part(&mut a, &mut b, &mut depth) {
				Ordering::Less => break true,
				Ordering::Equal => (),
				Ordering::Greater => break false,
			}
		};

		let c = 2;
		let d = 6;
		let mut c_bigger_than = 0;
		let mut d_bigger_than = 0;

		let nums: [Option<u8>; 2] =
			determine_first_num_in_first_list(&original_a[1..], &original_b[1..]);
		for n in nums {
			if let Some(n) = n {
				if n < c {
					c_bigger_than += 1;
				}
				if n < d {
					d_bigger_than += 1;
				}
			} else {
				c_bigger_than += 1;
				d_bigger_than += 1;
			}
		}

		FourOrder {
			a_smaller_than_b,
			c_bigger_than,
			d_bigger_than,
		}
	}

	fn determine_first_num_in_first_list(mut a: &[u8], mut b: &[u8]) -> [Option<u8>; 2] {
		while a[0] == OPEN {
			a = &a[1..];
		}
		while b[0] == OPEN {
			b = &b[1..];
		}
		[
			(a[0] != CLOSE).then(|| FromRadix10::from_radix_10(a).0),
			(b[0] != CLOSE).then(|| FromRadix10::from_radix_10(b).0),
		]
	}

	fn order_one_part(a: &mut &[u8], b: &mut &[u8], depth: &mut u32) -> Ordering {
		let cmp = match [a[0], b[0]] {
			// Both are open
			[OPEN, OPEN] => {
				*depth += 1;
				advance(a, b);
				return Ordering::Equal;
			}
			// Both are close
			[CLOSE, CLOSE] => {
				*depth -= 1;
				advance(a, b);
				Ordering::Equal
			}

			// a is at least length 1, b is length 0
			[_, CLOSE] => return Ordering::Greater,
			// b is at least length 1, a is length 0
			[CLOSE, _] => return Ordering::Less,

			// At least one is a digit
			// b is digit
			[OPEN, digit_b] => {
				debug_assert!(digit_b.is_ascii_digit());
				let num_b = parse_consume_unsigned(b);
				order_num_and_list(num_b, a).reverse()
			}
			// a is digit
			[digit_a, OPEN] => {
				debug_assert!(digit_a.is_ascii_digit());
				let num_a = parse_consume_unsigned(a);
				order_num_and_list(num_a, b)
			}
			// both are digits
			[digit_a, digit_b] => {
				debug_assert!(digit_a.is_ascii_digit());
				debug_assert!(digit_b.is_ascii_digit());
				let num_a: u8 = parse_consume_unsigned(a);
				let num_b: u8 = parse_consume_unsigned(b);
				num_a.cmp(&num_b)
			}
		};

		// Remove next comma
		if a[0] == COMMA {
			*a = &a[1..];
		}
		if b[0] == COMMA {
			*b = &b[1..];
		}

		cmp
	}

	fn order_num_and_list(num: u8, list: &mut &[u8]) -> Ordering {
		// Determine depth of list's first number
		let mut list_depth = 0;
		while list[0] == OPEN {
			*list = &list[1..];
			list_depth += 1;
		}

		// If this list is length zero, it is less than num, which is a pseudo list of length 1
		if list[0] == CLOSE {
			return Ordering::Greater;
		}

		// Otherwise, we have a number
		let list_num = parse_consume_unsigned(list);
		num.cmp(&list_num).then_with(|| {
			while list_depth != 0 {
				// When the numbers are equal, check the next character
				match list[0] {
					// When it's a comma, the list is longer, so num is less
					COMMA => return Ordering::Less,
					// When it's a closing bracket, the lists are the same length, so check the next
					// list up
					CLOSE => {
						*list = &list[1..];
						list_depth -= 1;
					}
					_ => panic!(
						"Invalid next character {:?} in {:?}",
						list[0].to_display_byte(),
						list.to_display_slice()
					),
				}
			}
			// The list was a number equal to num inside some amount of nested length-1 lists
			Ordering::Equal
		})
	}

	fn advance(a: &mut &[u8], b: &mut &[u8]) {
		*a = &a[1..];
		*b = &b[1..];
	}

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
				Self::Int(int) => write!(f, "{int}"),
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
