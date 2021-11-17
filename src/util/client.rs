use reqwest::{header::HeaderMap, Client};

use super::config::Configs;

pub struct GQLClient;

impl GQLClient {
	pub fn new_authorized(configs: &Configs) -> super::UtilResult<Client> {
		let mut headers = HeaderMap::new();
		if let Some(token) = &Configs::get_railway_token() {
			headers.insert("project-access-token", token.parse()?);
		} else if let Some(token) = &configs.root_config.user.token {
			headers.insert("authorization", format!("Bearer {}", token).parse()?);
		} else {
			Err("Failed to authorize request")?;
		}
		headers.insert("x-source", "cli-rs".parse()?);
		let client = Client::builder()
			.user_agent("cli-rs/0.0.0")
			.default_headers(headers)
			.build()?;
		Ok(client)
	}
	pub fn new_unauthorized() -> super::UtilResult<Client> {
		let mut headers = HeaderMap::new();
		headers.insert("x-source", "cli-rs".parse()?);
		let client = Client::builder()
			.user_agent("cli-rs/0.0.0")
			.default_headers(headers)
			.build()?;
		Ok(client)
	}
}
