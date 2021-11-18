use crate::{
	gql::queries::get_project,
	gql::queries::GetProject,
	util::{client::GQLClient, config::Configs, errors::RailwayError},
};
use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use graphql_client::reqwest::post_graphql;

#[derive(Parser)]
pub struct Args {
	#[clap(index = 1, about = "Environment to switch to")]
	environment: Option<String>,
}

pub async fn command(args: Args) -> super::CommandResult {
	let mut config = Configs::new().await?;
	let linked_project = config.get_linked_project()?;

	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<GetProject, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		get_project::Variables {
			project_id: linked_project.project.clone(),
		},
	)
	.await?;
	let body: get_project::ResponseData = res.data.ok_or("Failed to retrieve response body")?;

	let mut linked_project = config.get_linked_project_mut()?;

	let environments = body.project_by_id.environments;
	let environment = match args.environment {
		Some(arg_environment) => {
			let environment = environments
				.iter()
				.find(|x| x.id == arg_environment || x.name == arg_environment)
				.ok_or(RailwayError::EnvironmentNotFound)?;
			println!(
				"{} Select Environment · {}",
				"✔".green(),
				&environment.name.green()
			);
			environment
		}
		None => {
			let names = environments
				.iter()
				.map(|x| x.name.as_str())
				.collect::<Vec<_>>();
			let env_index = FuzzySelect::with_theme(&ColorfulTheme::default())
				.with_prompt("Select Environment")
				.default(0)
				.items(names.as_slice())
				.interact()?;
			&environments[env_index]
		}
	};
	linked_project.environment = environment.id.clone();
	config.write().await?;
	println!(
		"{} ProTip: You can view the active environment by running {}",
		"?".blue(),
		"railway status".blue()
	);
	Ok(())
}
