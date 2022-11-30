use std::str::FromStr;

use atoi::FromRadix10SignedChecked;

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

impl ParseBytes for &[u8] {
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
