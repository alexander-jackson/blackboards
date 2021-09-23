//! Handles verification and confirmation emails for user registrations.

use std::env;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::schema::custom_types;

/// The configuration for sending emails.
#[derive(Debug)]
pub struct Config {
    /// The address to send from.
    pub from_address: String,
    /// The name to send from.
    pub from_name: String,
    /// The application specific password for Gmail.
    pub app_password: String,
}

impl Config {
    /// Builds a configuration from the environment variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            from_address: env::var("FROM_ADDRESS")?,
            from_name: env::var("FROM_NAME")?,
            app_password: env::var("APP_PASSWORD")?,
        })
    }
}

/// Formats a Warwick email address given an identifier.
fn format_warwick_email(warwick_id: i32) -> String {
    format!("u{}@live.warwick.ac.uk", warwick_id)
}

/// Sends an email to the user confirming their booking for a given session.
pub async fn send_confirmation(
    name: &str,
    warwick_id: i32,
    session_title: &str,
    start_time: custom_types::DateTime,
) {
    // Check whether email settings are on
    if env::var("SEND_EMAILS").is_err() {
        return;
    }

    let config = Config::from_env().expect("Config was malformed");

    let from = format!("{} <{}>", config.from_name, config.from_address);
    let to = format!("{} <{}>", name, format_warwick_email(warwick_id));
    let body = format!(
        r#"Hey {},

Your booking for {} at {} has been confirmed, see you there!"#,
        name, session_title, start_time
    );

    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Warwick Barbell Session Confirmation")
        .body(body)
        .unwrap();

    let creds = Credentials::new(config.from_address, config.app_password);

    // Open a remote connection to gmail
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer
        .send(email)
        .await
        .expect("Failed to send an email to the client");
}
