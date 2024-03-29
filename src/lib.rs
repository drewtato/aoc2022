#![feature(type_alias_impl_trait)]
#![feature(array_windows)]
#![feature(array_try_from_fn)]
#![feature(byte_slice_trim_ascii)]
#![feature(slice_take)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]
#![feature(split_as_slice)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![allow(clippy::uninlined_format_args)]

pub const YEAR: u32 = 2022;
pub type Res<T> = Result<T, AocError>;
pub mod solution;
pub use solution::Solver;
mod error;
pub use error::AocError;
pub mod helpers;
pub mod runner;

pub mod days;
