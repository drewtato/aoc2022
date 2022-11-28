#![allow(dead_code)]

mod parse_bytes;
use std::io::stdin;
use std::iter::{Once, Repeat};
use std::num::Wrapping;
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

/// Universal methods for wrapping values.
///
/// # Examples
///
/// ```
/// # use aoc2022::helpers::Wrap;
/// let i = 3;
/// assert_eq!(Some(3), i.wrap(Some));
/// ```
///
/// ```
/// # use aoc2022::helpers::Wrap;
/// let i = 3;
/// assert_eq!(Box::new(3), i.wrap_box());
/// ```
///
/// ```
/// # use aoc2022::helpers::Wrap;
/// let i = 3;
/// assert_eq!(&3, i.refer());
/// let mut i = 3;
/// assert_eq!(&mut 3, i.refmut());
/// ```
///
/// ```
/// # use aoc2022::helpers::Wrap;
/// let i = usize::MAX.wrap_wrapping() + 1.wrap_wrapping();
/// assert_eq!(0, i.0);
/// ```
///
/// ```
/// # use aoc2022::helpers::Wrap;
/// let mut i = 3.wrap_repeat();
/// assert_eq!(Some(3), i.nth(1_000_000));
/// ```
///
/// ```
/// # use aoc2022::helpers::Wrap;
/// let mut i = 3.wrap_once();
/// assert_eq!(Some(3), i.next());
/// assert_eq!(None, i.next());
/// ```
pub trait Wrap: Sized {
	fn wrap<F, T>(self, func: F) -> T
	where
		F: FnOnce(Self) -> T,
	{
		func(self)
	}

	fn refer(&self) -> &Self {
		self
	}

	fn refmut(&mut self) -> &mut Self {
		self
	}

	fn wrap_box(self) -> Box<Self> {
		Box::new(self)
	}

	fn wrap_wrapping(self) -> Wrapping<Self> {
		Wrapping(self)
	}

	fn wrap_repeat(self) -> Repeat<Self>
	where
		Self: Clone,
	{
		std::iter::repeat(self)
	}

	fn wrap_once(self) -> Once<Self> {
		std::iter::once(self)
	}
}

impl<T> Wrap for T where T: Sized {}

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
