use clap::Parser;
use colored::Colorize;

use crate::util::consts;

#[derive(Parser)]
pub struct Args;

pub async fn command(_args: Args) -> super::CommandResult {
	println!("railway cli-rs version {}", consts::VERSION.purple());
	Ok(())
}
