use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client,
};

use super::{config::Configs, consts, errors::RailwayError};

pub struct GQLClient;

impl GQLClient {
	pub fn new_authorized(configs: &Configs) -> super::UtilResult<Client> {
		let mut headers = HeaderMap::new();
		if let Some(token) = &Configs::get_railway_token() {
			headers.insert("project-access-token", HeaderValue::from_str(token)?);
		} else if let Some(token) = &configs.root_config.user.token {
			if token.is_empty() {
				return Err(RailwayError::Unauthorized);
			}
			headers.insert(
				"authorization",
				HeaderValue::from_str(&format!("Bearer {}", token))?,
			);
		} else {
			return Err(RailwayError::Unauthorized);
		}
		headers.insert("x-source", HeaderValue::from_static("cli-rs"));
		let client = Client::builder()
			.user_agent(consts::get_user_agent())
			.default_headers(headers)
			.build()?;
		Ok(client)
	}
	pub fn new_unauthorized() -> super::UtilResult<Client> {
		let mut headers = HeaderMap::new();
		headers.insert("x-source", HeaderValue::from_static("cli-rs"));
		let client = Client::builder()
			.user_agent(consts::get_user_agent())
			.default_headers(headers)
			.build()?;
		Ok(client)
	}
}
