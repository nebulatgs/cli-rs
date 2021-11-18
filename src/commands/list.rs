use clap::Parser;
use graphql_client::reqwest::post_graphql;

use crate::{
	gql::queries::get_projects,
	gql::queries::GetProjects,
	util::{client::GQLClient, config::Configs, errors::RailwayError},
};
use colored::Colorize;

#[derive(Parser)]
pub struct Args;

pub async fn command(args: Args) -> super::CommandResult {
	let config = Configs::new().await?;
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<GetProjects, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		get_projects::Variables {},
	)
	.await?;
	let body: get_projects::ResponseData = res.data.ok_or("Failed to retrieve response body")?;
	let projects = body.me.projects;
	if let Ok(linked_project) = config.get_linked_project() {
		for project in &projects {
			if project.id == linked_project.project {
				println!("{}", project.name.purple());
				continue;
			}
			println!("{}", project.name.dimmed())
		}
	} else {
		for project in &projects {
			println!("{}", project.name.dimmed())
		}
	}

	Ok(())
}
