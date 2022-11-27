#![allow(dead_code)]

mod parse_bytes;
use std::io::stdin;
use std::str::FromStr;

pub use parse_bytes::*;

mod neighbors;
pub use neighbors::*;

mod multi_parse;
pub use multi_parse::*;

pub fn read_value<T>() -> Result<T, T::Err>
where
	T: FromStr,
{
	stdin().lines().next().unwrap().unwrap().trim().parse()
}

/// Returns a curried function that compares a value to another value.
///
/// Example:
///
/// ```
/// # use aoc2022::helpers::is;
/// assert!(is("hello")("hello"));
/// ```
pub fn is<T: ?Sized>(byte: &T) -> impl for<'b> Fn(&'b T) -> bool + '_
where
	T: PartialEq,
{
	move |b| byte.eq(b)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn neighbors() {
		#[rustfmt::skip]
		let v = vec![
			vec![1, 0, 0],
			vec![0, 0, 0],
			vec![1, 1, 0]
			];
		let arr = v.neighbors(0, 0);
		#[rustfmt::skip]
		assert_eq!(arr, [
			[None, None,     None    ],
			[None, Some(&1), Some(&0)],
			[None, Some(&0), Some(&0)],
		]);

		let arr = v.neighbors(1, 1);
		#[rustfmt::skip]
		assert_eq!(arr, [
			[Some(&1), Some(&0), Some(&0)],
			[Some(&0), Some(&0), Some(&0)],
			[Some(&1), Some(&1), Some(&0)],
		]);

		let arr = v.neighbors(2, 2);
		#[rustfmt::skip]
		assert_eq!(arr, [
			[Some(&0), Some(&0), None],
			[Some(&1), Some(&0), None],
			[None,     None,     None],
		]);
	}
}
