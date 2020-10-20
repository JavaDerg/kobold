use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
	pub pg: DatabaseConfig,
	pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
	pub user: String,
	pub password: String,
	pub dbname: String,
	pub host: String,
	pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
	pub listeners: Vec<String>,
	pub redis: String,
}

pub fn load() -> Result<AppConfig, config::ConfigError> {
	let mut s = config::Config::new();

	s.merge(config::File::with_name("app_config").required(false))?;

	s.merge(config::Environment::with_prefix("app"))?;

	s.try_into()
}
