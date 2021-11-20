use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::{
	gql::{
		mutations::{create_plugin, CreatePlugin},
		queries::{available_plugins_for_project, AvailablePluginsForProject},
	},
	util::{
		client::{post_graphql, GQLClient},
		config::Configs,
		spinner::create_spinner,
	},
};

#[derive(Parser)]
pub struct Args;

pub async fn command(_args: Args) -> super::CommandResult {
	let config = Configs::new().await?;
	let linked_project = config.get_linked_project()?;
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<AvailablePluginsForProject, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		available_plugins_for_project::Variables {
			project_id: linked_project.project.clone(),
		},
	)
	.await?;
	let body: available_plugins_for_project::ResponseData =
		res.data.ok_or("Failed to retrieve response body")?;

	let plugins = body.available_plugins_for_project;

	let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
		.with_prompt("Select Plugin")
		.default(0)
		.items(plugins.as_slice())
		.interact()?;
	let plugin_name = plugins[selection].cyan().bold();

	let (tx, spinner_task) = create_spinner(format!("Adding {} plugin", plugin_name), false);

	let config = Configs::new().await?;
	let linked_project = config.get_linked_project()?;
	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<CreatePlugin, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		create_plugin::Variables {
			project_id: linked_project.project.clone(),
			name: plugins[selection].clone(),
		},
	)
	.await?;
	res.data.ok_or("Failed to retrieve response body")?;
	tx.send(true).ok().ok_or("Failed to shutdown spinner")?;
	spinner_task.await?;
	println!("🎉 Created plugin {}", plugin_name);
	Ok(())
}
