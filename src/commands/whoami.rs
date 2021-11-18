use clap::Parser;
use colored::Colorize;

use crate::{
	gql::queries::get_user,
	gql::queries::GetUser,
	util::{client::post_graphql, client::GQLClient, config::Configs},
};

#[derive(Parser)]
pub struct Args;

pub async fn command(_args: Args) -> super::CommandResult {
	let config = Configs::new().await?;
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<GetUser, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		get_user::Variables {},
	)
	.await?;
	let body: get_user::ResponseData = res.data.ok_or("Failed to retrieve response body")?;

	let name = body.me.name.ok_or("Failed to retrieve name")?;
	let email = body.me.email;
	println!("👋 Hey {} ({})", name, email.purple());
	Ok(())
}
