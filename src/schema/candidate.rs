//! Allows modifications of the `candidates` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `candidates`.
    candidates (warwick_id) {
        /// The identifier of the candidate.
        warwick_id -> Integer,
        /// The name of the candidate.
        name -> Text,
        /// Whether they have been elected to the exec yet.
        elected -> Bool,
    }
}

/// Represents a row in the `candidates` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Candidate {
    /// The identifier of the candidate.
    pub warwick_id: i32,
    /// The name of the candidate.
    pub name: String,
    /// Whether they have been elected to the exec yet.
    pub elected: bool,
}

impl Candidate {
    /// Inserts the [`Candidate`] into the database.
    pub fn insert(&self, conn: &diesel::PgConnection) -> QueryResult<usize> {
        diesel::insert_into(candidates::table)
            .values(self)
            .execute(conn)
    }

    /// Gets all [`Candidate`] entries in the database.
    pub fn get_results(conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
        candidates::dsl::candidates.get_results::<Self>(conn)
    }

    /// Mark the winning candidates as such.
    pub fn mark_elected(winners: &[i32], conn: &diesel::PgConnection) -> QueryResult<usize> {
        log::trace!(
            "Marking the following identifiers as elected: {:?}",
            winners
        );

        // Remove all the existing winners
        diesel::update(candidates::dsl::candidates)
            .set(candidates::dsl::elected.eq(false))
            .execute(conn)?;

        diesel::update(
            candidates::dsl::candidates.filter(candidates::dsl::warwick_id.eq_any(winners)),
        )
        .set(candidates::dsl::elected.eq(true))
        .execute(conn)
    }
}
