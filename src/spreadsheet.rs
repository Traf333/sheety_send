use chrono::prelude::*;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpreadsheetError {
    #[error("Failed to fetch spreadsheet: {0}")]
    FetchError(#[from] reqwest::Error),
    #[error("Failed to create file: {0}")]
    FileCreationError(#[from] std::io::Error),
    #[error("Failed to convert path to string")]
    PathConversionError,
}

pub async fn fetch_and_save_sheet_data(config: &crate::config::Config) -> Result<String, SpreadsheetError> {
    // Construct the correct URL to export the sheet as xlsx
    let export_url = format!(
        "https://docs.google.com/spreadsheets/d/{}/export?format=xlsx",
        &config.spreadsheet_id
    );

    let response = Client::new()
        .get(&export_url)
        .send()
        .await?
        .bytes()
        .await?;

    let now = Local::now();
    let formatted_date = now.format("%d-%m-%Y").to_string();

    // Create the file name with the current date
    let file_name = format!("book-crossing-{}.xlsx", formatted_date);
    let file_path = Path::new(&file_name);
    let mut file = File::create(&file_path)?;
    file.write_all(&response)?;

    file_path
        .to_str()
        .ok_or(SpreadsheetError::PathConversionError)
        .map(|s| s.to_string())
}
