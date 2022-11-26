use clap::{ArgAction, Parser};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::fs::{create_dir_all, read_to_string, File};
// use std::fmt::Write;
use std::io::{self, stdout, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::solution::Solver;
use crate::{Res, YEAR};

/// Settings for running AoC. Usually created with [`clap::Parser::parse`].
#[derive(Debug, Parser)]
#[command(author, about)]
pub struct Settings {
	/// Specify which days to run. Passing 0 will run all 25.
	///
	/// To run a specific part, pass `day.part`, like `2.1` for part 1 of day 2, or `2.1.2` for
	/// both parts of day 2 (same as `2`).
	pub days: Vec<String>,

	/// Benchmark this run. Runs once, unless a number of runs is given. Saves output until
	/// everything is finished.
	#[arg(long, short, default_value_t = 0, default_missing_value = "1")]
	pub bench: u32,

	// /// Runs days in parallel.
	// #[arg(long, short)]
	// pub parallel: bool,
	/// Enables debug mode for the days.
	///
	/// Pass this flag multiple times to enable more debug info.
	#[arg(short, long, action = ArgAction::Count)]
	pub debug: u8,

	/// Run with the specified test input. Best used with one day selected.
	#[arg(short, long, default_value_t = 0)]
	pub test: u32,

	/// Enables debug info for the runner.
	#[arg(short, long, action = ArgAction::Count)]
	pub runner_debug: u8,

	#[arg(skip = None)]
	pub client: Option<Client>,
	#[arg(skip = None)]
	pub regex: Option<Regex>,
}

impl Settings {
	/// Run AoC with these settings.
	pub fn run(&mut self) -> Res<()> {
		let &mut Self {
			ref days,
			bench,
			// parallel,
			runner_debug,
			..
		} = self;
		let runner_time = Instant::now();

		if bench > 0 {
			if runner_debug > 0 {
				eprintln!("Running benchmark with {bench} cycles");
			}
			self.bench()?;

			if runner_debug > 0 {
				eprintln!("Finished benchmark");
			}
		}

		if days.is_empty() {
			eprintln!("No days specified. Use `--help` for more.");
			return Ok(());
		}

		if runner_debug > 0 {
			eprintln!("Parsing day arguments");
		}
		let days: Vec<_> = days
			.iter()
			.map(Self::parse_day_arg)
			.collect::<Result<_, _>>()?;

		if runner_debug > 0 {
			eprintln!("Running days");
		}

		let mut times = Duration::ZERO;
		for &(day, ref parts) in &days {
			if day == 0 {
				if runner_debug > 1 {
					eprintln!("Running all days");
				}
				for day in 1..=25 {
					times += self.run_day(day, parts)?;
				}
				if runner_debug > 1 {
					eprintln!("Finished all days");
				}
			} else {
				if runner_debug > 1 {
					eprintln!("Running day {day} with parts {parts:?}");
				}
				times += self.run_day(day, parts)?;
			}
		}

		if days.len() > 1 {
			println!("Time for all days: {times:?}",);
		}

		if runner_debug > 0 {
			let elapsed = runner_time.elapsed();
			eprintln!(
				"Whole time: {:?} (only runner: {:?})",
				elapsed,
				elapsed - times
			);
		}

		Ok(())
	}

	/// Run the specified parts of a single day and returns the time taken.
	pub fn run_day(&mut self, day: u32, parts: &[u32]) -> Res<Duration> {
		let solver = {
			use crate::days::*;
			match day {
				1 => Self::solver::<day01::Solution>,
				2 => Self::solver::<day02::Solution>,
				3 => Self::solver::<day03::Solution>,
				4 => Self::solver::<day04::Solution>,
				5 => Self::solver::<day05::Solution>,
				6 => Self::solver::<day06::Solution>,
				7 => Self::solver::<day07::Solution>,
				8 => Self::solver::<day08::Solution>,
				9 => Self::solver::<day09::Solution>,
				10 => Self::solver::<day10::Solution>,
				11 => Self::solver::<day11::Solution>,
				12 => Self::solver::<day12::Solution>,
				13 => Self::solver::<day13::Solution>,
				14 => Self::solver::<day14::Solution>,
				15 => Self::solver::<day15::Solution>,
				16 => Self::solver::<day16::Solution>,
				17 => Self::solver::<day17::Solution>,
				18 => Self::solver::<day18::Solution>,
				19 => Self::solver::<day19::Solution>,
				20 => Self::solver::<day20::Solution>,
				21 => Self::solver::<day21::Solution>,
				22 => Self::solver::<day22::Solution>,
				23 => Self::solver::<day23::Solution>,
				24 => Self::solver::<day24::Solution>,
				25 => Self::solver::<day25::Solution>,
				d => {
					println!("Invalid day {d}, skipping.");
					return Ok(Duration::ZERO);
				}
			}
		};

		let input = Self::get_input(self, day)?;

		print!("Initializing day {day}... ");
		stdout().flush()?;

		let times = solver(self, input, day, parts)?;

		println!("Time for day {day}: {times:?}\n");

		Ok(times)
	}

	/// Get an input from the filesystem, or if it's not there, from the network.
	pub fn get_input(&mut self, day: u32) -> Res<Vec<u8>> {
		let runner_debug = self.runner_debug;
		let test = self.test;
		let input_name = if test == 0 {
			format!("./inputs/day{day:02}/input.txt")
		} else {
			format!("./inputs/day{day:02}/input{test}.txt")
		};
		if runner_debug > 0 {
			eprintln!("Reading input from {input_name}");
		}

		let input_path = PathBuf::from(input_name.clone());
		let buf = if !input_path.exists() {
			if test == 0 {
				if runner_debug > 0 {
					eprintln!("The input file does not exist, fetching input from network");
				}
				Self::get_input_network(self, day, &input_path)?
			} else {
				return Err(format!("No test input at `{input_name}`").into());
			}
		} else {
			let mut file = File::open(input_path)?;
			let mut buf = Vec::new();
			file.read_to_end(&mut buf)?;
			buf
		};

		Ok(buf)
	}

	/// Get the input from the network and write it to the filesystem. Will overwrite any existing
	/// input files.
	pub fn get_input_network(&mut self, day: u32, path: &Path) -> Res<Vec<u8>> {
		let api_key = read_to_string("./API_KEY")?;
		let api_key = api_key.trim();

		// Get main input
		let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");
		if self.runner_debug > 0 {
			eprintln!("Fetching {url}");
		}
		let client = self.client.get_or_insert_with(Default::default);
		let req = client
			.get(url)
			.header(COOKIE, format!("session={api_key}"))
			.send()?;
		let status = req.status();
		if !status.is_success() {
			eprintln!(
				"Couldn't fetch input from network. Status: {}\nContent:\n{}",
				status,
				req.text()?
			);
			return Err("Failed to make network request.".into());
		}
		let data = req.bytes()?.to_vec();

		create_dir_all(path.parent().unwrap())?;
		File::create(path)?.write_all(&data)?;

		// Get prompt and test cases
		let url = format!("https://adventofcode.com/{YEAR}/day/{day}");
		if self.runner_debug > 0 {
			eprintln!("Fetching {url}");
		}
		let req = client
			.get(url)
			// .header(COOKIE, format!("session={api_key}"))
			.send()?;
		if !req.status().is_success() {
			let msg = format!(
				"Couldn't fetch prompt from network. Status {}, content:\n{}",
				req.status(),
				req.text()?
			);
			return Err(msg.into());
		}
		let text = req.text()?;

		// Save prompt
		let prompt_path = path.parent().unwrap().join("prompt.html");
		File::create(prompt_path)?.write_all(text.as_bytes())?;

		// Save each code block as a test case
		let regex = self
			.regex
			.get_or_insert_with(|| Regex::new(r"<pre>\s*<code>([^<]+)</code>\s*</pre>").unwrap());
		for (i, code) in regex.captures_iter(&text).enumerate() {
			let i = i + 1;
			let code = &code[1];
			let test_path = path.parent().unwrap().join(format!("input{i}.txt"));
			let file = File::create(test_path)?;
			let mut file = BufWriter::new(file);
			html_escape::decode_html_entities_to_writer(code, &mut file)?;
		}

		Ok(data)
	}

	/// Run the solver for a day and print info. Returns the time taken to solve.
	pub fn solver<S: Solver>(
		&self,
		file: Vec<u8>,
		day: u32,
		parts: &[u32],
	) -> Result<Duration, io::Error> {
		let debug = self.debug;
		let (init_time, mut sol) =
			time_fn(|| crate::days::day01::Solution::initialize_dbg(file, debug));
		println!("took {:?}", init_time);

		Ok(if parts.is_empty() {
			print!("Running day {day} part 1... ");
			stdout().flush()?;

			let (p1_time, p1) = time_fn(|| sol.part_one_dbg(debug));

			println!("took {p1_time:?}",);
			println!("d{day:02}p1: {p1}",);

			print!("Running day {day} part 2... ");
			stdout().flush()?;

			let (p2_time, p2) = time_fn(|| sol.part_two_dbg(debug));

			println!("took {p2_time:?}");
			println!("d{day:02}p2: {p2}");

			init_time + p1_time + p2_time
		} else {
			let mut times = Duration::ZERO;
			for &part in parts {
				print!("Running day {day} part {part}... ");
				stdout().flush()?;

				let (time, ans) = match part {
					1 => {
						let (time, ans) = time_fn(|| sol.part_one_dbg(debug));
						(time, ans.to_string())
					}
					2 => {
						let (time, ans) = time_fn(|| sol.part_two_dbg(debug));
						(time, ans.to_string())
					}
					p => {
						if let (time, Ok(s)) = time_fn(|| sol.run_any_dbg(p, debug)) {
							(time, s)
						} else {
							println!("Day {day} did not include a part {p}, skipping.");
							continue;
						}
					}
				};

				println!("took {time:?}");
				println!("d{day:02}p{part}: {ans}");
				times += time;
			}
			times
		})
	}

	/// Parses a single day command-line argument.
	pub fn parse_day_arg<S: AsRef<str>>(s: S) -> Result<(u32, Vec<u32>), String> {
		let s = s.as_ref();
		let mut parts = s.split('.').map(|part| {
			part.parse()
				.map_err(|_| format!("Could not parse `{part}` as integer in argument `{s}`"))
		});

		let day = parts
			.next()
			.ok_or_else(|| format!("No day specified in argument `{s}`"))??;
		let parts = parts.collect::<Result<_, _>>()?;
		Ok((day, parts))
	}

	/// Benchmark a day, or all if the day is 0. Will load all inputs from disk first, then runs
	/// all necessary parts in sequence and times it as a whole.
	pub fn bench(&self) -> Res<()> {
		todo!()
	}
}

/// Time a single function.
pub fn time_fn<F: FnOnce() -> T, T>(f: F) -> (Duration, T) {
	let start = Instant::now();
	let t = f();
	let end = start.elapsed();
	(end, t)
}
