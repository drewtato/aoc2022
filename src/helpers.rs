pub use crate::{AocError, Res, Solver};
pub use itertools::Itertools;
pub use regex::bytes::Regex;

mod parse_bytes;
pub use std::cmp::Reverse;
use std::io::stdin;
pub use std::num::Wrapping;
use std::ops::AddAssign;
use std::str::FromStr;

pub use ahash::{AHashMap as HashMap, AHashSet as HashSet, HashMapExt, HashSetExt};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

pub use parse_bytes::*;

mod neighbors;
pub use neighbors::*;

mod multi_parse;
pub use multi_parse::*;

mod input_data;
pub use input_data::*;

pub fn read_value<T>() -> Result<T, T::Err>
where
	T: FromStr,
{
	stdin().lines().next().unwrap().unwrap().trim().parse()
}

pub trait SelfSum: Iterator + Sized
where
	Self::Item: AddAssign + Default + Sized,
{
	fn sum_self(self) -> Self::Item {
		self.fold(Default::default(), |mut left, right| {
			left += right;
			left
		})
	}
}

impl<I> SelfSum for I
where
	I: Iterator + Sized,
	Self::Item: AddAssign + Default + Sized,
{
}

mod display_bytes;
pub use display_bytes::*;

/// Returns a curried function that compares a value to another value.
///
/// Example:
///
/// ```ignore
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
/// ```ignore
/// # use aoc2022::helpers::Wrap;
/// let i = 3;
/// assert_eq!(Some(3), i.wrap(Some));
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::Wrap;
/// let i = 3;
/// assert_eq!(Box::new(3), i.wrap_box());
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::Wrap;
/// let i = 3;
/// assert_eq!(&3, i.refer());
/// let mut i = 3;
/// assert_eq!(&mut 3, i.refmut());
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::Wrap;
/// let i = usize::MAX.wrap_wrapping() + 1.wrap_wrapping();
/// assert_eq!(0, i.0);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::Wrap;
/// let mut i = 3.wrap_repeat();
/// assert_eq!(Some(3), i.nth(1_000_000));
/// ```
///
/// ```ignore
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

	fn wrap_repeat(self) -> std::iter::Repeat<Self>
	where
		Self: Clone,
	{
		std::iter::repeat(self)
	}

	fn wrap_once(self) -> std::iter::Once<Self> {
		std::iter::once(self)
	}

	fn wrap_rev(self) -> Reverse<Self> {
		Reverse(self)
	}
}

impl<T> Wrap for T where T: Sized {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore]
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
