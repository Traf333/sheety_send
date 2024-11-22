use dotenv::dotenv;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(#[from] env::VarError),
    #[error("Failed to load .env file")]
    DotEnvError,
}

pub struct Config {
    pub spreadsheet_id: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().map_err(|_| ConfigError::DotEnvError)?;

        Ok(Config {
            spreadsheet_id: env::var("SPREADSHEET_ID")?,
        })
    }
}
