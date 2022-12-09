use std::str::FromStr;

use atoi::{FromRadix10, FromRadix10Signed, FromRadix10SignedChecked};

/// Examples:
/// ```ignore
/// # use aoc2022::helpers::FromBytes;
/// let s: String = FromBytes::from_bytes(b"hello").unwrap();
/// assert_eq!("hello", &s);
/// ```
pub trait FromBytes: Sized {
	fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

pub trait ParseBytes {
	fn parse<I>(&self) -> Option<I>
	where
		I: FromBytes;
}

impl ParseBytes for [u8] {
	fn parse<I>(&self) -> Option<I>
	where
		I: FromBytes,
	{
		FromBytes::from_bytes(self)
	}
}

impl FromBytes for bool {
	fn from_bytes(bytes: &[u8]) -> Option<Self> {
		Some(match bytes {
			b"true" => true,
			b"false" => false,
			_ => return None,
		})
	}
}

macro_rules! from_bytes_through_str {
	($($t:ty),*) => {$(
		impl FromBytes for $t {
			fn from_bytes(bytes: &[u8]) -> Option<Self> {
				FromStr::from_str(std::str::from_utf8(bytes).ok()?).ok()
			}
		}
	)*};
}

from_bytes_through_str! { String, f32, f64, char }

macro_rules! from_bytes_integer {
	($($t:ty),*) => {$(
		impl FromBytes for $t {
			fn from_bytes(bytes: &[u8]) -> Option<Self> {
				FromRadix10SignedChecked::from_radix_10_signed_checked(bytes).0
			}
		}
	)*};
}

from_bytes_integer! { u8, u16, u32, u64, u128, usize }
from_bytes_integer! { i8, i16, i32, i64, i128, isize }

/// Parses an integer and modifies the slice to start after the integer. Returns 0 when no number
/// was found.
///
/// # Examples
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"".as_slice();
/// let n: u32 = parse_consume_unsigned(&mut s);
/// assert_eq!(n, 0);
/// assert_eq!(s.len(), 0);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"abc".as_slice();
/// let n: u32 = parse_consume_unsigned(&mut s);
/// assert_eq!(n, 0);
/// assert_eq!(s.len(), 3);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"123".as_slice();
/// let n: u32 = parse_consume_unsigned(&mut s);
/// assert_eq!(n, 123);
/// assert_eq!(s.len(), 0);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"123abc".as_slice();
/// let n: u32 = parse_consume_unsigned(&mut s);
/// assert_eq!(n, 123);
/// assert_eq!(s.len(), 3);
/// ```
pub fn parse_consume_unsigned<I>(s: &mut &[u8]) -> I
where
	I: FromRadix10,
{
	let (n, size) = FromRadix10::from_radix_10(s);
	*s = &s[size..];
	n
}
/// Parses an integer and modifies the slice to start after the integer. Returns 0 when no number
/// was found.
///
/// # Examples
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"abc".as_slice();
/// let n: u32 = parse_consume_signed(&mut s);
/// assert_eq!(n, 0);
/// assert_eq!(s.len(), 3);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"123".as_slice();
/// let n: u32 = parse_consume_signed(&mut s);
/// assert_eq!(n, 123);
/// assert_eq!(s.len(), 0);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"123abc".as_slice();
/// let n: u32 = parse_consume_signed(&mut s);
/// assert_eq!(n, 123);
/// assert_eq!(s.len(), 3);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"-123abc".as_slice();
/// let n: i32 = parse_consume_signed(&mut s);
/// assert_eq!(n, -123);
/// assert_eq!(s.len(), 3);
/// ```
///
/// ```ignore
/// # use aoc2022::helpers::*;
/// let mut s = b"+123abc".as_slice();
/// let n: u32 = parse_consume_signed(&mut s);
/// assert_eq!(n, 123);
/// assert_eq!(s.len(), 3);
/// ```
pub fn parse_consume_signed<I>(s: &mut &[u8]) -> I
where
	I: FromRadix10Signed,
{
	let (n, size) = FromRadix10Signed::from_radix_10_signed(s);
	*s = &s[size..];
	n
}
