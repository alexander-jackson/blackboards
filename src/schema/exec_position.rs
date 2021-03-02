//! Allows modifications of the `exec_positions` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `exec_positions`.
    exec_positions {
        /// The identifier for the position.
        id -> Integer,
        /// The name of the position.
        title -> Text,
        /// Whether voting is open for this position or not.
        open -> Bool,
    }
}

/// Represents a row in the `exec_positions` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct ExecPosition {
    /// The identifier for the position
    pub id: i32,
    /// The title of the position
    pub title: String,
    /// Whether voting is open for this position or not
    pub open: bool,
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

    /// Checks whether voting is open for a given identifier.
    pub fn voting_is_open(position_id: i32, conn: &diesel::SqliteConnection) -> bool {
        exec_positions::dsl::exec_positions
            .filter(exec_positions::dsl::id.eq(position_id))
            .first::<Self>(conn)
            .map(|row| row.open)
            .unwrap_or_default()
    }
}
