//! Allows modifications of the `nominations` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `registrations`.
    nominations (position_id, warwick_id) {
        /// The identifier of the exec position.
        position_id -> Integer,
        /// The identifier of the candidate.
        warwick_id -> Integer,
    }
}

/// Represents a row in the `nominations` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Nomination {
    /// The identifier of the exec position.
    pub position_id: i32,
    /// The identifier of the candidate.
    pub warwick_id: i32,
}

impl Nomination {
    /// Inserts the [`Nomination`] into the database.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(nominations::table)
            .values(self)
            .execute(conn)
    }

    /// Gets all [`Nomination`] entries in the database.
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        nominations::dsl::nominations.get_results::<Self>(conn)
    }

    /// Gets all the [`Nomination`] entries for a position identifier.
    pub fn for_position(
        position_id: i32,
        conn: &diesel::SqliteConnection,
    ) -> QueryResult<Vec<Self>> {
        nominations::dsl::nominations
            .filter(nominations::dsl::position_id.eq(position_id))
            .get_results::<Self>(conn)
    }
}
