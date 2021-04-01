//! Allows modifications of the `nominations` table in the database.

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, QueryResult, RunQueryDsl};

use crate::schema::candidate::candidates;

table! {
    /// Represents the schema for `nominations`.
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
    pub fn insert(&self, conn: &diesel::PgConnection) -> QueryResult<usize> {
        diesel::insert_into(nominations::table)
            .values(self)
            .execute(conn)
    }

    /// Gets all [`Nomination`] entries in the database.
    pub fn get_results(conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
        nominations::dsl::nominations.get_results::<Self>(conn)
    }

    /// Gets all the [`Nomination`] entries for a position identifier.
    pub fn for_position_with_names(
        position_id: i32,
        conn: &diesel::PgConnection,
    ) -> QueryResult<Vec<(i32, String)>> {
        nominations::dsl::nominations
            .filter(nominations::dsl::position_id.eq(position_id))
            .inner_join(
                candidates::dsl::candidates
                    .on(candidates::dsl::warwick_id.eq(nominations::dsl::warwick_id)),
            )
            .filter(candidates::dsl::elected.eq(false))
            .select((nominations::dsl::warwick_id, candidates::dsl::name))
            .get_results::<(i32, String)>(conn)
    }
}
