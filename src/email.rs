// // src/email.rs

// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Message, SmtpTransport, Transport};
// use std::fs::File;

// pub fn send_email(config: &crate::config::Config) {
//     let email = Message::builder()
//         .from("you@example.com".parse().unwrap())
//         .to(config.recipient_email.parse().unwrap())
//         .subject("Spreadsheet Data")
//         .body(String::from("Please find the spreadsheet data attached."))
//         .unwrap()
//         .attachment(
//             File::open("sheet_data.txt").unwrap(),
//             "sheet_data.txt",
//             &mime::TEXT_PLAIN,
//         )
//         .unwrap();

//     let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

//     let mailer = SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();

//     match mailer.send(&email) {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => panic!("Could not send email: {:?}", e),
//     }
// }
