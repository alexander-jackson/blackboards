//! Stores custom request guards for Rocket routes.

#[database("sqlite_database")]
pub struct DatabaseConnection(diesel::SqliteConnection);
