//! Stores the Tera context's needed for rendering the frontend webpages.

use std::collections::HashMap;

use rocket::request::FlashMessage;

use crate::schema;

/// Represents the registrations for a given session.
#[derive(Debug, Serialize)]
pub struct Registrations {
    /// The starting time of the session
    pub start_time: schema::custom_types::DateTime,
    /// The title of the session
    pub title: String,
    /// The names of the members signed up for it
    pub members: Vec<String>,
}

/// Represents a flash message, but including the variant.
#[derive(Serialize)]
pub struct Message {
    /// The type of message, such as "error" or "success"
    pub variant: String,
    /// The message to display
    pub message: String,
}

impl From<FlashMessage<'_>> for Message {
    fn from(flash: FlashMessage) -> Self {
        Self {
            variant: flash.name().to_string(),
            message: flash.msg().to_string(),
        }
    }
}

/// The context for session registrations.
#[derive(Serialize)]
pub struct Context {
    /// The sessions that are available.
    pub sessions: Vec<schema::Session>,
    /// The currently selected session if it exists.
    pub current: Option<schema::Session>,
    /// The message to display to the user, for errors.
    pub message: Option<Message>,
    /// The registrations for each session.
    pub registrations: Option<Vec<Registrations>>,
    /// Whether or not the user is a site administrator.
    pub is_site_admin: bool,
}

/// The context for managing upcoming sessions.
#[derive(Serialize)]
pub struct ManageSessions {
    /// The sessions that are available.
    pub sessions: Vec<schema::Session>,
    /// The session currently being managed, if one is
    pub current: Option<schema::Session>,
    /// The message to display to the user, for errors.
    pub message: Option<Message>,
}

/// The context for automatically redirecting on authentication.
#[derive(Serialize)]
pub struct Authenticated {
    /// The path to redirect the user to.
    pub uri: String,
}

/// The context for attendance registrations.
#[derive(Serialize)]
pub struct Attendance {
    /// The sessions that are available.
    pub sessions: Vec<schema::Session>,
    /// The currently selected session if it exists.
    pub current: Option<schema::Session>,
    /// The message to display to the user, for errors.
    pub message: Option<Message>,
}

/// The context for the blackboards page.
#[derive(Serialize)]
pub struct Blackboard {
    /// The recorded personal bests for each PL user
    pub pl: Vec<schema::PersonalBest>,
    /// The recorded personal bests for each WL user
    pub wl: Vec<schema::PersonalBest>,
    /// The Warwick ID of the viewer if they are logged in
    pub user_id: Option<i32>,
}

/// The context for updating personal bests.
#[derive(Serialize)]
pub struct PersonalBests {
    /// The user's personal bests
    pub personal_bests: schema::PersonalBest,
    /// The message to display to the user, for errors
    pub message: Option<Message>,
}

/// The context for displaying the exec positions.
#[derive(Serialize)]
pub struct Elections {
    /// The positions to show
    pub exec_positions: Vec<schema::ExecPosition>,
    /// The message to display to the user, for errors
    pub message: Option<Message>,
    /// Whether or not the user is an election administrator
    pub admin: bool,
}

/// The context for displaying the voting page.
#[derive(Serialize)]
pub struct Voting {
    /// The position we are voting for
    pub position_id: i32,
    /// The title of the position itself.
    pub position_title: String,
    /// The positions to show
    pub nominations: Vec<(i32, String)>,
    /// The user's current votes for this position, if they have voted
    pub current_ballot: Option<Vec<String>>,
    /// The message to display to the user, for errors
    pub message: Option<Message>,
}

/// The result of a single election on a position.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ElectionResult<'a> {
    /// The identifier of the position.
    pub position_id: i32,
    /// The title of the position
    pub title: String,
    /// The people who won the election
    pub winners: Vec<(i32, &'a str, usize)>,
    /// The number of people who voted
    pub voter_count: usize,
}

/// The context for displaying the election results.
#[derive(Serialize)]
pub struct ElectionResults<'a> {
    /// The results of each election
    pub results: Vec<ElectionResult<'a>>,
}

/// The context for displaying the `403 Forbidden` page.
#[derive(Serialize)]
pub struct Forbidden<'a> {
    /// The path the user was requesting
    pub path: &'a str,
}

/// Returns an empty `HashMap` for templates that don't require context.
pub fn get_empty() -> HashMap<&'static str, &'static str> {
    HashMap::new()
}
