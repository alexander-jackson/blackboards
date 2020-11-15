//! Stores custom request guards for Rocket routes.

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
