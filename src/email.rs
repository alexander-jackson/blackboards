use std::env;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::schema;

#[derive(Debug)]
pub struct Config {
    pub from_address: String,
    pub from_name: String,
    pub app_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            from_address: env::var("FROM_ADDRESS")?,
            from_name: env::var("FROM_NAME")?,
            app_password: env::var("APP_PASSWORD")?,
        })
    }
}

fn format_warwick_email(warwick_id: i32) -> String {
    format!("u{}@live.warwick.ac.uk", warwick_id)
}

pub fn confirm_address(request: &schema::Request, session: &schema::Session) {
    // Check whether email settings are on
    if env::var("SEND_EMAILS").is_err() {
        return;
    }

    let config = Config::from_env().expect("Config was malformed");

    let from = format!("{} <{}>", config.from_name, config.from_address);
    let to = format!(
        "{} <{}>",
        request.name,
        format_warwick_email(request.warwick_id),
    );
    let body = format!(
        r#"Hey {},

To confirm your booking for {} on {}, please click the following link:
https://blackboards.pl/session/confirm/{}

Thanks!"#,
        request.name, session.title, session.start_time, request.identifier
    );

    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Warwick Barbell Session Registration")
        .body(body)
        .unwrap();

    let creds = Credentials::new(config.from_address, config.app_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
}

pub fn send_confirmation(registration: &schema::Registration, session: &schema::Session) {
    // Check whether email settings are on
    if env::var("SEND_EMAILS").is_err() {
        return;
    }

    let config = Config::from_env().expect("Config was malformed");

    let from = format!("{} <{}>", config.from_name, config.from_address);
    let to = format!(
        "{} <{}>",
        registration.name,
        format_warwick_email(registration.warwick_id)
    );
    let body = format!(
        r#"Hey {},

Your booking for {} at {} has been confirmed, see you there!"#,
        registration.name, session.title, session.start_time
    );

    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Warwick Barbell Session Confirmation")
        .body(body)
        .unwrap();

    let creds = Credentials::new(config.from_address, config.app_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
}
