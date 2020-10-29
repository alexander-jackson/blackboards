//! Represents the Rust side of the database schema and the tables within it.

pub mod attendance;
pub mod custom_types;
pub mod registration;
pub mod request;
pub mod session;
pub mod verified_email;

pub use attendance::{attendances, Attendance};
pub use registration::{registrations, Registration};
pub use request::{requests, Request};
pub use session::{sessions, Session};
pub use verified_email::{verified_emails, VerifiedEmail};

joinable!(registrations -> sessions (session_id));
allow_tables_to_appear_in_same_query!(registrations, sessions);
