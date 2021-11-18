use crate::{
	gql::queries::get_project,
	gql::queries::GetProject,
	util::{client::GQLClient, config::Configs},
};
use clap::Parser;
use colored::Colorize;
use graphql_client::reqwest::post_graphql;
#[derive(Parser)]
pub struct Args;

pub async fn command(_args: Args) -> super::CommandResult {
	let mut config = Configs::new().await?;
	let project_id = config.unlink_project()?.project;
	config.write().await?;
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<GetProject, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		get_project::Variables {
			project_id: project_id.into(),
		},
	)
	.await?;
	let body: get_project::ResponseData = res.data.ok_or("Failed to retrieve response body")?;
	let project_name = body.project_by_id.name;
	println!("🎉 Disconnected from {}", project_name.purple());
	Ok(())
}
