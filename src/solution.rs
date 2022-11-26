use std::fmt::Display;

use std::iter::Filter;

use std::slice::Split;

/// Trait to be implemented for each day.
#[allow(unused_variables)]
pub trait Solver: Sized {
	/// The type returned from part one.
	type AnswerOne: Sized + Display;
	/// The type returned from part two.
	type AnswerTwo: Sized + Display;

	/// Like [`Default`] but takes a file. Used to perform operations to prepare for part one or
	/// part two. This takes a [`Vec`] so that the buffer can be reused and modified. It will likely
	/// be stored in `Self`, depending on the prompt.
	fn initialize(file: Vec<u8>) -> Self;
	/// Runs part one. This will always be called after [`initialize`](Solver::initialize).
	fn part_one(&mut self) -> Self::AnswerOne;
	/// Runs part two. This will always be called after [`initialize`](Solver::initialize).
	fn part_two(&mut self) -> Self::AnswerTwo;
	/// Runs parts other than one and two. This will always be called after
	/// [`initialize`](Solver::initialize) and won't include `1` or `2`.
	///
	/// Returns `Err(())` if this part is unimplemented.
	fn run_any(&mut self, part: u32) -> Result<String, ()>;
	/// Runs parts one and two. This includes a call to [`initialize`](Solver::initialize). This
	/// will be used for full benchmarking.
	fn run_all(file: Vec<u8>) -> (Self::AnswerOne, Self::AnswerTwo) {
		let mut sol = Self::initialize(file);
		(sol.part_one(), sol.part_two())
	}

	/// Same as [`initialize`](Solver::initialize) but takes the debug flag. Runs `initialize` by
	/// default.
	fn initialize_dbg(file: Vec<u8>, debug: u8) -> Self {
		Self::initialize(file)
	}
	/// Same as [`part_one`](Solver::part_one) but takes the debug flag. Runs `part_one` by default.
	fn part_one_dbg(&mut self, debug: u8) -> Self::AnswerOne {
		self.part_one()
	}
	/// Same as [`part_two`](Solver::part_two) but takes the debug flag. Runs `part_two` by default.
	fn part_two_dbg(&mut self, debug: u8) -> Self::AnswerTwo {
		self.part_two()
	}
	/// Same as [`run_any`](Solver::run_any) but takes the debug flag. Runs `run_any` by default.
	fn run_any_dbg(&mut self, part: u32, debug: u8) -> Result<String, ()> {
		self.run_any(part)
	}
	/// Same as [`run_all`](Solver::run_all) but takes the debug flag. Runs `run_all` by default.
	fn run_all_dbg(file: Vec<u8>, debug: u8) -> (Self::AnswerOne, Self::AnswerTwo) {
		let mut sol = Self::initialize_dbg(file, debug);
		(sol.part_one_dbg(debug), sol.part_two_dbg(debug))
	}
}

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
	/// Returns a 2D grid of the input after transforming with the provided closure. This will be a
	/// fully rectangular grid, so all the lengths of the inner [`Vec`]s will be the same.
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
