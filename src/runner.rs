use chrono::{Duration as ChDuration, FixedOffset, NaiveDate, Utc};
use clap::{ArgAction, Parser, ValueEnum};
use regex::bytes::Regex;
use reqwest::blocking::Client;

use std::fmt::Display;
use std::fs::{create_dir_all, File};
use std::hint::black_box;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::solution::SolverSafe;
use crate::{AocError, Res, Solver, YEAR};

/// User agent (see [Eric's post on the
/// subreddit](https://www.reddit.com/r/adventofcode/comments/z9dhtd))
const USER_AGENT: &str = "\
	drewtato-aoc-runner-2022 \
	at github.com/drewtato/aoc2022 \
	by 15526875+drewtato@users.noreply.github.com\
";

/// Settings for running AoC. Usually created with [`clap::Parser::parse`].
#[derive(Debug, Parser, Clone)]
#[command(author, about)]
pub struct Settings {
	/// Specify which days to run.
	///
	/// Passing 0 will run all 25. To run a specific part, pass `day.part`, like `2.1` for part 1
	/// of day 2, or `2.1.2` for both parts of day 2 (same as `2`).
	pub days: Vec<String>,

	/// Select which mode to run in.
	#[arg(short, long, value_enum, default_value_t = Mode::Run)]
	pub mode: Mode,

	/// Specify a number of milliseconds.
	///
	/// Overridden by `--bench-count` if nonzero. When in bench mode, you can specify how long to
	/// repeatedly run each day. This runs for one second by default.
	#[arg(short = 's', long = "bench-time", default_value_t = 1000)]
	pub bench_time: u64,

	/// Specify a number of iterations.
	///
	/// Overrides `--bench-time`. When in bench mode, specify to do a set number of iteratons
	/// instead of running as many as possible in a certain amount of time.
	#[arg(short = 'c', long = "bench-count", default_value_t = 0)]
	pub bench_count: usize,

	/// Hide answers in output.
	#[arg(short = 'a', long)]
	pub hide_answers: bool,

	// /// Runs days in parallel.
	// #[arg(long, short)]
	// pub parallel: bool,
	/// Enables debug mode for the days.
	///
	/// Pass this flag multiple times to enable more debug info.
	#[arg(short, long, action = ArgAction::Count)]
	pub debug: u8,

	/// Run with the specified test input.
	///
	/// Best used with one day selected. 0 corresponds to the real input.
	#[arg(short, long, default_value_t = 0)]
	pub test: u8,

	/// Enables debug info for the runner.
	#[arg(short, long, action = ArgAction::Count)]
	pub runner_debug: u8,

	#[arg(skip = None)]
	pub client: Option<Client>,
	#[arg(skip = None)]
	pub regex: Option<Regex>,
}

/// Mode to run [`Settings`] in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, ValueEnum)]
pub enum Mode {
	#[default]
	Run,
	/// Run the specified days and generate their output.
	R,
	Bench,
	/// Benchmark the specified days.
	///
	/// This ignores part info and will always run initialization, part one, and part two as fast
	/// as possible.
	B,
	Save,
	/// Save the specified days' output as validation files, to be used with `--validate`.
	S,
	Validate,
	/// Validate that the output of the specified days equals the saved output in validation files.
	V,
}

macro_rules! debug_println {
	($dbg:expr, $level:expr, $($tok:expr),*$(,)?) => {
		if $dbg >= $level {
			eprintln!($($tok),*);
		}
	};
}

impl Settings {
	pub fn run(&mut self) -> Res<()> {
		let runner_time = Instant::now();
		let mut solver_time = Duration::ZERO;

		debug_println!(self.runner_debug, 2, "{:?}", self);
		debug_println!(self.runner_debug, 1, "Starting runner");

		let day_parts = self
			.days
			.iter()
			.map(|word| parse_day(word))
			.collect::<Res<Vec<_>>>()?;

		match self.mode {
			Mode::Run | Mode::R => solver_time += self.run_days(&day_parts)?,
			Mode::Bench | Mode::B => solver_time += self.benchmark(&day_parts)?,
			Mode::Save | Mode::S => todo!(),
			Mode::Validate | Mode::V => todo!(),
		}

		let runner_time = runner_time.elapsed();
		debug_println!(
			self.runner_debug,
			1,
			"Total time: {:?}\nRunner time: {:?}",
			runner_time,
			runner_time - solver_time,
		);
		Ok(())
	}

