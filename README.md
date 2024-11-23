# Sheety Send - Google Sheets Downloader

A Rust application that takes snapshot of a public spreadsheet as an `.xlsx` file and saves it locally with the current date in the filename.

## Features

- Fetches a public Google Sheets spreadsheet using its ID
- Saves the spreadsheet as an `.xlsx` file with the current date in the filename
- Configurable output directory for downloaded files
- Comprehensive error handling and logging
- Automatic date-based file naming

## Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

## Setup

1. **Clone the repository:**

```bash
git clone git@github.com:Traf333/sheety_send.git
cd sheety_send
```

2. **Create a .env file in the project root:**

```env
# Required: Google Sheets ID from the spreadsheet URL
SPREADSHEET_ID="YOUR_SPREADSHEET_ID"

# Optional: Directory where files will be saved (defaults to current directory)
OUTPUT_DIR="./downloads"
```

### Configuration Options

- `SPREADSHEET_ID`: (Required) Your Google Sheets ID. You can find this in the spreadsheet's URL:
  `https://docs.google.com/spreadsheets/d/YOUR_SPREADSHEET_ID/edit`
- `OUTPUT_DIR`: (Optional) Directory where downloaded files will be saved. If not specified, files will be saved in the current directory. The directory must exist.

## Usage

Build and run the application:

```bash
cargo run
```

The application will:

1. Download the specified spreadsheet
2. Save it as `book-crossing-DD-MM-YYYY.xlsx` in the configured output directory
3. Log the process, including success or any errors

### Logging

To see detailed logs while running, use:

```bash
RUST_LOG=info cargo run
```
