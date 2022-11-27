use aoc2022::runner::Settings;
use aoc2022::Res;
use clap::Parser;

fn main() -> Res<()> {
	let mut settings = Settings::parse();
	settings.run()
}
