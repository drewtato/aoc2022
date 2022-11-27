use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AocError {
	PartNotFound,
}

impl Display for AocError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Part not found")
	}
}

impl Error for AocError {}
