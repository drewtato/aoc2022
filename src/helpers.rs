mod parse_bytes;
use std::io::stdin;
use std::ops::{Add, AddAssign, Div, Mul};
use std::str::FromStr;

pub use crate::{AocError, Res, Solver};
pub use itertools::Itertools;
pub use regex::bytes::Regex;
pub use std::array::{from_fn as from_fn_array, try_from_fn};
pub use std::cmp::Reverse;
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
pub use std::mem::{replace, swap, take};
pub use std::num::Wrapping;
pub use std::str::from_utf8;

pub use ahash::{AHashMap as HashMap, AHashSet as HashSet, HashMapExt, HashSetExt};

pub use std::iter::{
	empty as empty_iter, from_fn as from_fn_iter, from_generator as gen_iter, once as once_iter,
	once_with as once_with_iter, repeat as repeat_iter, repeat_with as repeat_with_iter,
	successors,
};

pub use parse_bytes::*;

mod neighbors;
pub use neighbors::*;

mod multi_parse;
pub use multi_parse::*;

mod input_data;
pub use input_data::*;

mod min_heap;
pub use min_heap::*;

/// Computes the triangular number.
///
/// # Example
/// ```ignore
/// # use aoc2022::helpers::triangular_number;
/// for (n, ans) in [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55].into_iter().enumerate() {
///     assert_eq!(triangular_number(n), ans);
/// }
/// ```
pub fn triangular_number<N>(n: N) -> N
where
	N: Add<Output = N> + Mul<Output = N> + Div<Output = N> + From<u8> + Copy,
{
	n * (n + 1u8.into()) / 2u8.into()
}

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

pub fn add<T>(a: [T; 2], b: [T; 2]) -> [T; 2]
where
	T: Add<Output = T>,
{
	let [a1, a2] = a;
	let [b1, b2] = b;
	[a1 + b1, a2 + b2]
}

pub fn get_2d<T>(map: &[Vec<T>], point: [isize; 2]) -> Option<&T> {
	map.get(point[0] as usize)
		.and_then(|row| row.get(point[1] as usize))
}

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

pub use crate::dbg_small;
/// New dbg macro modified from [`std::dbg`] that doesn't use alternate form.
#[macro_export]
macro_rules! dbg_small {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg_small!($val)),+,)
    };
}
