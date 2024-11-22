use log::{info, error};

mod config;

mod spreadsheet;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    info!("Starting sheety_send application...");

    let config = match config::Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    info!("Configuration loaded successfully");

    match spreadsheet::fetch_and_save_sheet_data(&config).await {
        Ok(file_path) => {
            info!("Successfully saved spreadsheet to: {}", file_path);
        }
        Err(e) => {
            error!("Failed to fetch and save spreadsheet: {}", e);
            std::process::exit(1);
        }
    }

    info!("Application completed successfully");
}