	fn run_days(&mut self, day_parts: &[(u32, Vec<u32>)]) -> Res<Duration> {
		let mut test_time = Duration::ZERO;
		let mut buffer = String::new();
		for &(day, ref parts) in day_parts {
			debug_println!(self.runner_debug, 1, "Starting day {day}");

			let mut day_time = Duration::ZERO;

			if !(1..=25).contains(&day) {
				eprintln!("Day {day} not found, skipping");
				continue;
			}

			let file = self.get_input(day)?;
			let (time, mut solver) = day_to_solver(day, file, self.debug)?;
			print_times(day, 0, "", time);
			day_time += time;

			if parts.is_empty() {
				let time = solver.part_one(self.debug, &mut buffer);
				day_time += time;

				if !self.hide_answers {
					print_times(day, 1, &buffer, time);
				} else {
					print_times(day, 1, "", time);
				}
				buffer.clear();

				let time = solver.part_two(self.debug, &mut buffer);
				day_time += time;

				if !self.hide_answers {
					print_times(day, 2, &buffer, time);
				} else {
					print_times(day, 2, "", time);
				}
				buffer.clear();
			}

			for &part in parts {
				let time = match part {
					1 => solver.part_one(self.debug, &mut buffer),
					2 => solver.part_two(self.debug, &mut buffer),
					p => solver.run_any(p, self.debug, &mut buffer)?,
				};
				day_time += time;

				if !self.hide_answers {
					print_times(day, part, &buffer, time);
				} else {
					print_times(day, part, "", time);
				}
				buffer.clear();
			}

			println!("d{day:02} total: {day_time:?}\n");
			test_time += day_time;
		}
		println!("All: {:?}", test_time);
		Ok(test_time)
	}

	fn get_input(&mut self, day: u32) -> Res<Vec<u8>> {
		let input_main = input_main(day);
		if !input_main.exists() {
			let time_until_release = time_until_input_is_released(day);
			// If the puzzle is very far out
			if time_until_release > ChDuration::hours(1) {
				eprintln!(
					"Puzzle doesn't release for {:?}",
					time_until_release.to_std().unwrap()
				);
				return Err(AocError::HasNotReleasedYet {
					day,
					duration: time_until_release,
				});
			}

			// If the puzzle hasn't been out for at least 5 seconds
			if time_until_release > ChDuration::seconds(-5) {
				let delay = time_until_release + ChDuration::seconds(5);
				eprintln!(
					"Puzzle releases in {}, waiting {}",
					readable_time(time_until_release.to_std().unwrap_or_default(), 0),
					readable_time(delay.to_std().unwrap(), 0),
				);
				std::thread::sleep(delay.to_std().unwrap());
			}

			self.get_input_network(day)?;
		}

		let input = if self.test == 0 {
			std::fs::read(input_main)
		} else {
			std::fs::read(input_test(day, self.test))
		}?;

		Ok(input)
	}

	/// Get the input from the network and write it to the filesystem. Will overwrite any existing
	/// input files.
	fn get_input_network(&mut self, day: u32) -> Res<()> {
		let api_key = std::fs::read_to_string("./API_KEY")?;
		let api_key = api_key.trim();

		// Get main input
		let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");
		if self.runner_debug > 0 {
			eprintln!("Fetching {url}");
		}
		let client = self
			.client
			.get_or_insert_with(|| Client::builder().user_agent(USER_AGENT).build().unwrap());
		let req = client
			.get(url)
			.header(reqwest::header::COOKIE, format!("session={api_key}"))
			.send()?;
		if !req.status().is_success() {
			return Err(AocError::InputResponse {
				status: req.status(),
				response: req.text()?,
			});
		}
		let data = req.bytes()?.to_vec();

		let path = input_base_name(day);
		create_dir_all(path)?;
		let input_path = input_main(day);
		std::fs::write(input_path, data)?;

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
			return Err(AocError::PromptResponse {
				status: req.status(),
				response: req.text()?,
			});
		}
		let text = req.bytes()?;

		// Save prompt
		let prompt_path = prompt(day);
		File::create(prompt_path)?.write_all(&text)?;

		// Save each code block as a test case
		let regex = self
			.regex
			.get_or_insert_with(|| Regex::new(r"<pre>\s*<code>([^<]+)</code>\s*</pre>").unwrap());
		for (i, code) in regex.captures_iter(&text).enumerate() {
			let Ok(i) = (i + 1).try_into() else {
				eprintln!("{}, skipping the rest", AocError::TooManyTestCases);
				break;
			};
			if self.runner_debug > 0 {
				eprintln!("Got a code match, making a test {i}");
			}

			let code = &code[1];

			let test_path = input_test(day, i);
			let file = File::create(test_path)?;
			let mut file = BufWriter::new(file);

			html_escape::decode_html_entities_to_writer(
				std::str::from_utf8(code).map_err(|_| AocError::NonUtf8InPromptCodeBlock)?,
				&mut file,
			)?;
		}

