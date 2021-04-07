//! Allows modifications of the `personal_bests` table in the database.

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::forms;

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
        /// Whether to show the user for the PL board.
        show_pl -> Bool,
        /// Whether to show the user for the WL board.
        show_wl -> Bool,
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
    /// Whether to show the user for the PL board.
    pub show_pl: bool,
    /// Whether to show the user for the WL board.
    pub show_wl: bool,
}

impl PersonalBest {
    /// Creates a new [`PersonalBest`] instance.
    pub fn new(warwick_id: i32, name: String) -> Self {
        Self {
            warwick_id,
            name,
            ..Self::default()
        }
    }

    /// Inserts the [`PersonalBest`] into the database.
    pub fn insert(&self, conn: &diesel::PgConnection) -> QueryResult<usize> {
        diesel::insert_into(personal_bests::table)
            .values(self)
            .execute(conn)
    }

    /// Gets all personal bests currently in the database.
    pub fn get_results(conn: &diesel::PgConnection) -> QueryResult<(Vec<Self>, Vec<Self>)> {
        let pl = Self::get_pl(conn)?;
        let wl = Self::get_wl(conn)?;

        Ok((pl, wl))
    }

    /// Gets all personal bests currently in the database.
    pub fn get_pl(conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
        let filter = personal_bests::dsl::show_pl.eq(true);

        personal_bests::dsl::personal_bests
            .filter(filter)
            .get_results::<Self>(conn)
    }

    /// Gets all personal bests currently in the database.
    pub fn get_wl(conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
        let filter = personal_bests::dsl::show_wl.eq(true);

        personal_bests::dsl::personal_bests
            .filter(filter)
            .get_results::<Self>(conn)
    }

    /// Finds a user's personal bests in the database given their Warwick ID.
    pub fn find(user_id: i32, name: String, conn: &diesel::PgConnection) -> QueryResult<Self> {
        if let Ok(pbs) = personal_bests::dsl::personal_bests
            .find(user_id)
            .first::<Self>(conn)
        {
            return Ok(pbs);
        }

        // Insert a blank one and return that instead
        let pbs = Self::new(user_id, name);
        pbs.insert(conn)?;

        Ok(pbs)
    }

    /// Updates a user's personal bests based on their form submission.
    pub fn update(
        user_id: i32,
        name: String,
        data: forms::PersonalBests,
        conn: &diesel::PgConnection,
    ) -> QueryResult<usize> {
        let filter = personal_bests::dsl::warwick_id.eq(user_id);
        let current = Self::find(user_id, name, conn)?;

        // Check which columns need updating
        let updates = (
            personal_bests::dsl::squat.eq(data.squat.or(current.squat)),
            personal_bests::dsl::bench.eq(data.bench.or(current.bench)),
            personal_bests::dsl::deadlift.eq(data.deadlift.or(current.deadlift)),
            personal_bests::dsl::snatch.eq(data.snatch.or(current.snatch)),
            personal_bests::dsl::clean_and_jerk.eq(data.clean_and_jerk.or(current.clean_and_jerk)),
            personal_bests::dsl::show_pl.eq(data.show_pl),
            personal_bests::dsl::show_wl.eq(data.show_wl),
        );

        diesel::update(personal_bests::dsl::personal_bests.filter(filter))
            .set(updates)
            .execute(conn)
    }
}
