// src/spreadsheet.rs

use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub async fn fetch_and_save_sheet_data(config: &crate::config::Config) -> String {
    // Construct the correct URL to export the sheet as xlsx
    let export_url = format!(
        "https://docs.google.com/spreadsheets/d/{}/export?format=xlsx",
        &config.spreadsheet_id
    );

    let response = Client::new()
        .get(&export_url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let file_path = Path::new("sheet_data.xlsx");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(&response).unwrap();

    file_path.to_str().unwrap().to_string()
}
