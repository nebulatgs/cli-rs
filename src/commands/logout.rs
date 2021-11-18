use clap::Parser;
use colored::Colorize;
use graphql_client::reqwest::post_graphql;

use crate::{
	gql::mutations::logout,
	gql::mutations::Logout,
	util::{client::GQLClient, config::Configs},
};

#[derive(Parser)]
pub struct Args;

pub async fn command(args: Args) -> super::CommandResult {
	let config = Configs::new().await?;
	if config.root_config.user.token.is_none() {
		eprintln!("{}", "🚪  Already logged out".yellow());
		return Ok(());
	}
	if let Some(token) = &config.root_config.user.token {
		if token.is_empty() {
			eprintln!("{}", "🚪  Already logged out".yellow());
			return Ok(());
		}
	}
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<Logout, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		logout::Variables {},
	)
	.await?;
	res.data.ok_or("Failed to retrieve response body")?;
	println!("{}", "👋 Logged out".yellow());
	Ok(())
}
