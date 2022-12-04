use chrono::{Duration as ChDuration, FixedOffset, NaiveDate, Utc};
use clap::{ArgAction, Parser};
use regex::bytes::Regex;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{self, stdout, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use crate::solution::Solver;
use crate::{AocError, Res, YEAR};

/// User agent (see [Eric's post on the
/// subreddit](https://www.reddit.com/r/adventofcode/comments/z9dhtd))
const USER_AGENT: &str = "\
	drewtato-aoc-runner-2022 \
	at github.com/drewtato/aoc2022 \
	by 15526875+drewtato@users.noreply.github.com\
";

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

		if days.is_empty() {
			eprintln!("No days specified. Use `--help` for more.");
			return Ok(());
		}

		if runner_debug > 0 {
			eprintln!("Parsing day arguments");
		}
		let days = days
			.iter()
			.map(Self::parse_day_arg)
			.collect::<Result<Vec<_>, _>>()?;

		if runner_debug > 0 {
			eprintln!("Running days");
		}

		let times = if bench > 0 {
			if runner_debug > 0 {
				eprintln!("Running benchmark with {bench} cycles");
			}
			let times = self.benchmark(days)?;

			if runner_debug > 0 {
				eprintln!("Finished benchmark");
			}

			times
		} else {
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
			times
		};

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

		let input = self.get_input(day)?;

		print!("Initializing day {day}... ");
		stdout().flush()?;

		let times = solver(self, input, day, parts)?;

		println!("Time for day {day}: {times:?}\n");

		Ok(times)
	}

	/// Get an input from the filesystem, or if it's not there, from the network.
	pub fn get_input(&mut self, day: u32) -> Res<Vec<u8>> {
		let input_name = if self.test == 0 {
			format!("./inputs/day{day:02}/input.txt")
		} else {
			format!("./inputs/day{day:02}/input{:02}.txt", self.test)
		};
		if self.runner_debug > 0 {
			eprintln!("Reading input from {input_name}");
		}

		let input_path = PathBuf::from(input_name.clone());
		let buf = if !input_path.exists() {
			if self.test == 0 {
				if self.runner_debug > 0 {
					eprintln!("The input file does not exist, fetching input from network");
				}
				let until = time_until_input_is_released(day);
				if until > ChDuration::zero() {
					if until < ChDuration::seconds(60) {
						let wait = until.num_seconds() + 5;
						eprintln!(
							"Day {day} releases in {} seconds, waiting {} seconds.",
							until.num_seconds(),
							wait
						);
						thread::sleep(Duration::from_secs(wait as u64));
					} else {
						return Err(crate::AocError::HasNotReleasedYet {
							day,
							duration: until,
						});
					}
				}
				self.get_input_network(day, &input_path)?
			} else {
				return Err(crate::AocError::NoTestInputFound { path: input_name });
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
		let client = self
			.client
			.get_or_insert_with(|| Client::builder().user_agent(USER_AGENT).build().unwrap());
		let req = client
			.get(url)
			.header(COOKIE, format!("session={api_key}"))
			.send()?;
		if !req.status().is_success() {
			return Err(AocError::InputResponse {
				status: req.status(),
				response: req.text()?,
			});
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
			return Err(AocError::PromptResponse {
				status: req.status(),
				response: req.text()?,
			});
		}
		let text = req.bytes()?;

		// Save prompt
		let prompt_path = path.parent().unwrap().join("prompt.html");
		File::create(prompt_path)?.write_all(&text)?;

		// Save each code block as a test case
		let regex = self
			.regex
			.get_or_insert_with(|| Regex::new(r"<pre>\s*<code>([^<]+)</code>\s*</pre>").unwrap());
		for (i, code) in regex.captures_iter(&text).enumerate() {
			let i = i + 1;
			if self.runner_debug > 0 {
				eprintln!("Got a code match, making a test {i}");
			}

			let code = &code[1];

			let test_path = path.parent().unwrap().join(format!("input{i:02}.txt"));
			let file = File::create(test_path)?;
			let mut file = BufWriter::new(file);

			html_escape::decode_html_entities_to_writer(
				std::str::from_utf8(code).map_err(|_| AocError::NonUtf8InPromptCodeBlock)?,
				&mut file,
			)?;
		}

		Ok(data)
	}

	/// Run the solver for a day. Returns the time taken to solve and the solutions.
	pub fn solver_quiet<S: Solver>(
		&self,
		file: Vec<u8>,
		day: u32,
		parts: &[u32],
	) -> Result<(Duration, Vec<String>), io::Error> {
		let mut solutions = Vec::new();

		let (init_time, mut sol) = time_fn(|| S::initialize_dbg(file, self.debug));

		Ok(if parts.is_empty() {
			let (p1_time, p1) = time_fn(|| sol.part_one_dbg(self.debug));

			let (p2_time, p2) = time_fn(|| sol.part_two_dbg(self.debug));

			let times = init_time + p1_time + p2_time;
			solutions.push(p1.to_string());
			solutions.push(p2.to_string());

			(times, solutions)
		} else {
			let mut times = Duration::ZERO;
			for &part in parts {
				let (time, ans) = match part {
					1 => {
						let (time, ans) = time_fn(|| sol.part_one_dbg(self.debug));
						(time, ans.to_string())
					}
					2 => {
						let (time, ans) = time_fn(|| sol.part_two_dbg(self.debug));
						(time, ans.to_string())
					}
					p => {
						if let (time, Ok(s)) = time_fn(|| sol.run_any_dbg(p, self.debug)) {
							(time, s)
						} else {
							println!("Day {day} did not include a part {p}, skipping.");
							continue;
						}
					}
				};

				times += time;
				solutions.push(ans);
			}

			(times, solutions)
		})
	}

	/// Run the solver for a day and print info. Returns the time taken to solve.
	pub fn solver<S: Solver>(
		&self,
		file: Vec<u8>,
		day: u32,
		parts: &[u32],
	) -> Result<Duration, io::Error> {
		let (init_time, mut sol) = time_fn(|| S::initialize_dbg(file, self.debug));
		println!("took {:?}", init_time);

		Ok(if parts.is_empty() {
			print!("Running day {day} part 1... ");
			stdout().flush()?;

			let (p1_time, p1) = time_fn(|| sol.part_one_dbg(self.debug));

			println!("took {p1_time:?}",);
			println!("d{day:02}p1: {p1}",);

			print!("Running day {day} part 2... ");
			stdout().flush()?;

			let (p2_time, p2) = time_fn(|| sol.part_two_dbg(self.debug));

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
						let (time, ans) = time_fn(|| sol.part_one_dbg(self.debug));
						(time, ans.to_string())
					}
					2 => {
						let (time, ans) = time_fn(|| sol.part_two_dbg(self.debug));
						(time, ans.to_string())
					}
					p => {
						if let (time, Ok(s)) = time_fn(|| sol.run_any_dbg(p, self.debug)) {
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
	pub fn parse_day_arg<S: AsRef<str>>(s: S) -> Res<(u32, Vec<u32>)> {
		let s = s.as_ref();
		let mut parts = s.split('.').map(|part| {
			part.parse().map_err(|_| AocError::Parse {
				part: part.to_string(),
				arg: s.to_string(),
			})
		});

		let day = parts
			.next()
			.ok_or(AocError::NoDaySpecified { arg: s.to_string() })??;
		let parts = parts.collect::<Result<_, _>>()?;
		Ok((day, parts))
	}

	/// Benchmark a day, or all if the day is 0. Will load all inputs from disk first, then runs
	/// all necessary parts in sequence and time it as a whole.
	pub fn benchmark(&mut self, days: Vec<(u32, Vec<u32>)>) -> Res<Duration> {
		let mut total_times = Duration::ZERO;
		let mut answers = Vec::with_capacity(1000);

		for (day, parts) in days {
			let solver = {
				use crate::days::*;
				match day {
					1 => Self::solver_quiet::<day01::Solution>,
					2 => Self::solver_quiet::<day02::Solution>,
					3 => Self::solver_quiet::<day03::Solution>,
					4 => Self::solver_quiet::<day04::Solution>,
					5 => Self::solver_quiet::<day05::Solution>,
					6 => Self::solver_quiet::<day06::Solution>,
					7 => Self::solver_quiet::<day07::Solution>,
					8 => Self::solver_quiet::<day08::Solution>,
					9 => Self::solver_quiet::<day09::Solution>,
					10 => Self::solver_quiet::<day10::Solution>,
					11 => Self::solver_quiet::<day11::Solution>,
					12 => Self::solver_quiet::<day12::Solution>,
					13 => Self::solver_quiet::<day13::Solution>,
					14 => Self::solver_quiet::<day14::Solution>,
					15 => Self::solver_quiet::<day15::Solution>,
					16 => Self::solver_quiet::<day16::Solution>,
					17 => Self::solver_quiet::<day17::Solution>,
					18 => Self::solver_quiet::<day18::Solution>,
					19 => Self::solver_quiet::<day19::Solution>,
					20 => Self::solver_quiet::<day20::Solution>,
					21 => Self::solver_quiet::<day21::Solution>,
					22 => Self::solver_quiet::<day22::Solution>,
					23 => Self::solver_quiet::<day23::Solution>,
					24 => Self::solver_quiet::<day24::Solution>,
					25 => Self::solver_quiet::<day25::Solution>,
					d => {
						println!("Invalid day {d}, skipping.");
						continue;
					}
				}
			};

			let file = self.get_input(day)?;
			let mut times = Duration::ZERO;
			for _ in 0..self.bench {
				let file = file.clone();
				let (time, ans) = solver(self, file, day, &parts)?;
				times += time;
				answers.push(ans);
			}
			println!(
				"d{day:02}: {times:?} total, {:.6}ms per run",
				(times / self.bench).as_secs_f64() * 1000.0
			);
			total_times += times;
			if answers.len() > 990 {
				answers.truncate(100);
			}
		}

		println!(
			"All: {total_times:?} total, {:.6}ms per run",
			(total_times / self.bench).as_secs_f64() * 1000.0
		);

		println!("Some answers: {:?}", &answers[0..3]);

		Ok(total_times)
	}
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
pub fn time_until_input_is_released(day: u32) -> ChDuration {
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
