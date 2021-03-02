//! Stores the Tera context's needed for rendering the frontend webpages.

use std::collections::HashMap;

use rocket::request::FlashMessage;

use crate::schema;

/// Represents the session title, start time and the users' registered for it.
pub type Registrations = ((String, String), Vec<String>);

/// Represents a flash message, but including the variant.
#[derive(Serialize)]
pub struct Message {
    /// The type of message, such as "error" or "success"
    pub variant: String,
    /// The message to display
    pub message: String,
}

impl From<FlashMessage<'_, '_>> for Message {
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
    /// The Warwick ID of the viewer
    pub user_id: i32,
}

/// The context for updating personal bests.
#[derive(Serialize)]
pub struct PersonalBests {
    /// The user's personal bests
    pub personal_bests: schema::PersonalBest,
    /// The message to display to the user, for errors
    pub message: Option<Message>,
}

/// The context for displaying the Taskmaster leaderboard.
#[derive(Serialize)]
pub struct TaskmasterLeaderboard {
    /// The state of the leaderboard
    pub leaderboard: Vec<schema::TaskmasterEntry>,
    /// Whether the user has permission to edit the board
    pub admin: bool,
    /// The message to display to the user, for errors
    pub message: Option<Message>,
}

/// The context for displaying the Taskmaster leaderboard.
#[derive(Serialize)]
pub struct TaskmasterEdit {
    /// The state of the leaderboard, as a CSV
    pub leaderboard_csv: String,
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
    /// The positions to show
    pub nominations: Vec<schema::Nomination>,
    /// The position we are voting for
    pub position_id: i32,
    /// The message to display to the user, for errors
    pub message: Option<Message>,
}

/// The result of a single election on a position.
#[derive(Serialize)]
pub struct ElectionResult<'a> {
    /// The title of the position
    pub title: String,
    /// The winner, if there was one
    pub winner: Option<&'a str>,
    /// The candidates who tied, if there was one
    pub tie: Option<Vec<&'a str>>,
}

/// The context for displaying the election results.
#[derive(Serialize)]
pub struct ElectionResults<'a> {
    /// The results of each election
    pub results: Vec<ElectionResult<'a>>,
}

/// Returns an empty `HashMap` for templates that don't require context.
pub fn get_empty() -> HashMap<&'static str, &'static str> {
    HashMap::new()
}
