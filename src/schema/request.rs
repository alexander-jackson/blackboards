//! Allows modifications of the `requests` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rand::Rng;

use crate::email;
use crate::forms;
use crate::schema::{Registration, Session, VerifiedEmail};

table! {
    /// Represents the schema for `requests`.
    requests (warwick_id) {
        /// The identifier for the session.
        session_id -> Integer,
        /// The user's Warwick ID.
        warwick_id -> Integer,
        /// The user's name.
        name -> Text,
        /// The unique identifier for the request
        identifier -> Integer,
    }
}

/// Represents a request to be verified for a session.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Request {
    /// The session requesting approval for
    pub session_id: i32,
    /// The user's Warwick ID
    pub warwick_id: i32,
    /// The user's name
    pub name: String,
    /// The unique identifier for the request
    pub identifier: i32,
}

impl Request {
    /// Creates a new [`Request`] from the registration form.
    pub fn create(data: forms::Register) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id.0,
            name: data.name,
            identifier: rand::thread_rng().gen::<i32>().abs(),
        }
    }

    /// Inserts the [`Request`] into the database.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Ensure the session has spaces
        let session = Session::find(self.session_id, conn)?;

        if session.remaining == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        // Email the user
        email::confirm_address(&self, &session);

        diesel::insert_into(requests::table)
            .values(self)
            .execute(conn)
    }

    /// Verifies a user request.
    ///
    /// This creates a new [`Registration`] to be inserted and deletes the current request. It then
    /// verifies the user's email with a [`VerifiedEmail`] and inserts that, before deleting the
    /// given request.
    pub fn verify(identifier: i32, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Find the request
        let request: Self = requests::dsl::requests
            .filter(requests::dsl::identifier.eq(&identifier))
            .first(conn)?;

        let registration = Registration::create(request);
        registration.insert(conn)?;

        // Add the user to the confirmed emails
        let verification = VerifiedEmail::create(registration.warwick_id, registration.name);
        verification.insert(conn)?;

        Request::delete(identifier, conn)
    }

    /// Deletes a [`Request`] from the database given the unique identifier the user is sent.
    pub fn delete(identifier: i32, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::delete(requests::dsl::requests.filter(requests::dsl::identifier.eq(&identifier)))
            .execute(conn)
    }
}