		Ok(())
	}

	fn benchmark(&mut self, day_parts: &[(u32, Vec<u32>)]) -> Res<Duration> {
		let mut bench_times = Duration::ZERO;
		let mut total_time = Duration::ZERO;

		for &(day, _) in day_parts {
			let mut day_time = Duration::ZERO;

			if !(1..=25).contains(&day) {
				eprintln!("Day {day} not found, skipping");
				continue;
			}

			let file = self.get_input(day)?;
			let mut a1 = String::new();
			let mut a2 = String::new();

			let runs = if self.bench_count > 0 {
				for _ in 0..self.bench_count {
					let (time, p1, p2) = day_to_bench(day, file.clone(), self.debug)?;
					(a1, a2) = black_box((p1, p2));
					day_time += time;
				}
				self.bench_count
			} else {
				let start = Instant::now();
				let mut runs = 0;
				while start.elapsed() < Duration::from_millis(self.bench_time) {
					runs += 10;
					for _ in 0..10 {
						let (time, p1, p2) = day_to_bench(day, file.clone(), self.debug)?;
						(a1, a2) = black_box((p1, p2));
						day_time += time;
					}
				}
				runs
			};

			let avg_time = day_time / runs as _;

			print!(
				"d{day:02}: ran {runs:>7} times over {:>10} for avg of {:>10}",
				readable_time(day_time, 3),
				readable_time(avg_time, 3),
			);

			if !self.hide_answers {
				println!(" {:?}", [a1, a2],);
			} else {
				println!();
			}

			bench_times += avg_time;
			total_time += day_time;
		}

		println!("All: run avg of {:>10}", readable_time(bench_times, 3),);

		Ok(total_time)
	}
}

fn readable_time(duration: Duration, places: usize) -> String {
	match duration.as_millis() {
		0 => format!("{:.places$}μs", duration.as_nanos() as f32 / 1e3),
		1..=999 => format!("{:.places$}ms", duration.as_nanos() as f32 / 1e6),
		1_000..=119_999 => format!("{:.places$}s", duration.as_nanos() as f32 / 1e9),
		120_000.. => format!("{:.places$} minutes", duration.as_nanos() as f32 / 1e12),
	}
}

fn print_times<D: Display>(day: u32, part: u32, ans: D, time: Duration) {
	println!("d{day:02}p{part:02}: ({time:?}) {ans}");
}

fn prompt(day: u32) -> PathBuf {
	let mut name = input_base_name(day);
	name.push("prompt.html");
	name
}

fn input_test(day: u32, test: u8) -> PathBuf {
	let mut name = input_base_name(day);
	name.push(format!("input{test:02}.txt"));
	name
}

fn input_main(day: u32) -> PathBuf {
	let mut name = input_base_name(day);
	name.push("input.txt");
	name
}

fn input_base_name(day: u32) -> PathBuf {
	PathBuf::from(format!("./inputs/day{day:02}"))
}

fn parse_day(word: &str) -> Res<(u32, Vec<u32>)> {
	let mut nums = word.split('.');
	let day = if let Some(n) = nums.next() {
		if n.is_empty() {
			Err(AocError::NoDaySpecified {
				arg: word.to_string(),
			})
		} else {
			n.parse().map_err(|_| AocError::Parse {
				part: n.to_string(),
				arg: word.to_string(),
			})
		}
	} else {
		Err(AocError::EmptyArgument)
	}?;

	let rest = nums
		.map(|n| {
			if n.is_empty() {
				Err(AocError::EmptyPart {
					arg: word.to_string(),
				})
			} else {
				n.parse().map_err(|_| AocError::Parse {
					part: n.to_string(),
					arg: word.to_string(),
				})
			}
		})
		.collect::<Res<Vec<u32>>>()?;
	Ok((day, rest))
}

