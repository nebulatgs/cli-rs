use crate::gql::mutations::{consume_login_session, create_login_session};
use crate::gql::mutations::{ConsumeLoginSession, CreateLoginSession};
use crate::gql::queries::get_user;
use crate::gql::queries::GetUser;
use crate::util::client::GQLClient;
use crate::util::config::Configs;
use clap::Parser;
use colored::Colorize;
use graphql_client::reqwest::post_graphql;
use std::io::stdin;
use std::time::Duration;

#[derive(Parser)]
pub struct Args {
	#[clap(long)]
	browserless: Option<bool>,
}

pub async fn command(args: Args) -> super::CommandResult {
	let mut config = Configs::new().await?;
	let url = Configs::get_railway_url();
	println!("Press Enter to open the browser (^C to quit)");
	stdin().read_line(&mut String::new())?;
	let hostname = hostname::get()?
		.to_str()
		.ok_or("Invalid hostname")?
		.to_string();
	match open::that(url.clone()) {
		// Continue browser-based login
		Ok(_) => {
			todo!("Browser-based login")
		}

		// Fallback to browserless login
		Err(_) => {
			eprintln!("Failed to open browser, attempting browserless login.");
			let client = GQLClient::new_unauthorized()?;
			let res = post_graphql::<CreateLoginSession, _>(
				&client,
				format!("{}/graphql", Configs::get_host()),
				create_login_session::Variables {},
			)
			.await?;
			let body: create_login_session::ResponseData =
				res.data.ok_or("Failed to retrieve response body")?;
			let code = body.create_login_session;
			let login_url = format!(
				"{}/cli-login?d={}",
				url,
				base64::encode(format!("wordCode={}&hostname={}", code, hostname))
			);
			println!("Your pairing code is: {}", code.purple());
			println!(
				"To authenticate with Railway, please go to\n    {}",
				login_url
			);
			loop {
				let res = post_graphql::<ConsumeLoginSession, _>(
					&client,
					format!("{}/graphql", Configs::get_host()),
					consume_login_session::Variables { code: code.clone() },
				)
				.await?;
				let body: consume_login_session::ResponseData =
					res.data.ok_or("Failed to retrieve response body")?;
				if body.consume_login_session == None {
					tokio::time::sleep(Duration::from_millis(100)).await;
					continue;
				}
				let token = body.consume_login_session.ok_or("Failed to get token")?;
				config.root_config.user.token = Some(token);
				config.write().await?;

				let client = GQLClient::new_authorized(&config)?;
				let res = post_graphql::<GetUser, _>(
					&client,
					format!("{}/graphql", Configs::get_host()),
					get_user::Variables {},
				)
				.await?;
				let body: get_user::ResponseData =
					res.data.ok_or("Failed to retrieve response body")?;

				let name = body.me.name.ok_or("Failed to retrieve name")?;
				let email = body.me.email;
				println!("\n🎉 Logged in as {} ({})", name.bold(), email);
				break;
			}
		}
	};
	Ok(())
}
