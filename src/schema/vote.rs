//! Allows modifications of the `votes` table in the database.

use std::collections::HashMap;

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `votes`.
    votes (warwick_id, position_id, candidate_id) {
        /// The Warwick identifier of the user voting.
        warwick_id -> Integer,
        /// The position identifier they voted for.
        position_id -> Integer,
        /// The identifier of the candidate they voted for.
        candidate_id -> Integer,
        /// The ranking they gave it.
        ranking -> Integer,
    }
}

/// Represents a row in the `votes` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Vote {
    /// The Warwick identifier of the user voting.
    pub warwick_id: i32,
    /// The position identifier they voted for.
    pub position_id: i32,
    /// The identifier of the candidate they voted for.
    pub candidate_id: i32,
    /// The ranking they gave them.
    pub ranking: i32,
}

impl Vote {
    /// Inserts the [`Vote`] into the database.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(votes::table).values(self).execute(conn)
    }

    /// Inserts a group of [`Vote`]s into the database.
    pub fn insert_all(
        user_id: i32,
        position_id: i32,
        votes: &HashMap<i32, i32>,
        conn: &diesel::SqliteConnection,
    ) -> QueryResult<usize> {
        // Delete all previous votes to avoid clashes
        diesel::delete(
            votes::dsl::votes.filter(
                votes::dsl::warwick_id
                    .eq(user_id)
                    .and(votes::dsl::position_id.eq(position_id)),
            ),
        )
        .execute(conn)?;

        let votes: Vec<Self> = votes
            .iter()
            .map(|(ranking, candidate_id)| Vote {
                warwick_id: user_id,
                position_id,
                candidate_id: *candidate_id,
                ranking: *ranking,
            })
            .collect();

        diesel::insert_into(votes::table)
            .values(votes)
            .execute(conn)
    }

    /// Gets all [`Vote`] entries in the database.
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        votes::dsl::votes.get_results::<Self>(conn)
    }
}
