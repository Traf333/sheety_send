use dotenv::dotenv;
use std::env;

pub struct Config {
    pub spreadsheet_id: String,
    pub recipient_email: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Config {
            spreadsheet_id: env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID not set"),
            recipient_email: env::var("RECIPIENT_EMAIL").expect("RECIPIENT_EMAIL not set"),
            smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set"),
            smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set"),
        }
    }
}
