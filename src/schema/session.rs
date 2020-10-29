//! Allows modifications of the `sessions` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::schema::custom_types;

table! {
    /// Represents the schema for `sessions`.
    sessions {
        /// The identifier for the session.
        id -> Integer,
        /// The title for the session.
        title -> Text,
        /// The starting time for the session.
        start_time -> BigInt,
        /// The number of remaining places.
        remaining -> Integer,
    }
}

/// Represents a session in the database.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Session {
    /// The identifier for the session.
    pub id: i32,
    /// The title for the session.
    pub title: String,
    /// The starting time for the session.
    pub start_time: custom_types::DateTime,
    /// The number of remaining places.
    pub remaining: i32,
}

impl Session {
    /// Gets all available sessions currently in the database.
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        sessions::dsl::sessions
            .order_by(sessions::dsl::start_time.asc())
            .get_results::<Self>(conn)
    }

    /// Finds a session in the database given its identifier.
    pub fn find(id: i32, conn: &diesel::SqliteConnection) -> QueryResult<Self> {
        sessions::dsl::sessions.find(id).first::<Session>(conn)
    }

    /// Decreases the number of remaining places for a session given its identifier.
    pub fn decrement_remaining(id: i32, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        let current = Self::find(id, conn)?.remaining;

        diesel::update(sessions::dsl::sessions.filter(sessions::dsl::id.eq(&id)))
            .set(sessions::dsl::remaining.eq(current - 1))
            .execute(conn)
    }
}
