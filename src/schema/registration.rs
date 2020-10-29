//! Allows modifications of the `registrations` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::email;
use crate::forms;
use crate::schema::{custom_types, sessions, Request, Session};

table! {
    /// Represents the schema for `registrations`.
    registrations (warwick_id) {
        /// The identifier for the session.
        session_id -> Integer,
        /// The user's Warwick ID.
        warwick_id -> Integer,
        /// The user's name.
        name -> Text,
    }
}

/// Represents a row in the `registrations` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Registration {
    /// The identifier for the session
    pub session_id: i32,
    /// The user's Warwick ID
    pub warwick_id: i32,
    /// The user's name
    pub name: String,
}

impl Registration {
    /// Inserts the [`Registration`] into the database.
    ///
    /// This fails if the session has no remaining places, and sends the user a confirmation email
    /// upon success. It also decrements the number of remaining places for the given session.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Ensure the session has spaces
        let session = Session::find(self.session_id, conn)?;

        if session.remaining == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        diesel::insert_into(registrations::table)
            .values(self)
            .execute(conn)?;

        email::send_confirmation(&self, &session);

        Session::decrement_remaining(self.session_id, conn)
    }

    /// Gets the number of registrations for a given session.
    pub fn count(session_id: i32, conn: &diesel::SqliteConnection) -> QueryResult<i64> {
        registrations::dsl::registrations
            .filter(registrations::dsl::session_id.eq(&session_id))
            .count()
            .get_result(conn)
    }

    /// Gets the session data and names of those registered for all sessions in the database.
    pub fn get_registration_list(
        conn: &diesel::SqliteConnection,
    ) -> QueryResult<Vec<(i32, custom_types::DateTime, String, String)>> {
        let columns = (
            sessions::dsl::id,
            sessions::dsl::start_time,
            sessions::dsl::title,
            registrations::dsl::name,
        );
        let ordering = (sessions::dsl::start_time, sessions::dsl::id);

        registrations::dsl::registrations
            .inner_join(sessions::dsl::sessions)
            .select(columns)
            .order_by(ordering)
            .load(conn)
    }
}

impl From<Request> for Registration {
    /// Creates a new [`Registration`] from a [`Request`].
    fn from(data: Request) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id,
            name: data.name,
        }
    }
}

impl From<forms::Register> for Registration {
    /// Creates a new [`Registration`] assuming the user is already verified.
    fn from(data: forms::Register) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id.0,
            name: data.name,
        }
    }
}
