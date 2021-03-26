//! Stores custom request guards for Rocket routes.

// This is only really for `DatabaseConnection`
#![allow(missing_docs)]

use std::env;
use std::str::FromStr;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[database("blackboards")]
pub struct DatabaseConnection(diesel::PgConnection);

/// Represents an authorised user for a given route.
#[derive(Debug)]
pub struct AuthorisedUser {
    /// The user's Warwick ID
    pub id: i32,
    /// The user's name
    pub name: String,
}

impl AuthorisedUser {
    /// Returns true if the user is a member of the given environment variable.
    fn user_id_in_var(&self, var: &str) -> bool {
        // Get the environment variable and parse it
        let var = match env::var(var) {
            Ok(value) => value,
            Err(_) => return false,
        };

        var.split(',').any(|v| i32::from_str(v) == Ok(self.id))
    }

    /// Returns true if the user is a Taskmaster administrator.
    pub fn is_taskmaster_admin(&self) -> bool {
        self.user_id_in_var("TASKMASTER_ADMINS")
    }

    /// Returns true if the user is a election administrator.
    pub fn is_election_admin(&self) -> bool {
        self.user_id_in_var("ELECTION_ADMINS")
    }

    /// Returns true if the user is a member of the club.
    pub fn is_barbell_member(&self) -> bool {
        self.user_id_in_var("BARBELL_MEMBERS")
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorisedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
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
