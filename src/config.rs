use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(#[from] env::VarError),
    #[error("Failed to load .env file")]
    DotEnvError,
    #[error("Invalid file path: {0}")]
    InvalidPath(String),
}

pub struct Config {
    pub spreadsheet_id: String,
    pub output_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().map_err(|_| ConfigError::DotEnvError)?;

        let spreadsheet_id = env::var("SPREADSHEET_ID")?;
        let output_dir = env::var("OUTPUT_DIR").unwrap_or_else(|_| ".".to_string());
        
        let output_path = Path::new(&output_dir);
        if !output_path.exists() {
            return Err(ConfigError::InvalidPath(format!(
                "Output directory does not exist: {}",
                output_dir
            )));
        }

        Ok(Config {
            spreadsheet_id,
            output_dir: output_path.to_path_buf(),
        })
    }
}
