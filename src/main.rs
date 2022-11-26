#![feature(type_alias_impl_trait)]
#![feature(array_windows)]

const YEAR: u32 = 2022;

use std::error::Error;

use clap::Parser;

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
	let mut settings = Settings::parse();
	settings.run()
}

mod runner;
use runner::Settings;

mod solution;
use solution::{Grid, InputData, Solver};

mod helpers;

mod days;
