use clap::Parser;
use colored::Colorize;

use crate::{
	gql::mutations::delete_project,
	gql::mutations::DeleteProject,
	gql::queries::get_projects,
	gql::queries::GetProjects,
	util::{client::post_graphql, client::GQLClient, config::Configs, errors::RailwayError},
};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

#[derive(Parser)]
pub struct Args {
	#[clap(index = 1, about = "Name or ID of project to delete")]
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
				.with_prompt("Select Project to delete")
				.default(0)
				.items(names.as_slice())
				.interact()?;
			&projects[selection]
		}
	};
	let should_delete = Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt(format!(
			"Are you sure you want to delete project {}",
			project.name.purple()
		))
		.default(false)
		.wait_for_newline(true)
		.interact()?;
	if should_delete {
		if let Some(path) = config.root_config.projects.iter().find_map(|(k, v)| {
			if v.project == project.id {
				Some(k)
			} else {
				None
			}
		}) {
			let key = path.as_str().to_owned();
			config.root_config.projects.remove(&key);
			config.write().await?;
		}

		let client = GQLClient::new_authorized(&config)?;
		let res = post_graphql::<DeleteProject, _>(
			&client,
			format!("{}/graphql", Configs::get_host()),
			delete_project::Variables {
				project_id: project.id.clone(),
			},
		)
		.await?;
		res.data.ok_or("Failed to retrieve response body")?;

		println!("🗑️  Deleted project {}", project.name.purple());
	} else {
		println!("Not deleting project {}", project.name.purple());
	}
	// let path = std::env::current_dir()?.to_string_lossy().into_owned();
	// let project_id = project.id.clone();
	config.write().await?;
	Ok(())
}