fn day_to_solver(day: u32, file: Vec<u8>, dbg: u8) -> Res<(Duration, Box<dyn SolverSafe>)> {
	use crate::days::*;
	#[allow(clippy::zero_prefixed_literal)]
	Ok(match day {
		01 => {
			let (time, solver) = time_fn(|| day01::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		02 => {
			let (time, solver) = time_fn(|| day02::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		03 => {
			let (time, solver) = time_fn(|| day03::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		04 => {
			let (time, solver) = time_fn(|| day04::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		05 => {
			let (time, solver) = time_fn(|| day05::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		06 => {
			let (time, solver) = time_fn(|| day06::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		07 => {
			let (time, solver) = time_fn(|| day07::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		08 => {
			let (time, solver) = time_fn(|| day08::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		09 => {
			let (time, solver) = time_fn(|| day09::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		10 => {
			let (time, solver) = time_fn(|| day10::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		11 => {
			let (time, solver) = time_fn(|| day11::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		12 => {
			let (time, solver) = time_fn(|| day12::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		13 => {
			let (time, solver) = time_fn(|| day13::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		14 => {
			let (time, solver) = time_fn(|| day14::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		15 => {
			let (time, solver) = time_fn(|| day15::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		16 => {
			let (time, solver) = time_fn(|| day16::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		17 => {
			let (time, solver) = time_fn(|| day17::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		18 => {
			let (time, solver) = time_fn(|| day18::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		19 => {
			let (time, solver) = time_fn(|| day19::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		20 => {
			let (time, solver) = time_fn(|| day20::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		21 => {
			let (time, solver) = time_fn(|| day21::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		22 => {
			let (time, solver) = time_fn(|| day22::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		23 => {
			let (time, solver) = time_fn(|| day23::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		24 => {
			let (time, solver) = time_fn(|| day24::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		25 => {
			let (time, solver) = time_fn(|| day25::Solution::initialize(file, dbg));
			(time, Box::new(solver))
		}
		d => return Err(AocError::DayNotFound(d)),
	})
}

#[allow(dead_code)]
fn day_to_bench(day: u32, file: Vec<u8>, dbg: u8) -> Res<(Duration, String, String)> {
	use crate::days::*;
	#[allow(clippy::zero_prefixed_literal)]
	let res = match day {
		01 => day01::Solution::run_both_string(file, dbg),
		02 => day02::Solution::run_both_string(file, dbg),
		03 => day03::Solution::run_both_string(file, dbg),
		04 => day04::Solution::run_both_string(file, dbg),
		05 => day05::Solution::run_both_string(file, dbg),
		06 => day06::Solution::run_both_string(file, dbg),
		07 => day07::Solution::run_both_string(file, dbg),
		08 => day08::Solution::run_both_string(file, dbg),
		09 => day09::Solution::run_both_string(file, dbg),
		10 => day10::Solution::run_both_string(file, dbg),
		11 => day11::Solution::run_both_string(file, dbg),
		12 => day12::Solution::run_both_string(file, dbg),
		13 => day13::Solution::run_both_string(file, dbg),
		14 => day14::Solution::run_both_string(file, dbg),
		15 => day15::Solution::run_both_string(file, dbg),
		16 => day16::Solution::run_both_string(file, dbg),
		17 => day17::Solution::run_both_string(file, dbg),
		18 => day18::Solution::run_both_string(file, dbg),
		19 => day19::Solution::run_both_string(file, dbg),
		20 => day20::Solution::run_both_string(file, dbg),
		21 => day21::Solution::run_both_string(file, dbg),
		22 => day22::Solution::run_both_string(file, dbg),
		23 => day23::Solution::run_both_string(file, dbg),
		24 => day24::Solution::run_both_string(file, dbg),
		25 => day25::Solution::run_both_string(file, dbg),
		d => return Err(AocError::DayNotFound(d)),
	};
	Ok(res)
}

/// Returns `None` if the input is released, otherwise returns the time until release. Returns
/// `None` if the time cannot be determined.
///
/// # Warning
///
/// This is likely to break (by not allowing downloading of the puzzle for an extra hour) if the
/// United States decides to remove time changes in favor of sticking to Daylight Saving Time,
/// and Eric Wastl continues to keep AoC on US-East time. In such an event, change
/// `ERIC_TIME_OFFSET` to `-4`.
// Note: chrono is actually way more confusing than I thought. Idk if this is the correct way to
// use it but it seems to work.
fn time_until_input_is_released(day: u32) -> ChDuration {
	const ERIC_TIME_OFFSET: i32 = -5;

	let t = Utc::now().naive_utc();

	let release = NaiveDate::from_ymd_opt(YEAR as _, 12, day)
		.unwrap()
		.and_hms_opt(0, 0, 0)
		.unwrap()
		.and_local_timezone(FixedOffset::east_opt(ERIC_TIME_OFFSET * 60 * 60).unwrap())
		.unwrap()
		.naive_utc();

	release - t
}

/// Time a single function.
pub fn time_fn<F: FnOnce() -> T, T>(f: F) -> (Duration, T) {
	let start = Instant::now();
	let t = f();
	let end = start.elapsed();
	(end, t)
}
