use bytes::Bytes;
use chrono::prelude::*;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
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

async fn fetch_sheet(spreadsheet_id: &str) -> Result<Bytes, SpreadsheetError> {
    // Construct the correct URL to export the sheet as xlsx
    let export_url = format!(
        "https://docs.google.com/spreadsheets/d/{}/export?format=xlsx",
        spreadsheet_id
    );

    let response = Client::new().get(&export_url).send().await?.bytes().await?;

    Ok(response)
}

async fn save_file(data: Bytes, output_dir: PathBuf) -> Result<String, SpreadsheetError> {
    let now = Local::now();
    let formatted_date = now.format("%d-%m-%Y").to_string();

    // Create the file name with the current date
    let file_name = format!("book-crossing-{}.xlsx", formatted_date);
    let file_path = output_dir.join(file_name);
    
    let mut file = File::create(&file_path)?;
    file.write_all(&data)?;

    file_path
        .to_str()
        .ok_or(SpreadsheetError::PathConversionError)
        .map(|s| s.to_string())
}

pub async fn fetch_and_save_sheet_data(
    config: &crate::config::Config,
) -> Result<String, SpreadsheetError> {
    let sheet_data = fetch_sheet(&config.spreadsheet_id).await?;
    save_file(sheet_data, config.output_dir.clone()).await
}
