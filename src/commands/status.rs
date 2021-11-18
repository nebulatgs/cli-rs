use crate::{
	gql::queries::get_project,
	gql::queries::GetProject,
	util::{client::post_graphql, client::GQLClient, config::Configs, errors::RailwayError},
};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
pub struct Args;

pub async fn command(_args: Args) -> super::CommandResult {
	let config = Configs::new().await?;
	let project = &config.get_linked_project()?;
	config.write().await?;
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<GetProject, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		get_project::Variables {
			project_id: project.project.to_owned(),
		},
	)
	.await?;
	let body: get_project::ResponseData = res.data.ok_or("Failed to retrieve response body")?;

	println!("Project: {}", body.project_by_id.name.purple().bold());
	println!(
		"Environment: {}",
		body.project_by_id
			.environments
			.iter()
			.find(|env| env.id == project.environment)
			.ok_or(RailwayError::EnvironmentNotFound)?
			.name
			.blue()
			.bold()
	);
	println!("Plugins:");
	for plugin in body.project_by_id.plugins.into_iter() {
		println!("{}", format!("{:?}", plugin.name).dimmed().bold());
	}
	Ok(())
}
