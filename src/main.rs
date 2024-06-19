mod config;
mod email;
mod spreadsheet;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();

    spreadsheet::fetch_and_save_sheet_data(&config).await;

    // email::send_email(&config);
}
