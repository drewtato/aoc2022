use std::iter::Filter;
use std::slice::Split;

/// Type of the grid returned by [`InputData::grid`].
pub type Grid<G> = Vec<Vec<G>>;

/// A value that can be used as input. This is usually either [`Vec<u8>`] or [`&[u8]`](slice).
pub trait InputData<'a> {
	/// Return type of [`lines`](InputData::lines).
	type Lines: Iterator<Item = &'a [u8]>;
	/// Return type of [`words`](InputData::words).
	type Words: Iterator<Item = &'a [u8]>;

	/// Returns an iterator over slices between `\n` bytes. Does not include the `\n` byte.
	fn lines(&'a self) -> Self::Lines;
	/// Returns an iterator over slices between whitespace. Does not include whitespace.
	fn words(&'a self) -> Self::Words;
	/// Returns a 2D grid of the input after transforming with the provided closure. This will
	/// be a fully rectangular grid, so all the lengths of the inner [`Vec`]s will be the same.
	fn grid<G, F>(&self, f: F) -> Grid<G>
	where
		F: FnMut(u8) -> G,
		G: Default + Clone;
}

impl<'a> InputData<'a> for [u8] {
	type Lines = Split<'a, u8, fn(&u8) -> bool>;
	type Words = Filter<Split<'a, u8, fn(&u8) -> bool>, fn(&&[u8]) -> bool>;

	fn lines(&'a self) -> Self::Lines {
		self.split(byte_is_newline)
	}

	fn words(&'a self) -> Self::Words {
		self.split(byte_is_ascii_whitespace as _)
			.filter(slice_is_not_empty)
	}

	fn grid<G, F>(&self, mut f: F) -> Grid<G>
	where
		F: FnMut(u8) -> G,
		G: Default + Clone,
	{
		let mut grid: Grid<G> = self
			.lines()
			.map(|line| line.iter().map(|&byte| f(byte)).collect())
			.collect();

		let max = grid.iter().map(|v| v.len()).max().unwrap_or_default();
		for row in &mut grid {
			row.resize(max, Default::default());
		}

		grid
	}
}

fn byte_is_newline(byte: &u8) -> bool {
	*byte == b'\n'
}

fn byte_is_ascii_whitespace(byte: &u8) -> bool {
	byte.is_ascii_whitespace()
}

/// Necessary for higher-ranked lifetime error when using closure instead
fn slice_is_not_empty(s: &&[u8]) -> bool {
	!s.is_empty()
}
