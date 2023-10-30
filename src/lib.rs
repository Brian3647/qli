use anyhow::Result;
use reqwest::blocking::{Client, Response};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestConfig {
	pub url: String,
	pub method: String,
	pub headers: Option<HashMap<String, String>>,
	pub body: Option<String>,
}

pub fn from_yaml(yaml_content: &str) -> Result<Response> {
	let config: RequestConfig = serde_yaml::from_str(yaml_content)?;
	make_request(&config)
}

pub fn from_json(json_content: &str) -> Result<Response> {
	let config: RequestConfig = serde_json::from_str(json_content)?;
	make_request(&config)
}

fn make_request(config: &RequestConfig) -> Result<Response> {
	let client = Client::new();
	let mut request = client.request(
		Method::from_str(&config.method.to_uppercase())?,
		&config.url,
	);

	if let Some(headers) = &config.headers {
		for (name, value) in headers.iter() {
			request = request.header(name, value);
		}
	}

	if let Some(body) = &config.body {
		request = request.body(body.to_string());
	}

	Ok(request.send()?)
}
