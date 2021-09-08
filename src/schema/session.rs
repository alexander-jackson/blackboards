//! Allows modifications of the `sessions` table in the database.

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rand::Rng;

use crate::schema::custom_types;
use crate::session_window::SessionWindow;

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
    /// Creates a new instance of a [`Session`].
    pub fn create_and_insert(
        conn: &diesel::PgConnection,
        title: String,
        start_time: i64,
        remaining: u32,
    ) -> QueryResult<usize> {
        // Generate a random identifier and check it
        let id = loop {
            let potential = rand::thread_rng().gen::<i32>().abs();
            let exists = diesel::select(diesel::dsl::exists(
                sessions::dsl::sessions.filter(sessions::dsl::id.eq(potential)),
            ))
            .get_result(conn);

            if let Ok(false) = exists {
                break potential;
            }
        };

        log::info!(
            "Creating a new session with id={}, title='{}', timestamp={} and remaining={}",
            id,
            title,
            start_time,
            remaining
        );

        let session = Self {
            id,
            title,
            start_time: custom_types::DateTime::new(start_time),
            remaining: remaining as i32,
        };

        diesel::insert_into(sessions::table)
            .values(session)
            .execute(conn)
    }

    /// Gets all available sessions currently in the database.
    pub fn get_results(conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
        sessions::dsl::sessions
            .order_by(sessions::dsl::start_time.asc())
            .get_results::<Self>(conn)
    }

    /// Gets all available sessions currently in the database.
    pub fn get_results_between(
        conn: &diesel::PgConnection,
        window: SessionWindow,
    ) -> QueryResult<Vec<Self>> {
        let filter = sessions::dsl::start_time
            .gt(window.start)
            .and(sessions::dsl::start_time.lt(window.end));

        log::debug!("Getting all the sessions in window={:?}", window);

        sessions::dsl::sessions
            .filter(filter)
            .order_by(sessions::dsl::start_time.asc())
            .get_results::<Self>(conn)
    }

    /// Gets all available sessions in the window or after.
    pub fn get_results_within_and_after(
        conn: &diesel::PgConnection,
        window: SessionWindow,
    ) -> QueryResult<Vec<Self>> {
        let filter = sessions::dsl::start_time.gt(window.start);

        log::debug!("Getting all the sessions after time={}", window.start);

        sessions::dsl::sessions
            .filter(filter)
            .order_by(sessions::dsl::start_time.asc())
            .get_results::<Self>(conn)
    }

    /// Finds a session in the database given its identifier.
    pub fn find(id: i32, conn: &diesel::PgConnection) -> QueryResult<Self> {
        sessions::dsl::sessions.find(id).first::<Session>(conn)
    }

    /// Deletes the session with the given identifier.
    pub fn delete(conn: &diesel::PgConnection, id: i32) -> QueryResult<usize> {
        log::warn!("Deleting session with id={}, including registrations", id);

        diesel::delete(sessions::dsl::sessions.filter(sessions::dsl::id.eq(id))).execute(conn)
    }

    /// Decreases the number of remaining places for a session given its identifier.
    pub fn decrement_remaining(id: i32, conn: &diesel::PgConnection) -> QueryResult<usize> {
        let current = Self::find(id, conn)?.remaining;

        log::debug!(
            "Decrementing remaining places for session_id={}, currently has {}",
            id,
            current
        );

        diesel::update(sessions::dsl::sessions.filter(sessions::dsl::id.eq(&id)))
            .set(sessions::dsl::remaining.eq(current - 1))
            .execute(conn)
    }

    /// Increases the number of remaining places for a session given its identifier.
    pub fn increment_remaining(id: i32, conn: &diesel::PgConnection) -> QueryResult<usize> {
        let current = Self::find(id, conn)?.remaining;

        log::debug!(
            "Incrementing remaining places for session_id={}, currently has {}",
            id,
            current
        );

        diesel::update(sessions::dsl::sessions.filter(sessions::dsl::id.eq(&id)))
            .set(sessions::dsl::remaining.eq(current + 1))
            .execute(conn)
    }
}
