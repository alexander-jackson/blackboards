//! Allows modifications of the `personal_bests` table in the database.

use diesel::{QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `personal_bests`.
    personal_bests (warwick_id) {
        /// The user's Warwick ID.
        warwick_id -> Integer,
        /// The user's name.
        name -> Text,
        /// The user's best squat.
        squat -> Nullable<Float>,
        /// The user's best bench.
        bench -> Nullable<Float>,
        /// The user's best deadlift.
        deadlift -> Nullable<Float>,
        /// The user's best snatch.
        snatch -> Nullable<Float>,
        /// The user's best clean and jerk.
        clean_and_jerk -> Nullable<Float>,
    }
}

/// Represents a row in the `personal_bests` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct PersonalBest {
    /// The user's Warwick ID
    pub warwick_id: i32,
    /// The user's name
    pub name: String,
    /// The user's best squat.
    pub squat: Option<f32>,
    /// The user's best bench.
    pub bench: Option<f32>,
    /// The user's best deadlift.
    pub deadlift: Option<f32>,
    /// The user's best snatch.
    pub snatch: Option<f32>,
    /// The user's best clean and jerk.
    pub clean_and_jerk: Option<f32>,
}

impl PersonalBest {
    /// Inserts the [`PersonalBest`] into the database.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(personal_bests::table)
            .values(self)
            .execute(conn)
    }

    /// Gets all personal bests currently in the database.
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        personal_bests::dsl::personal_bests.get_results::<Self>(conn)
    }
}
