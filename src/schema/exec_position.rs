//! Allows modifications of the `exec_positions` table in the database.

use diesel::{QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `registrations`.
    exec_positions {
        /// The identifier for the position.
        id -> Integer,
        /// The name of the position.
        title -> Text,
    }
}

/// Represents a row in the `exec_positions` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct ExecPosition {
    /// The identifier for the position
    pub id: i32,
    /// The title of the position
    pub title: String,
}

impl ExecPosition {
    /// Inserts the [`ExecPosition`] into the database.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(exec_positions::table)
            .values(self)
            .execute(conn)
    }

    /// Gets all [`ExecPosition`] entries in the database.
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        exec_positions::dsl::exec_positions.get_results::<Self>(conn)
    }
}
