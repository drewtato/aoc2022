use std::fmt::{Display, Write};

use crate::AocError;

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
	fn run_any(&mut self, part: u32) -> Result<String, AocError> {
		let mut s = String::new();
		self.run_any_write(part, &mut s)?;
		Ok(s)
	}
	/// Runs parts other than one and two, and writes the result plus a newline into a writer. This
	/// will always be called after [`initialize`](Solver::initialize) and won't include `1` or `2`.
	///
	/// Returns `Err(())` if this part is unimplemented.
	fn run_any_write<W: Write>(&mut self, part: u32, writer: W) -> Result<(), AocError>;
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
	fn run_any_dbg(&mut self, part: u32, debug: u8) -> Result<String, AocError> {
		self.run_any(part)
	}
	/// Same as [`run_all`](Solver::run_all) but takes the debug flag. Runs `run_all` by default.
	fn run_all_dbg(file: Vec<u8>, debug: u8) -> (Self::AnswerOne, Self::AnswerTwo) {
		let mut sol = Self::initialize_dbg(file, debug);
		(sol.part_one_dbg(debug), sol.part_two_dbg(debug))
	}
	/// Same as [`run_any_write`](Solver::run_any_write) but takes the debug flag. Runs
	/// `run_any_write` by default.
	fn run_any_write_dbg<W: Write>(&mut self, part: u32, writer: W) -> Result<(), AocError> {
		self.run_any_write(part, writer)
	}
}
