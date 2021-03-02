//! Stores custom request guards for Rocket routes.

use std::env;
use std::str::FromStr;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[database("sqlite_database")]
pub struct DatabaseConnection(diesel::SqliteConnection);

/// Represents an authorised user for a given route.
#[derive(Debug)]
pub struct AuthorisedUser {
    /// The user's Warwick ID
    pub id: i32,
    /// The user's name
    pub name: String,
}

impl AuthorisedUser {
    /// Returns true if the user is a Taskmaster administrator.
    pub fn is_taskmaster_admin(&self) -> bool {
        // Get the environment variable and parse it
        let var = match env::var("TASKMASTER_ADMINS") {
            Ok(value) => value,
            Err(_) => return false,
        };

        var.split(',')
            .find(|v| i32::from_str(v) == Ok(self.id))
            .is_some()
    }

    /// Returns true if the user is a election administrator.
    pub fn is_election_admin(&self) -> bool {
        // Get the environment variable and parse it
        let var = match env::var("ELECTION_ADMINS") {
            Ok(value) => value,
            Err(_) => return false,
        };

        var.split(',')
            .find(|v| i32::from_str(v) == Ok(self.id))
            .is_some()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorisedUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let failure = Outcome::Failure((Status::Unauthorized, ()));

        let id = match request.cookies().get_private("id") {
            Some(id) => id,
            None => return failure,
        };
        let name = match request.cookies().get_private("name") {
            Some(name) => name,
            None => return failure,
        };

        Outcome::Success(Self {
            id: i32::from_str(id.value()).unwrap(),
            name: String::from(name.value()),
        })
    }
}
