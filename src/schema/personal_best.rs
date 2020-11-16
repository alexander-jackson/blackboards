//! Allows modifications of the `personal_bests` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::forms;
use crate::guards::AuthorisedUser;

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
#[derive(Debug, Default, Insertable, Queryable, Serialize)]
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

    /// Finds a user's personal bests in the database given their Warwick ID.
    pub fn find(user: AuthorisedUser, conn: &diesel::SqliteConnection) -> QueryResult<Self> {
        if let Ok(pbs) = personal_bests::dsl::personal_bests
            .find(user.id)
            .first::<Self>(conn)
        {
            return Ok(pbs);
        }

        // Insert a blank one and return that instead
        let pbs = PersonalBest::from(user);
        pbs.insert(conn)?;

        Ok(pbs)
    }

    /// Updates a user's personal bests based on their form submission.
    pub fn update(
        user: AuthorisedUser,
        data: forms::PersonalBests,
        conn: &diesel::SqliteConnection,
    ) -> QueryResult<usize> {
        let filter = personal_bests::dsl::warwick_id.eq(user.id);
        let current = Self::find(user, conn)?;

        // Check which columns need updating
        let updates = (
            personal_bests::dsl::squat.eq(data.squat.or(current.squat)),
            personal_bests::dsl::bench.eq(data.bench.or(current.bench)),
            personal_bests::dsl::deadlift.eq(data.deadlift.or(current.deadlift)),
            personal_bests::dsl::snatch.eq(data.snatch.or(current.snatch)),
            personal_bests::dsl::clean_and_jerk.eq(data.clean_and_jerk.or(current.clean_and_jerk)),
        );

        diesel::update(personal_bests::dsl::personal_bests.filter(filter))
            .set(updates)
            .execute(conn)
    }
}

impl From<AuthorisedUser> for PersonalBest {
    fn from(user: AuthorisedUser) -> Self {
        Self {
            warwick_id: user.id,
            name: user.name,
            ..Default::default()
        }
    }
}
