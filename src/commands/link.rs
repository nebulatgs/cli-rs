use clap::Parser;

use crate::util::{client::GQLClient, config::Configs, errors::RailwayError};

#[derive(Parser)]
pub struct Args;

pub async fn command(args: Args) -> super::CommandResult {
	let config = Configs::new().await?;
	let client = GQLClient::new_authorized(&config)?;
	Ok(())
}
