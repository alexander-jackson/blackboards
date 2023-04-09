//! Allows modifications of the `auth_pairs` table in the database.

use std::borrow::Cow;

use serde::Serialize;

use crate::auth;
use crate::schema::Pool;

/// Represents a row in the `auth_pairs` table.
#[derive(Debug, Serialize)]
pub struct AuthPair {
    /// The user's OAuth token.
    pub token: String,
    /// The user's OAuth secret.
    pub secret: Option<String>,
}

impl AuthPair {
    /// Inserts the data into the appropriate table.
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO auth_pairs (token, secret) VALUES ($1, $2)",
            self.token,
            self.secret
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Finds an [`AuthPair`] given a token.
    pub async fn find(token: &str, pool: &mut Pool) -> sqlx::Result<Self> {
        sqlx::query_as!(Self, "SELECT * FROM auth_pairs WHERE token = $1", token)
            .fetch_one(pool)
            .await
    }
}

impl From<auth::TokenPair> for AuthPair {
    fn from(pair: auth::TokenPair) -> Self {
        Self {
            token: pair.token,
            secret: Some(pair.secret),
        }
    }
}

impl From<(&Cow<'_, str>, &Cow<'_, str>)> for AuthPair {
    fn from(tuple: (&Cow<'_, str>, &Cow<'_, str>)) -> Self {
        Self {
            token: tuple.0.to_string(),
            secret: Some(tuple.1.to_string()),
        }
    }
}
