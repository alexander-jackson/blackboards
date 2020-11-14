//! Allows modifications of the `auth_pairs` table in the database.

use std::borrow::Cow;

use diesel::{QueryDsl, QueryResult, RunQueryDsl};

use crate::auth;

table! {
    /// Represents the schema for `auth_pairs`.
    auth_pairs (token) {
        /// The user's OAuth token.
        token -> Text,
        /// The user's OAuth secret.
        secret -> Text,
    }
}

/// Represents a row in the `auth_pairs` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct AuthPair {
    /// The user's OAuth token.
    pub token: String,
    /// The user's OAuth secret.
    pub secret: String,
}

impl AuthPair {
    /// Inserts the data into the appropriate table.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(auth_pairs::table)
            .values(self)
            .execute(conn)
    }

    /// Finds an [`AuthPair`] given a token.
    pub fn find(token: &str, conn: &diesel::SqliteConnection) -> QueryResult<Self> {
        auth_pairs::dsl::auth_pairs.find(token).first::<Self>(conn)
    }
}

impl From<auth::TokenPair> for AuthPair {
    fn from(pair: auth::TokenPair) -> Self {
        Self {
            token: pair.token,
            secret: pair.secret,
        }
    }
}

impl From<(&Cow<'_, str>, &Cow<'_, str>)> for AuthPair {
    fn from(tuple: (&Cow<'_, str>, &Cow<'_, str>)) -> Self {
        Self {
            token: tuple.0.to_string(),
            secret: tuple.1.to_string(),
        }
    }
}
