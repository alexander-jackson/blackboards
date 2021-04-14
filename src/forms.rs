//! Stores the expected structure of various forms for the user to submit.

use rocket::form::{self, FromFormField, ValueField};

/// Defines a custom struct that can only contain a valid Warwick ID.
#[derive(Copy, Clone, Debug)]
pub struct WarwickId(pub i32);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for WarwickId {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let value = field.value;

        if !(value.chars().all(char::is_numeric) && value.len() == 7) {
            return Err(form::Error::validation(
                "Value was either not numeric or incorrect length",
            )
            .into());
        }

        Ok(Self(value.parse::<i32>().unwrap()))
    }
}

/// Defines the information needed to create a new session.
#[derive(Debug, FromForm)]
pub struct SessionCreate {
    /// The title of the session.
    pub title: String,
    /// The number of available spaces.
    pub spaces: u32,
    /// The date of the session.
    pub date: String,
    /// The starting time of the session.
    pub start_time: String,
}

/// Defines the contents of the registration form for a session.
#[derive(Debug, FromForm)]
pub struct Register {
    /// The identifier for the session.
    pub session_id: i32,
}

/// Defines the contents of the cancellation form for a session.
#[derive(Debug, FromForm)]
pub struct Cancel {
    /// The identifier for the session.
    pub session_id: i32,
}

/// Defines the contents of the attendance form for a session.
#[derive(Copy, Clone, Debug, FromForm)]
pub struct Attendance {
    /// The identifier for the session.
    pub session_id: i32,
    /// The user's Warwick ID.
    pub warwick_id: WarwickId,
}

/// Defines the contents of the personal bests form.
#[derive(Clone, Debug, FromForm)]
pub struct PersonalBests {
    /// The user's best squat.
    pub squat: Option<f32>,
    /// The user's best bench.
    pub bench: Option<f32>,
    /// The user's best deadlift.
    pub deadlift: Option<f32>,
    /// The user's best snatch.
    pub snatch: Option<f32>,
    /// The user's best clean and jerk.
    pub clean_and_jerk: Option<f32>,
    /// Whether to display the user on the PL board.
    pub show_pl: bool,
    /// Whether to display the user on the WL board.
    pub show_wl: bool,
}

/// Defines the contents of a taskmaster board update.
#[derive(Debug, FromForm)]
pub struct TaskmasterUpdate {
    /// The CSV representing the new board state.
    pub leaderboard: String,
}
