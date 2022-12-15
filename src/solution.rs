use std::fmt::{Display, Write};

use std::time::Duration;

use crate::runner::time_fn;
use crate::Res;

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
	fn initialize(file: Vec<u8>, dbg: u8) -> Self;

	/// Runs part one. This will always be called after [`initialize`](Solver::initialize).
	fn part_one(&mut self, dbg: u8) -> Self::AnswerOne;

	/// Runs part two. This will always be called after [`initialize`](Solver::initialize).
	fn part_two(&mut self, dbg: u8) -> Self::AnswerTwo;

	/// Runs parts other than one and two, and writes the result plus a newline into a writer. This
	/// will always be called after [`initialize`](Solver::initialize) and won't include `1` or `2`.
	///
	/// Returns `Err(())` if this part is unimplemented.
	fn run_any<W: Write>(&mut self, part: u32, writer: W, dbg: u8) -> Res<Duration>;

	/// Runs parts one and two. This includes a call to [`initialize`](Solver::initialize). This
	/// will be used for full benchmarking.
	fn run_both(file: Vec<u8>, dbg: u8) -> (Self::AnswerOne, Self::AnswerTwo) {
		let mut sol = Self::initialize(file, dbg);
		(sol.part_one(dbg), sol.part_two(dbg))
	}

	/// Same as `run_both` but returns timing info and results as strings.
	fn run_both_string(file: Vec<u8>, dbg: u8) -> (Duration, String, String) {
		let (time, (p1, p2)) = time_fn(|| Self::run_both(file, dbg));
		(time, p1.to_string(), p2.to_string())
	}
}

/// Object-safe version of [`Solver`].
pub trait SolverSafe {
	/// Runs part one. This will always be called after [`initialize`](Solver::initialize).
	fn part_one(&mut self, dbg: u8, writer: &mut String) -> Duration;

	/// Runs part two. This will always be called after [`initialize`](Solver::initialize).
	fn part_two(&mut self, dbg: u8, writer: &mut String) -> Duration;

	/// Runs parts other than one and two, and writes the result plus a newline into a writer. This
	/// will always be called after [`initialize`](Solver::initialize) and won't include `1` or `2`.
	///
	/// Returns `Err(())` if this part is unimplemented.
	fn run_any(&mut self, part: u32, dbg: u8, writer: &mut String) -> Res<Duration>;
}

impl<T> SolverSafe for T
where
	T: Solver,
{
	fn part_one(&mut self, dbg: u8, writer: &mut String) -> Duration {
		let (time, a1) = time_fn(|| self.part_one(dbg));
		write!(writer, "{a1}").unwrap();
		time
	}

	fn part_two(&mut self, dbg: u8, writer: &mut String) -> Duration {
		let (time, a2) = time_fn(|| self.part_two(dbg));
		write!(writer, "{a2}").unwrap();
		time
	}

	fn run_any(&mut self, part: u32, dbg: u8, writer: &mut String) -> Res<Duration> {
		self.run_any(part, writer, dbg)
	}
}
