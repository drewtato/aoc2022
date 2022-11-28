use std::array;
use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

use super::{FromBytes, ParseBytes};

/// Error type for [`multi_parse`](MultiParseBytes::multi_parse).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiParseError {
	NotEnoughItems,
	TooManyItems,
	ParseError,
	NotUtf8,
}

impl std::error::Error for MultiParseError {}

impl Display for MultiParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			MultiParseError::NotEnoughItems => "Not enough items in multiparse",
			MultiParseError::TooManyItems => "Too many items in multiparse",
			MultiParseError::ParseError => "Parse error in multiparse",
			MultiParseError::NotUtf8 => "Slice was not valid UTF-8",
		})
	}
}

/// Trait that parses iterators into collections.
///
/// This works on any collection that implements [`MultiFromBytes`]. These include `Vec`, arrays, and
/// tuples of length 0 to 12, when the item type or types in those collections implement
/// [`FromBytes`]. This trait and `MultiFromBytes` are analogous to [`str::parse`] and [`FromStr`](std::str::FromStr).
///
/// # Examples
///
/// With a [`Vec`]:
/// ```
/// # use aoc2022::helpers::{MultiParseBytes, is};
/// let vec: Vec<usize> = b"1,2,3".split(is(&b',')).multi_parse().unwrap();
/// assert_eq!(vec, vec![1, 2, 3]);
/// ```
///
/// With an [prim@array]:
/// ```
/// # use aoc2022::helpers::{MultiParseBytes, is};
/// let arr: [usize; 3] = b"1,2,3".split(is(&b',')).multi_parse().unwrap();
/// assert_eq!(arr, [1, 2, 3]);
/// ```
///
/// With a [tuple]:
/// ```
/// # use aoc2022::helpers::{MultiParseBytes, is};
/// let tup: (usize, usize, usize) = b"1,2,3".split(is(&b',')).multi_parse().unwrap();
/// assert_eq!(tup, (1, 2, 3));
/// ```
///
/// Tuples can also have elements with different types:
/// ```
/// # use aoc2022::helpers::{MultiParseBytes, is};
/// let tup: (usize, String, f32) = b"1,2,3".split(is(&b',')).multi_parse().unwrap();
/// assert_eq!(tup, (1, "2".to_string(), 3.0));
/// ```
///
/// And both tuples and arrays can be immediately destructured. Usually if you pass these to (or
/// return them from) a function at some point, their types can be inferred completely:
/// ```
/// # use aoc2022::helpers::{MultiParseBytes, is};
/// let (a, b, c): (char, f64, u8) = b"1,2,3".split(is(&b',')).multi_parse().unwrap();
/// assert!(a == '1' && b == 2.0 && c == 3);
///
/// let [a, b, c]: [i128; 3] = b"1,2,3".split(is(&b',')).multi_parse().unwrap();
/// assert_eq!(a - b + c, 2);
/// ```
pub trait MultiParseBytes {
	fn multi_parse<T>(self) -> Result<T, MultiParseError>
	where
		T: MultiFromBytes;
}

impl<I: IntoIterator<Item = S>, S: AsRef<[u8]>> MultiParseBytes for I {
	/// Returns a collection of parsed values built from the iterator.
	///
	/// If the iterator did not fit exactly into the collection, or if the parse failed, this
	/// returns an error. For arbitrary-length collections such as [Vec], only parse errors are
	/// encountered.
	fn multi_parse<T>(self) -> Result<T, MultiParseError>
	where
		T: MultiFromBytes,
	{
		MultiFromBytes::multi_from_bytes(self.into_iter())
	}
}

