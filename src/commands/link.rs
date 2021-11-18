use clap::Parser;
use graphql_client::reqwest::post_graphql;

use crate::{
	gql::queries::get_projects,
	gql::queries::GetProjects,
	util::{
		client::GQLClient,
		config::{Configs, Project},
		errors::RailwayError,
	},
};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

#[derive(Parser)]
pub struct Args {
	#[clap(index = 1, about = "Name or ID of project to link")]
	project: Option<String>,
}

pub async fn command(args: Args) -> super::CommandResult {
	let mut config = Configs::new().await?;
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<GetProjects, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		get_projects::Variables {},
	)
	.await?;
	let body: get_projects::ResponseData = res.data.ok_or("Failed to retrieve response body")?;

	let mut projects = body.me.projects;
	projects.sort_by(|l, r| r.updated_at.cmp(&l.updated_at));

	let project = match args.project {
		Some(project) => {
			match projects
				.iter()
				.find(|item| item.name == project || item.id == project)
			{
				Some(found) => found,
				None => return Err(RailwayError::ProjectNotFound),
			}
		}
		None => {
			let names = projects
				.iter()
				.map(|project| project.name.as_str())
				.collect::<Vec<_>>();

			let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
				.with_prompt("Select Project")
				.default(0)
				.items(names.as_slice())
				.interact()?;
			&projects[selection]
		}
	};
	let mut env_index: usize = 0;
	if project.environments.len() > 1 {
		let names = project
			.environments
			.iter()
			.map(|env| &env.name)
			.collect::<Vec<_>>();
		env_index = FuzzySelect::with_theme(&ColorfulTheme::default())
			.with_prompt("Select Environment")
			.default(0)
			.items(names.as_slice())
			.interact()?;
	}
	let path = std::env::current_dir()?.to_string_lossy().into_owned();
	let project_id = project.id.clone();
	let env_id = project.environments[env_index].id.clone();
	let value = Project {
		environment: env_id,
		project: project_id,
		project_path: path.clone(),
	};
	config.root_config.projects.insert(path, value);
	config.write().await?;
	Ok(())
}
