use dotenv::dotenv;
use std::env;

pub struct Config {
    pub spreadsheet_id: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Config {
            spreadsheet_id: env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID not set"),
        }
    }
}
