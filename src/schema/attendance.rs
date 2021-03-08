//! Allows modifications of the `attendances` table in the database.

use diesel::{QueryResult, RunQueryDsl};

use crate::forms;

table! {
    /// Represents the schema for `attendances`.
    attendances (session_id, warwick_id) {
        /// The identifier for the session.
        session_id -> Integer,
        /// The user's Warwick ID.
        warwick_id -> Integer,
    }
}

/// Represents a row in the `attendances` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Attendance {
    /// The identifier for the session.
    pub session_id: i32,
    /// The user's Warwick ID.
    pub warwick_id: i32,
}

impl Attendance {
    /// Inserts the data into the appropriate table.
    pub fn insert(&self, conn: &diesel::PgConnection) -> QueryResult<usize> {
        diesel::insert_into(attendances::table)
            .values(self)
            .execute(conn)
    }
}

impl From<forms::Attendance> for Attendance {
    /// Creates a new [`Attendance`] struct from the form data.
    fn from(data: forms::Attendance) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id.0,
        }
    }
}
