# Rust Spreadsheet Downloader and Emailer

This Rust application downloads a public Google Sheets spreadsheet as an `.xlsx` file and sends it via email as an attachment.

## Features

- Fetches a public Google Sheets spreadsheet using its URL.
- Saves the spreadsheet as an `.xlsx` file.
- Sends the saved file as an email attachment.

## Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install)).
- SMTP server credentials for sending emails (e.g., Gmail SMTP).

## Setup

1. **Clone the repository:**

```bash
   git clone https://github.com/your_username/your_project.git
   cd your_project
```

2. **Create a .env file in the project root:**

```
SPREADSHEET_URL="https://docs.google.com/spreadsheets/d/YOUR_SPREADSHEET_ID/edit#gid=0"
RECIPIENT_EMAIL="recipient@example.com"
SMTP_USERNAME="your_smtp_username"
SMTP_PASSWORD="your_smtp_password"
```

Replace placeholders (YOUR_SPREADSHEET_ID, recipient@example.com, your_smtp_username, your_smtp_password) with your actual values.

Build and run the application:

```
cargo run
```
This will fetch the spreadsheet, save it as sheet_data.xlsx, and send it to the specified email address.
