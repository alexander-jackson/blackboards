//! Stores the expected structure of various forms for the user to submit.

use rocket::http::RawStr;
use rocket::request::FromFormValue;

/// Defines a custom struct that can only contain a valid Warwick ID.
#[derive(Copy, Clone, Debug)]
pub struct WarwickId(pub i32);

impl<'v> FromFormValue<'v> for WarwickId {
    type Error = &'v RawStr;

    /// Converts a form value into a `WarwickId` if it matches the length.
    fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
        if !(form_value.chars().all(char::is_numeric) && form_value.len() == 7) {
            return Err(form_value);
        }

        Ok(Self(form_value.parse::<i32>().unwrap()))
    }
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
