// use std::env;

pub struct Config {
    pub spreadsheet_id: String,
    // pub recipient_email: String,
    // pub smtp_username: String,
    // pub smtp_password: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            spreadsheet_id: "1xS2W2HoyAJ39W-KPuPgJ6gWjOpBzoYKiM03yhswW3P0".to_string(),
            // spreadsheet_id: env::var("SPREADSHEET_URL").expect("SPREADSHEET_URL not set"),
            // recipient_email: env::var("RECIPIENT_EMAIL").expect("RECIPIENT_EMAIL not set"),
            // smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set"),
            // smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set"),
        }
    }
}
