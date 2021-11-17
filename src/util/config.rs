use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
	pub projectPath: String,
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
	pub railway_token: Option<String>,
}

#[derive(Debug)]
pub struct Configs {
	pub root_config: RailwayConfig,
	root_config_path: PathBuf,
}

impl Configs {
	pub async fn new() -> super::UtilResult<Self> {
		let mut root_config_partial_path = ".railway/config.json";
		if Self::is_dev_mode().is_ok() {
			root_config_partial_path = ".railway/dev-config.json";
		}

		if Self::is_staging_mode().is_ok() {
			root_config_partial_path = ".railway/staging-config.json";
		}

		let home_dir = dirs::home_dir().ok_or("Unable to get home directory")?;
		let root_config_path = std::path::Path::new(&home_dir).join(root_config_partial_path);

		let mut file = File::open(&root_config_path).await?;
		let mut serialized_config = vec![];
		file.read_to_end(&mut serialized_config).await?;

		let root_config: RailwayConfig = serde_json::from_slice(&serialized_config)?;
		Ok(Self {
			root_config,
			root_config_path,
		})
	}
	pub fn is_staging_mode() -> super::UtilResult<bool> {
		let env = std::env::var("RAILWAY_ENV")?;
		Ok(env == "staging")
	}
	pub fn is_dev_mode() -> super::UtilResult<bool> {
		let env = std::env::var("RAILWAY_ENV")?;
		Ok(env == "develop")
	}
	pub fn get_railway_url() -> super::UtilResult<String> {
		let env = std::env::var("RAILWAY_URL").unwrap_or(super::consts::RAILWAY_URL.to_string());
		Ok(env)
	}
	pub fn get_host() -> String {
		let mut base_url = "https://backboard.railway.app";
		if Self::is_dev_mode().is_ok() {
			base_url = "https://backboard.railway-develop.app"
		}

		if Self::is_staging_mode().is_ok() {
			base_url = "https://backboard.railway-staging.app"
		}
		base_url.to_string()
	}
	pub async fn write(&self) -> super::UtilResult<()> {
		let mut file = File::open(&self.root_config_path).await?;
		let serialized_config = serde_json::to_vec_pretty(&self.root_config)?;
		file.write_buf(&mut serialized_config.as_slice()).await?;
		file.sync_all().await?;
		Ok(())
	}
}
