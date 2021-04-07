//! Stores custom request guards for Rocket routes.

// This is only really for `DatabaseConnection`
#![allow(missing_docs)]

use std::str::FromStr;
use std::{env, marker::PhantomData};

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[database("blackboards")]
pub struct DatabaseConnection(diesel::PgConnection);

/// Represents a generic user who is at Warwick.
pub struct Generic;
/// Represents a member of Warwick Barbell.
pub struct Member;
/// Represents a Taskmaster administrator.
pub struct TaskmasterAdmin;
/// Represents a election administrator.
pub struct ElectionAdmin;

/// Methods for allowing access control.
pub trait AccessControl {
    /// The environment variable to check for this user.
    fn env_var() -> Option<&'static str>;
}

macro_rules! control_vars {
    ($($struct:path => $statement:expr,)*) => {
        $(impl AccessControl for $struct {
            fn env_var() -> Option<&'static str> {
                $statement
            }
        })*
    };
}

control_vars! {
    Generic => None,
    Member => Some("BARBELL_MEMBERS"),
    TaskmasterAdmin => Some("TASKMASTER_ADMINS"),
    ElectionAdmin => Some("ELECTION_ADMINS"),
}

/// Represents an authorised user for a given route.
#[derive(Debug, Deserialize)]
pub struct User<T: AccessControl> {
    /// The user's Warwick ID
    pub id: i32,
    /// The user's name
    pub name: String,
    /// The privilege level of the user.
    level: PhantomData<T>,
}

impl<T: AccessControl> User<T> {
    /// Checks whether the given user is also a member of another environment variable.
    pub fn is_also<U: AccessControl>(&self) -> bool {
        let id = self.id.to_string();

        U::env_var()
            .map(|key| env::var(key).expect("Failed to get environment variable"))
            .map(|value| value.split(',').any(|v| v == id))
            .unwrap_or_default()
    }
}

#[rocket::async_trait]
impl<T: AccessControl, 'r> FromRequest<'r> for User<T> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let unauthorised = Outcome::Failure((Status::Unauthorized, ()));
        let forbidden = Outcome::Failure((Status::Forbidden, ()));

        let id = match request.cookies().get_private("id") {
            Some(id) => id,
            None => return unauthorised,
        };

        let name = match request.cookies().get_private("name") {
            Some(name) => name,
            None => return unauthorised,
        };

        if let Some(key) = T::env_var() {
            // Get the environment variable and parse it
            let var = env::var(key).expect("Failed to get environment variable");

            if !var.split(',').any(|v| v == id.value()) {
                log::warn!(
                    "user_id={} was not found in the following environment variable: {}",
                    id.value(),
                    key
                );

                return forbidden;
            }

            log::debug!(
                target: "blackboards",
                "user_id={} was found in the following environment variable: {}",
                id.value(),
                key
            );
        }

        Outcome::Success(Self {
            id: i32::from_str(id.value()).unwrap(),
            name: String::from(name.value()),
            level: PhantomData,
        })
    }
}
