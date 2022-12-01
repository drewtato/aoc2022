use std::process::exit;

use aoc2022::runner::Settings;
use clap::Parser;

fn main() {
	let mut settings = Settings::parse();
	if let Err(e) = settings.run() {
		eprintln!("{e}");
		exit(1);
	}
}
