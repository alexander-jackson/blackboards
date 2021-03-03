//! Represents the Rust side of the database schema and the tables within it.

pub mod attendance;
pub mod auth_pair;
pub mod custom_types;
pub mod exec_position;
pub mod nomination;
pub mod personal_best;
pub mod registration;
pub mod session;
pub mod taskmaster_entry;
pub mod vote;

pub use attendance::{attendances, Attendance};
pub use auth_pair::{auth_pairs, AuthPair};
pub use exec_position::{exec_positions, ExecPosition};
pub use nomination::{nominations, Nomination};
pub use personal_best::{personal_bests, PersonalBest};
pub use registration::{registrations, Registration};
pub use session::{sessions, Session};
pub use taskmaster_entry::{taskmaster_entries, TaskmasterEntry};
pub use vote::{votes, Vote};

joinable!(registrations -> sessions (session_id));
allow_tables_to_appear_in_same_query!(registrations, sessions);
allow_tables_to_appear_in_same_query!(nominations, votes);
