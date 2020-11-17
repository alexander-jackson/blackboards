//! Stores the Tera context's needed for rendering the frontend webpages.

use std::collections::HashMap;

use crate::schema;

/// Represents the session title, start time and the users' registered for it.
pub type Registrations = ((String, String), Vec<String>);

/// The context for session registrations.
#[derive(Serialize)]
pub struct Context {
    /// The sessions that are available.
    pub sessions: Vec<schema::Session>,
    /// The currently selected session if it exists.
    pub current: Option<schema::Session>,
    /// The message to display to the user, for errors.
    pub message: Option<String>,
    /// The registrations for each session.
    pub registrations: Option<Vec<Registrations>>,
}

/// The context for attendance registrations.
#[derive(Serialize)]
pub struct Attendance {
    /// The sessions that are available.
    pub sessions: Vec<schema::Session>,
    /// The currently selected session if it exists.
    pub current: Option<schema::Session>,
    /// The message to display to the user, for errors.
    pub message: Option<String>,
}

/// The context for the blackboards page.
#[derive(Serialize)]
pub struct Blackboard {
    /// The recorded personal bests for each PL user
    pub pl: Vec<schema::PersonalBest>,
    /// The recorded personal bests for each WL user
    pub wl: Vec<schema::PersonalBest>,
    /// The Warwick ID of the viewer
    pub user_id: i32,
}

/// The context for updating personal bests.
#[derive(Serialize)]
pub struct PersonalBests {
    /// The user's personal bests
    pub personal_bests: schema::PersonalBest,
    /// The message to display to the user, for errors
    pub message: Option<String>,
}

/// Returns an empty `HashMap` for templates that don't require context.
pub fn get_empty() -> HashMap<&'static str, &'static str> {
    HashMap::new()
}
