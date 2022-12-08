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

	/// Runs a closure on each line of input. The closure is given a slice that starts at the
	/// beginning of a line and goes all the way to the end of the input. The closure returns how
	/// many characters to skip forward so that this function doesn't have to recheck those for a
	/// newline byte. It also allows it to consume multiple lines if necessary. If the closure
	/// returns `Ok(skip)`, `consume_lines` will skip forward that much and then skip until a
	/// newline is found. If the closure returns `Err(skip)` for the skip count, it means to only
	/// skip forward that much without advancing more for newline. This is useful if the newline has
	/// been found inside the closure, or if the closure doesn't want to skip an entire line.
	fn consume_lines<F>(&self, f: F)
	where
		F: FnMut(&[u8]) -> Result<usize, usize>;
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

	fn consume_lines<F>(&self, mut f: F)
	where
		F: FnMut(&[u8]) -> Result<usize, usize>,
	{
		let mut current = self;
		loop {
			let skip = f(current);
			match advance_to_newline(current, skip) {
				Some(new_current) => current = new_current,
				None => break,
			}
		}
	}
}

fn advance_to_newline(mut current: &[u8], skip: Result<usize, usize>) -> Option<&[u8]> {
	match skip {
		Ok(s) => current = current.get(s..)?,
		Err(s) => return current.get(s..),
	}

	loop {
		let &first = current.take_first()?;
		if first == b'\n' {
			break;
		}
	}

	Some(current)
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

#[cfg(test)]
mod tests {
	use itertools::Itertools;

	use super::*;

	#[test]
	fn consume_lines() {
		let data = b"hello\n123   \n123\nyes\n";
		let mut res = Vec::new();
		data.consume_lines(|line| {
			let line_stuff = line
				.iter()
				.copied()
				.take_while(|b| !b.is_ascii_whitespace())
				.collect_vec();
			let len = line_stuff.len();
			res.push(line_stuff);
			if line.get(len) == Some(&b'\n') {
				Err(len + 1)
			} else {
				Ok(len)
			}
		});
		assert_eq!(
			res.as_slice(),
			&[
				b"hello".to_vec(),
				b"123".to_vec(),
				b"123".to_vec(),
				b"yes".to_vec(),
				b"".to_vec(),
			]
		);
	}
}
