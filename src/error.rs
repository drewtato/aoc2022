use chrono::Duration;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocError {
	#[error("Part not found")]
	PartNotFound,
	#[error("Day {day} hasn't released yet. It releases {}:{:02}:{:02}:{:02} from now.",
	.duration.num_days(),
	.duration.num_hours() - .duration.num_days() * 24,
	.duration.num_minutes() - .duration.num_hours() * 60,
	.duration.num_seconds() - .duration.num_minutes() * 60)]
	HasNotReleasedYet { day: u32, duration: Duration },
	#[error("No test input found with the name {path}")]
	NoTestInputFound { path: String },
	#[error(transparent)]
	File {
		#[from]
		source: std::io::Error,
	},
	#[error(transparent)]
	Request {
		#[from]
		source: reqwest::Error,
	},
	#[error("Couldn't fetch prompt from network. Status {status}, content:\n{response}")]
	PromptResponse {
		status: StatusCode,
		response: String,
	},
	#[error(transparent)]
	OtherError {
		#[from]
		source: Box<dyn std::error::Error>,
	},
	#[error("Couldn't fetch input from network. Status: {status}\nContent:\n{response}")]
	InputResponse {
		status: reqwest::StatusCode,
		response: String,
	},
	#[error("No day specified in argument `{arg}`")]
	NoDaySpecified { arg: String },
	#[error("Could not parse `{part}` as integer in argument `{arg}`")]
	Parse { part: String, arg: String },
}
