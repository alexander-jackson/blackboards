//! Stores custom request guards for Rocket routes.

use std::str::FromStr;
use std::{env, marker::PhantomData};

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use reqwest::StatusCode;
use serde::Deserialize;
use tower_cookies::{Cookies, Key};

/// Represents a generic user who is at Warwick.
pub struct Generic;
/// Represents a member of Warwick Barbell.
pub struct Member;
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
                    tracing::warn!(
                        user_id = %value,
                        environment_variable = %key,
                        "Failed to find user in the environment variable"
                    );
                }

                contains
            })
            .unwrap_or(true)
    }
}

#[async_trait]
impl<T: AccessControl, S> FromRequestParts<S> for User<T>
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state).await?;

        let key = Key::from(b"D4BAA06CFF71992CF798B05D714C23B1917887BB17AC6B1B5AD223156CFC4CE4D9050C88F4F1C2720C759F6357D83B34AF2E031D59647D0C55E4ECC4C2B05944");
        let cookies = cookies.private(&key);

        let id = cookies
            .get("id")
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        let name = cookies
            .get("name")
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Unauthorized"))?;

        if !Self::environment_contains(id.value()) {
            return Err((StatusCode::FORBIDDEN, "Forbidden"));
        }

        let user = User {
            id: i32::from_str(id.value()).unwrap(),
            name: String::from(name.value()),
            level: PhantomData,
        };

        Ok(user)
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
        env::set_var("ELECTION_ADMINS", "1707704");

        assert!(!User::<ElectionAdmin>::environment_contains("1702502"));
    }
}
