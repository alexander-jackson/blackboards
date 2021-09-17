//! Allows modifications of the `votes` table in the database.

use std::collections::HashMap;

use crate::schema::Pool;

/// Represents a row in the `votes` table.
#[derive(Copy, Clone, Debug, Serialize)]
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
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        tracing::debug!(?self, "Inserting an entry to the `vote` table");

        sqlx::query!("INSERT INTO votes (warwick_id, position_id, candidate_id, ranking) VALUES ($1, $2, $3, $4)", self.warwick_id, self.position_id, self.candidate_id, self.ranking).execute(pool).await?;

        Ok(())
    }

    /// Inserts a group of [`Vote`]s into the database.
    pub async fn insert_all(
        warwick_id: i32,
        position_id: i32,
        map: &HashMap<i32, i32>,
        pool: &mut Pool,
    ) -> sqlx::Result<()> {
        // Delete all previous votes to avoid clashes
        sqlx::query!(
            "DELETE FROM votes WHERE warwick_id = $1 AND position_id = $2",
            warwick_id,
            position_id
        )
        .execute(&mut *pool)
        .await?;

        tracing::info!(%warwick_id, %position_id, "Deleted all the votes in the database");

        let votes: Vec<Self> = map
            .iter()
            .map(|(ranking, candidate_id)| Vote {
                warwick_id,
                position_id,
                candidate_id: *candidate_id,
                ranking: *ranking,
            })
            .collect();

        // `sqlx` doesn't support multiple entries, so iterate instead
        for vote in votes {
            vote.insert(&mut *pool).await?;
        }

        tracing::info!(%warwick_id, %position_id, ?map, "Inserted a ballot into the database");

        Ok(())
    }

    /// Gets all [`Vote`] entries in the database.
    pub async fn get_results(pool: &mut Pool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM votes")
            .fetch_all(pool)
            .await
    }

    /// Gets a user's current ballot state, if they have voted.
    pub async fn get_current_ballot(
        user_id: i32,
        position_id: i32,
        pool: &mut Pool,
    ) -> sqlx::Result<Option<Vec<String>>> {
        tracing::debug!(%user_id, %position_id, "Fetching the current ballot");

        // Get their votes for this position
        sqlx::query!(
        r#"
            SELECT c.name AS name
            FROM votes v
            INNER JOIN nominations n ON n.warwick_id = v.candidate_id AND n.position_id = v.position_id
            INNER JOIN candidates c ON c.warwick_id = n.warwick_id
            WHERE v.warwick_id = $1 AND v.position_id = $2 ORDER BY v.ranking
        "#,
        user_id,
        position_id
        )
        .fetch_all(pool)
        .await
        .map(|v| if v.is_empty() { None } else { Some(v.into_iter().map(|e| e.name).collect()) })
    }
}

impl From<(i32, i32, i32, i32)> for Vote {
    fn from((position_id, warwick_id, candidate_id, ranking): (i32, i32, i32, i32)) -> Self {
        Self {
            warwick_id,
            position_id,
            candidate_id,
            ranking,
        }
    }
}
