//! Represents the Rust side of the database schema and the tables within it.

use sqlx::{pool::PoolConnection, Postgres};

pub mod attendance;
pub mod auth_pair;
pub mod candidate;
pub mod custom_types;
pub mod exec_position;
pub mod nomination;
pub mod personal_best;
pub mod registration;
pub mod session;
pub mod vote;

pub use attendance::Attendance;
pub use auth_pair::AuthPair;
pub use candidate::Candidate;
pub use exec_position::ExecPosition;
pub use nomination::Nomination;
pub use personal_best::PersonalBest;
pub use registration::Registration;
pub use session::Session;
pub use vote::Vote;

/// Easier type for handling pooled connections.
pub type Pool = PoolConnection<Postgres>;
