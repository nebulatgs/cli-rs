use clap::Parser;
use colored::Colorize;
use ignore::WalkBuilder;
use std::sync::{Arc, Mutex};
use synchronized_writer::SynchronizedWriter;

use gzp::{deflate::Gzip, ZBuilder};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use tar::Builder;

use crate::{
	entity::up::UpResponse,
	gql::queries::get_project,
	gql::queries::GetProject,
	util::{
		client::post_graphql,
		client::GQLClient,
		config::Configs,
		errors::RailwayError,
		spinner::{create_spinner, create_spinner_with_chars},
	},
};

#[derive(Parser)]
pub struct Args {
	#[clap(index = 1)]
	path: Option<String>,
	#[clap(
		short,
		long,
		takes_value = false,
		about = "Detach from cloud build/deploy logs"
	)]
	detach: Option<Option<bool>>,
	#[clap(short, long, about = "Specify an environment to up onto")]
	#[allow(clippy::option_option)]
	environment: Option<String>,
}

pub async fn command(args: Args) -> super::CommandResult {
	let config = Configs::new().await?;
	let project = config.get_linked_project()?;

	let client = GQLClient::new_authorized(&config)?;
	let res = post_graphql::<GetProject, _>(
		&client,
		format!("{}/graphql", Configs::get_host()),
		get_project::Variables {
			project_id: project.project.clone(),
		},
	)
	.await?;
	let body: get_project::ResponseData = res.data.ok_or("Failed to retrieve response body")?;

	let mut env_id = &project.environment;

	if let Some(arg_environment) = &args.environment {
		env_id = &body
			.project_by_id
			.environments
			.iter()
			.find(|x| x.id == *arg_environment || x.name == *arg_environment)
			.ok_or(RailwayError::EnvironmentNotFound)?
			.id;
	}

	let (tx, spinner_task) =
		create_spinner_with_chars("Indexing".cyan().bold().to_string(), true, "/-\\|");

	let bytes = Vec::<u8>::new();
	let arc = Arc::new(Mutex::new(bytes));
	let mut parz = ZBuilder::<Gzip, _>::new()
		.num_threads(64)
		.from_writer(SynchronizedWriter::new(arc.clone()));
	{
		let mut archive = Builder::new(&mut parz);
		let mut builder = WalkBuilder::new(args.path.unwrap_or_else(|| ".".to_string()));
		let walker = builder.follow_links(true).hidden(false);
		let walked = walker.build().collect::<Vec<_>>();
		tx.send(true).ok().ok_or("Failed to shutdown spinner")?;
		spinner_task.await?;
		let pg = ProgressBar::new(walked.len() as u64).with_message("Compressing");
		pg.enable_steady_tick(100);
		pg.set_style(
			ProgressStyle::default_bar()
				.template("  {msg:.cyan.bold} [{bar:20}] {percent}% {spinner}")
				.progress_chars("=> ")
				.tick_chars("/-\\|"),
		);
		for entry in walked.into_iter().progress_with(pg) {
			archive.append_path(entry?.path())?;
		}
	}
	parz.finish()?;
	let (tx, spinner_task) = create_spinner("Laying tracks in the clouds...".to_string(), false);
	let client = GQLClient::new_authorized(&config)?;
	let builder = client.post(format!(
		"https://backboard.railway.app/project/{}/environment/{}/up",
		project.project, env_id
	));

	let res = builder
		.header("Content-Type", "multipart/form-data")
		.body(arc.lock().unwrap().clone())
		.send()
		.await?
		.error_for_status()?;
	let body = res.json::<UpResponse>().await?;
	tx.send(true).ok().ok_or("Failed to shutdown spinner")?;
	spinner_task.await?;
	println!("☁️  Build logs available at {}", body.logs_url.dimmed());
	if args.detach.is_some() {
		return Ok(());
	}
	Ok(())
}
