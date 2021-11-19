use std::{env::VarError, error::Error, fmt::Display};

use backtrace::Backtrace;
use colored::Colorize;
use gzp::GzpError;
use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum RailwayError {
	Unauthorized,
	IOError(std::io::Error),
	ReqwestError(reqwest::Error),
	SerdeJsonError(serde_json::Error),
	IgnoreError(ignore::Error),
	GzpError(GzpError),
	InvalidHeaderValue(InvalidHeaderValue),
	VarError(VarError),
	ProjectNotFound,
	EnvironmentNotFound,
	NotLinked,
	String(&'static str),
	Panic(Box<dyn Error>, String),
}

impl Display for RailwayError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Unauthorized => {
				write!(
					f,
					"{}\nRun {}",
					"Not logged in.".red(),
					"railway login".bold()
				)
			}
			Self::IOError(io_err) => io_err.fmt(f),
			Self::ReqwestError(reqwest_error) => reqwest_error.fmt(f),
			Self::SerdeJsonError(serde_json_error) => serde_json_error.fmt(f),
			Self::IgnoreError(ignore_error) => ignore_error.fmt(f),
			Self::GzpError(gzp_error) => gzp_error.fmt(f),
			Self::InvalidHeaderValue(invalid_header_value_error) => {
				invalid_header_value_error.fmt(f)
			}
			Self::VarError(var_error) => var_error.fmt(f),
			Self::ProjectNotFound => {
				write!(f, "{} Tip If you haven't, do railway login\nOtherwise, run {}  to get plugged into a new project, or {} to get plugged into an existing project.", "Project not found.".red(), "railway init".bold(), "railway link".bold())
			}
			Self::EnvironmentNotFound => {
				write!(
					f,
					"{}",
					"Environment does not exist on project. Specify an existing environment".red()
				)
			}
			Self::NotLinked => {
				write!(f, "{}", "Not linked to a project!".red())
			}
			Self::String(string_error) => string_error.fmt(f),
			Self::Panic(err, _) => err.fmt(f),
		}
	}
}
impl Error for RailwayError {}

impl From<std::io::Error> for RailwayError {
	fn from(err: std::io::Error) -> Self {
		RailwayError::IOError(err)
	}
}

impl From<VarError> for RailwayError {
	fn from(err: VarError) -> Self {
		RailwayError::VarError(err)
	}
}

impl From<reqwest::Error> for RailwayError {
	fn from(err: reqwest::Error) -> Self {
		RailwayError::ReqwestError(err)
	}
}

impl From<serde_json::Error> for RailwayError {
	fn from(err: serde_json::Error) -> Self {
		RailwayError::SerdeJsonError(err)
	}
}

impl From<ignore::Error> for RailwayError {
	fn from(err: ignore::Error) -> Self {
		RailwayError::IgnoreError(err)
	}
}

impl From<GzpError> for RailwayError {
	fn from(err: GzpError) -> Self {
		RailwayError::GzpError(err)
	}
}

impl From<InvalidHeaderValue> for RailwayError {
	fn from(err: InvalidHeaderValue) -> Self {
		RailwayError::InvalidHeaderValue(err)
	}
}

impl From<Box<dyn Error>> for RailwayError {
	fn from(err: Box<dyn Error>) -> Self {
		let backtrace = format!("{:?}", Backtrace::new());
		RailwayError::Panic(err, backtrace)
	}
}

impl From<&'static str> for RailwayError {
	fn from(err: &'static str) -> Self {
		RailwayError::String(err)
	}
}
