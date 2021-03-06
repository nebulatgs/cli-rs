use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::{
	fs::{create_dir_all, File},
	io::{AsyncReadExt, AsyncWriteExt},
};

use super::errors::RailwayError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
	#[serde(rename = "projectPath")]
	pub project_path: String,
	pub project: String,
	pub environment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
	pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RailwayConfig {
	pub projects: HashMap<String, Project>,
	pub user: User,
}

#[derive(Debug)]
pub struct Configs {
	pub root_config: RailwayConfig,
	root_config_path: PathBuf,
}

impl Configs {
	pub async fn new() -> super::UtilResult<Self> {
		let mut root_config_partial_path = ".railway/config.json";
		if Self::is_dev_mode() {
			root_config_partial_path = ".railway/dev-config.json";
		}

		if Self::is_staging_mode() {
			root_config_partial_path = ".railway/staging-config.json";
		}

		let home_dir = dirs::home_dir().ok_or("Unable to get home directory")?;
		let root_config_path = std::path::Path::new(&home_dir).join(root_config_partial_path);

		if let Ok(mut file) = File::open(&root_config_path).await {
			let mut serialized_config = vec![];
			file.read_to_end(&mut serialized_config).await?;

			let root_config: RailwayConfig = serde_json::from_slice(&serialized_config)?;
			return Ok(Self {
				root_config,
				root_config_path,
			});
		}
		Ok(Self {
			root_config_path,
			root_config: RailwayConfig {
				projects: HashMap::new(),
				user: User { token: None },
			},
		})
	}
	pub fn is_staging_mode() -> bool {
		if let Ok(env) = std::env::var("RAILWAY_ENV") {
			return env.eq_ignore_ascii_case("staging");
		}
		false
	}
	pub fn is_dev_mode() -> bool {
		if let Ok(env) = std::env::var("RAILWAY_ENV") {
			return env.eq_ignore_ascii_case("develop");
		}
		false
	}
	pub fn get_railway_url() -> String {
		std::env::var("RAILWAY_URL").unwrap_or_else(|_| super::consts::RAILWAY_URL.to_string())
	}
	pub fn get_railway_token() -> Option<String> {
		std::env::var("RAILWAY_TOKEN").ok()
	}
	pub fn get_host() -> String {
		let mut base_url = "https://backboard.railway.app";
		if Self::is_dev_mode() {
			base_url = "https://backboard.railway-develop.app"
		}

		if Self::is_staging_mode() {
			base_url = "https://backboard.railway-staging.app"
		}
		base_url.to_string()
	}
	pub fn get_current_working_directory() -> super::UtilResult<String> {
		let current_dir = std::env::current_dir()?;
		let path = current_dir
			.to_str()
			.ok_or("Unable to get current working directory")?;
		Ok(path.into())
	}
	pub fn get_linked_project(&self) -> super::UtilResult<&Project> {
		let path = Self::get_current_working_directory()?;
		let project = self
			.root_config
			.projects
			.get(&path)
			.ok_or(RailwayError::NotLinked)?;
		Ok(project)
	}
	pub fn get_linked_project_mut(&mut self) -> super::UtilResult<&mut Project> {
		let path = Self::get_current_working_directory()?;
		let project = self
			.root_config
			.projects
			.get_mut(&path)
			.ok_or(RailwayError::NotLinked)?;
		Ok(project)
	}
	pub fn unlink_project(&mut self) -> super::UtilResult<Project> {
		let path = Self::get_current_working_directory()?;
		let project = self
			.root_config
			.projects
			.remove(&path)
			.ok_or(RailwayError::NotLinked)?;
		Ok(project)
	}
	pub async fn write(&self) -> super::UtilResult<()> {
		create_dir_all(self.root_config_path.parent().unwrap()).await?;
		let mut file = File::create(&self.root_config_path).await?;
		let serialized_config = serde_json::to_vec_pretty(&self.root_config)?;
		file.write_buf(&mut serialized_config.as_slice()).await?;
		file.sync_all().await?;
		Ok(())
	}
}
