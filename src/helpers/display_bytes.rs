use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplaySlice<'a>(&'a [u8]);

impl<'a> Display for DisplaySlice<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = std::str::from_utf8(self.0).unwrap();
		write!(f, "{s}")
	}
}

impl<'a> Debug for DisplaySlice<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = std::str::from_utf8(self.0).unwrap();
		write!(f, "{s:?}")
	}
}

impl<'a> DisplaySlice<'a> {
	pub fn new(s: &'a [u8]) -> Self {
		Self(s)
	}
}

pub trait ToDisplaySlice {
	fn to_display_slice(&self) -> DisplaySlice<'_>;
}

impl ToDisplaySlice for [u8] {
	fn to_display_slice(&self) -> DisplaySlice<'_> {
		DisplaySlice(self)
	}
}

impl ToDisplaySlice for Vec<u8> {
	fn to_display_slice(&self) -> DisplaySlice<'_> {
		DisplaySlice(self.as_slice())
	}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayByte(u8);

impl Display for DisplayByte {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0 as char)
	}
}

impl Debug for DisplayByte {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0 as char)
	}
}

impl DisplayByte {
	pub fn new(b: u8) -> Self {
		Self(b)
	}
}

pub trait ToDisplayByte {
	fn to_display_byte(self) -> DisplayByte;
}

impl ToDisplayByte for u8 {
	fn to_display_byte(self) -> DisplayByte {
		DisplayByte(self)
	}
}
