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
/// Represents a site administrator.
pub struct SiteAdmin;

/// Methods for allowing access control.
pub trait AccessControl {
    /// The name of the environment variable to check for this user.
    const KEY: Option<&'static str>;
}

macro_rules! control_vars {
    ($($struct:path => $statement:expr,)*) => {
        $(impl AccessControl for $struct {
            const KEY: Option<&'static str> = $statement;
        })*
    };
}

control_vars! {
    Generic => None,
    Member => Some("BARBELL_MEMBERS"),
    TaskmasterAdmin => Some("TASKMASTER_ADMINS"),
    ElectionAdmin => Some("ELECTION_ADMINS"),
    SiteAdmin => Some("SITE_ADMINS"),
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

        U::KEY
            .map(|key| env::var(key).expect("Failed to get environment variable"))
            .map(|value| value.split(',').any(|v| v == id))
            .unwrap_or_default()
    }

    fn environment_contains(value: &str) -> bool {
        T::KEY
            .map(|key| {
                // Get the environment variable and parse it
                let var = env::var(key).expect("Failed to get environment variable");
                let contains = var.split(',').any(|v| v == value);

                if !contains {
                    log::warn!(
                        "user_id={} was not found in the following environment variable: {}",
                        value,
                        key
                    );
                }

                contains
            })
            .unwrap_or(true)
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

        if !Self::environment_contains(id.value()) {
            return forbidden;
        }

        Outcome::Success(Self {
            id: i32::from_str(id.value()).unwrap(),
            name: String::from(name.value()),
            level: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_admins_can_be_checked() {
        // Place the values in the environment variable
        env::set_var("SITE_ADMINS", "1702502");

        assert!(User::<SiteAdmin>::environment_contains("1702502"));
    }

    #[test]
    fn multiple_users_can_be_within_environment_variables() {
        // Place the values in the environment variable
        env::set_var("BARBELL_MEMBERS", "1820900,1701229");

        assert!(User::<Member>::environment_contains("1820900"));
        assert!(User::<Member>::environment_contains("1701229"));
    }

    #[test]
    fn users_can_be_rejected_from_environment_variables() {
        // Place the values in the environment variable
        env::set_var("TASKMASTER_ADMINS", "1707704");

        assert!(!User::<TaskmasterAdmin>::environment_contains("1702502"));
    }
}
