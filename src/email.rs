use std::env;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::schema;

pub fn confirm_email_address(request: &schema::Request, session: &schema::Session) {
    let sender = env::var("EMAIL").expect("Failed to find an EMAIL in .env");
    let app_password = env::var("APP_PASSWORD").expect("Failed to find an APP_PASSWORD in .env");

    let to = format!("{} <{}>", request.name, request.email);
    let body = format!("To confirm your booking for {} on {}, please click the following link: https://blackboards.pl/session/confirm/{}", session.title, session.start_time, request.identifier);

    let email = Message::builder()
        .from("Alexander Jackson <alexj6868@gmail.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Warwick Barbell Session Registration")
        .body(body)
        .unwrap();

    let creds = Credentials::new(sender, app_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
}

pub fn send_confirmation_email(registration: &schema::Registration, session: &schema::Session) {
    let sender = env::var("EMAIL").expect("Failed to find an EMAIL in .env");
    let app_password = env::var("APP_PASSWORD").expect("Failed to find an APP_PASSWORD in .env");

    let to = format!("{} <{}>", registration.name, registration.email);
    let body = format!(
        "Your booking for {} at {} has been confirmed!",
        session.title, session.start_time
    );

    let email = Message::builder()
        .from("Alexander Jackson <alexj6868@gmail.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Warwick Barbell Session Confirmation")
        .body(body)
        .unwrap();

    let creds = Credentials::new(sender, app_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
}
