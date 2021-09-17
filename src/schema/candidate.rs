//! Allows modifications of the `candidates` table in the database.

use crate::schema::Pool;

/// Represents a row in the `candidates` table.
#[derive(Debug, Serialize)]
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
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO candidates (warwick_id, name, elected) VALUES ($1, $2, $3)",
            self.warwick_id,
            self.name,
            self.elected
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Gets all [`Candidate`] entries in the database.
    pub async fn get_results(pool: &mut Pool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM candidates")
            .fetch_all(pool)
            .await
    }

    /// Mark the winning candidates as such.
    pub async fn mark_elected(candidates: &[i32], pool: &mut Pool) -> sqlx::Result<()> {
        tracing::info!(
            ?candidates,
            "Marking some candidates as elected to positions"
        );

        // Remove all the existing winners
        sqlx::query!("UPDATE candidates SET elected = FALSE")
            .execute(&mut *pool)
            .await?;

        // Set each candidate to be elected TODO: this could use `IN`
        for candidate in candidates {
            sqlx::query!(
                "UPDATE candidates SET elected = TRUE WHERE warwick_id = $1",
                candidate
            )
            .execute(&mut *pool)
            .await?;
        }

        Ok(())
    }
}
