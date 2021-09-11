//! Allows modifications of the `personal_bests` table in the database.

use crate::forms;
use crate::schema::Pool;

/// Represents a row in the `personal_bests` table.
#[derive(Clone, Debug, Default, Serialize)]
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
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        sqlx::query!("INSERT INTO personal_bests (warwick_id, name, squat, bench, deadlift, snatch, clean_and_jerk, show_pl, show_wl) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", self.warwick_id, self.name, self.squat, self.bench, self.deadlift, self.snatch, self.clean_and_jerk, self.show_pl, self.show_wl).execute(pool).await?;

        Ok(())
    }

    /// Gets all personal bests currently in the database.
    pub async fn get_results(pool: &mut Pool) -> sqlx::Result<(Vec<Self>, Vec<Self>)> {
        let pl = Self::get_pl(pool).await?;
        let wl = Self::get_wl(pool).await?;

        Ok((pl, wl))
    }

    /// Gets all personal bests currently in the database.
    pub async fn get_pl(pool: &mut Pool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM personal_bests WHERE show_pl AND (squat IS NOT NULL OR bench IS NOT NULL OR deadlift IS NOT NULL) ORDER BY warwick_id").fetch_all(pool).await
    }

    /// Gets all personal bests currently in the database.
    pub async fn get_wl(pool: &mut Pool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM personal_bests WHERE show_wl AND (snatch IS NOT NULL OR clean_and_jerk IS NOT NULL) ORDER BY warwick_id").fetch_all(pool).await
    }

    /// Finds a user's personal bests in the database given their Warwick ID.
    pub async fn find(warwick_id: i32, name: &str, pool: &mut Pool) -> sqlx::Result<Self> {
        // See if we can find some personal bests first
        let potential = sqlx::query_as!(
            Self,
            "SELECT * FROM personal_bests WHERE warwick_id = $1",
            warwick_id
        )
        .fetch_optional(&mut *pool)
        .await?;

        if let Some(pbs) = potential {
            return Ok(pbs);
        }

        log::info!(
            "User ({}, {}) has no personal bests, inserting defaults",
            warwick_id,
            name
        );

        // Insert a blank one and return that instead
        let pbs = Self::new(warwick_id, String::from(name));
        pbs.insert(&mut *pool).await?;

        Ok(pbs)
    }

    /// Updates a user's personal bests based on their form submission.
    pub async fn update(
        user_id: i32,
        name: String,
        data: forms::PersonalBests,
        pool: &mut Pool,
    ) -> sqlx::Result<()> {
        log::info!(
            "Updating personal bests for ({}, {}) to: {:?}",
            user_id,
            name,
            data
        );

        sqlx::query!("UPDATE personal_bests SET squat = COALESCE(squat, $1), bench = COALESCE(bench, $2), deadlift = COALESCE(deadlift, $3), snatch = COALESCE(snatch, $4), clean_and_jerk = COALESCE(clean_and_jerk, $5), show_pl = $6, show_wl = $7", data.squat, data.bench, data.deadlift, data.snatch, data.clean_and_jerk, data.show_pl, data.show_wl).execute(pool).await?;

        Ok(())
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