/// Trait allowing a type to be built from an iterator of [`u8`].
///
/// It is best to implement this trait for collections, and then use
/// [`multi_parse`](MultiParseBytes::multi_parse) to invoke it. This is analogous to [`FromStr`](std::str::FromStr) and
/// [`str::parse`].
///
/// # Examples
/// ```
/// # use aoc2022::helpers::{MultiFromBytes, is};
/// let tup: (u16, String) = MultiFromBytes::multi_from_bytes(b"3,hello".split(is(&b','))).unwrap();
/// assert_eq!(tup, (3, "hello".to_string()));
/// ```
///
/// Here is the same thing, but using [`multi_parse`](MultiParseBytes::multi_parse):
/// ```
/// # use aoc2022::helpers::{MultiParseBytes, is};
/// let tup: (u16, String) = b"3,hello".split(is(&b',')).multi_parse().unwrap();
/// assert_eq!(tup, (3, "hello".to_string()));
/// ```
pub trait MultiFromBytes {
	fn multi_from_bytes<I, S>(iter: I) -> Result<Self, MultiParseError>
	where
		Self: Sized,
		I: IntoIterator<Item = S>,
		S: AsRef<[u8]>;
}

macro_rules! impl_multi_from_bytes_collection {
	($t:ty => $($gens:tt)*) => {
		impl<$($gens)*> MultiFromBytes for $t {
			fn multi_from_bytes<I, S>(iter: I) -> Result<Self, MultiParseError>
			where
				Self: Sized,
				I: IntoIterator<Item = S>,
				S: AsRef<[u8]>,
			{
				iter.into_iter()
					.map(|s| s.as_ref().parse().ok_or(MultiParseError::ParseError))
					.collect()
			}
		}
	};
}

impl_multi_from_bytes_collection! { Vec<T> => T: FromBytes }
impl_multi_from_bytes_collection! {	VecDeque<T> => T: FromBytes }
impl_multi_from_bytes_collection! {	BTreeSet<T> => T: FromBytes + Ord }
impl_multi_from_bytes_collection! {	BinaryHeap<T> => T: FromBytes + Ord }
impl_multi_from_bytes_collection! {	LinkedList<T> => T: FromBytes }
impl_multi_from_bytes_collection! {	HashSet<T> => T: FromBytes + Eq + Hash }

impl<T: FromBytes, const N: usize> MultiFromBytes for [T; N] {
	/// An array of any length can be built with [`multi_parse`](MultiParse::multi_parse).
	fn multi_from_bytes<I, S>(iter: I) -> Result<Self, MultiParseError>
	where
		Self: Sized,
		I: IntoIterator<Item = S>,
		S: AsRef<[u8]>,
	{
		let mut iter = iter.into_iter();
		array::try_from_fn(|_| {
			iter.next()
				.ok_or(MultiParseError::NotEnoughItems)
				.and_then(|s| s.as_ref().parse().ok_or(MultiParseError::ParseError))
		})
	}
}

macro_rules! tuple_multi_from_bytes_impl {
	($($i:ident,)*) => {
		impl<$($i,)*> MultiFromBytes for ($($i,)*)
		where
			$($i: FromBytes,)*
		{
			fn multi_from_bytes<I, S>(iter: I) -> Result<Self, MultiParseError>
			where
				Self: Sized,
				I: IntoIterator<Item = S>,
				S: AsRef<[u8]>,
			{
				let mut iter = iter.into_iter();
				let tup = ($(
					{
						iter.next()
							.ok_or(MultiParseError::NotEnoughItems)?
							.as_ref()
							// This turbofish is unnecessary for the generated code, but $i needs to be in here
							// somewhere so the macro knows what to repeat.
							.parse::<$i>()
							.ok_or(MultiParseError::ParseError)?
					},
				)*);
				if iter.next().is_some() {
					return Err(MultiParseError::TooManyItems);
				}
				Ok(tup)
			}
		}
	};
}

tuple_multi_from_bytes_impl!();
tuple_multi_from_bytes_impl!(A0,);
tuple_multi_from_bytes_impl!(A0, A1,);
tuple_multi_from_bytes_impl!(A0, A1, A2,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4, A5,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4, A5, A6,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4, A5, A6, A7,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,);
tuple_multi_from_bytes_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,);
