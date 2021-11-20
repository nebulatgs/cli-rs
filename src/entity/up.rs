use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpResponse {
	pub url: String,
	#[serde(rename = "logsUrl")]
	pub logs_url: String,
	#[serde(rename = "deploymentDomain")]
	pub deployment_domain: String,
}
