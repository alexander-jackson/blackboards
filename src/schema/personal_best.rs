//! Allows modifications of the `personal_bests` table in the database.

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

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
        let filter = personal_bests::dsl::show_pl.eq(true).and(
            personal_bests::dsl::squat
                .is_not_null()
                .or(personal_bests::dsl::bench
                    .is_not_null()
                    .or(personal_bests::dsl::deadlift.is_not_null())),
        );
        let order = personal_bests::dsl::warwick_id.asc();

        personal_bests::dsl::personal_bests
            .filter(filter)
            .order_by(order)
            .get_results::<Self>(conn)
    }

    /// Gets all personal bests currently in the database.
    pub fn get_wl(conn: &diesel::PgConnection) -> QueryResult<Vec<Self>> {
        let filter = personal_bests::dsl::show_wl.eq(true).and(
            personal_bests::dsl::snatch
                .is_not_null()
                .or(personal_bests::dsl::clean_and_jerk.is_not_null()),
        );
        let order = personal_bests::dsl::warwick_id.asc();

        personal_bests::dsl::personal_bests
            .filter(filter)
            .order_by(order)
            .get_results::<Self>(conn)
    }

    /// Finds a user's personal bests in the database given their Warwick ID.
    pub fn find(user_id: i32, name: &str, conn: &diesel::PgConnection) -> QueryResult<Self> {
        if let Ok(pbs) = personal_bests::dsl::personal_bests
            .find(user_id)
            .first::<Self>(conn)
        {
            return Ok(pbs);
        }

        log::info!(
            "User ({}, {}) has no personal bests, inserting defaults",
            user_id,
            name
        );

        // Insert a blank one and return that instead
        let pbs = Self::new(user_id, String::from(name));
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
        let current = Self::find(user_id, &name, conn)?;

        log::info!(
            "Updating personal bests for ({}, {}) to: {:?}",
            user_id,
            name,
            data
        );

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

    /// Checks whether the personal bests warrant a warning message.
    pub fn check_for_show_without_values(&self) -> Option<String> {
        if self.show_pl
            && !(self.squat.is_some() || self.bench.is_some() || self.deadlift.is_some())
        {
            return Some(String::from("You have checked to be shown for powerlifting but have no personal bests, so you have been hidden from this board"));
        }

        if self.show_wl && !(self.snatch.is_some() || self.clean_and_jerk.is_some()) {
            return Some(String::from("You have checked to be shown for weightlifting but have no personal bests, so you have been hidden from this board"));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_pl_with_no_pl_lifts_shows_a_warning() {
        let mut personal_bests = PersonalBest::new(1702502, String::from("Alex Jackson"));
        personal_bests.show_pl = true;

        assert!(personal_bests.check_for_show_without_values().is_some());
    }

    #[test]
    fn show_wl_with_no_wl_lifts_shows_a_warning() {
        let mut personal_bests = PersonalBest::new(1702502, String::from("Alex Jackson"));
        personal_bests.show_wl = true;

        assert!(personal_bests.check_for_show_without_values().is_some());
    }

    #[test]
    fn show_pl_with_some_pl_lifts_shows_no_warning() {
        let mut personal_bests = PersonalBest::new(1702502, String::from("Alex Jackson"));
        personal_bests.show_pl = true;
        personal_bests.squat = Some(100.0);

        assert!(personal_bests.check_for_show_without_values().is_none());
    }

    #[test]
    fn show_wl_with_some_wl_lifts_shows_no_warning() {
        let mut personal_bests = PersonalBest::new(1702502, String::from("Alex Jackson"));
        personal_bests.show_wl = true;
        personal_bests.snatch = Some(50.0);

        assert!(personal_bests.check_for_show_without_values().is_none());
    }
}
