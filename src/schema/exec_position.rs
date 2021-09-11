//! Allows modifications of the `exec_positions` table in the database.

use crate::schema::Pool;

/// Represents a row in the `exec_positions` table.
#[derive(Clone, Debug, Serialize)]
pub struct ExecPosition {
    /// The identifier for the position
    pub id: i32,
    /// The title of the position
    pub title: String,
    /// The number of people who can win in this position
    pub num_winners: i32,
    /// Whether voting is open for this position or not
    pub open: bool,
}

impl ExecPosition {
    /// Inserts the [`ExecPosition`] into the database.
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO exec_positions (id, title, num_winners, open) VALUES ($1, $2, $3, $4)",
            self.id,
            self.title,
            self.num_winners,
            self.open
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Gets all [`ExecPosition`] entries in the database.
    pub async fn get_results(pool: &mut Pool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM exec_positions")
            .fetch_all(pool)
            .await
    }

    /// Gets the title of a given position.
    pub async fn get_title(position_id: i32, pool: &mut Pool) -> sqlx::Result<String> {
        sqlx::query!(
            "SELECT title FROM exec_positions WHERE id = $1",
            position_id
        )
        .map(|row| row.title)
        .fetch_one(pool)
        .await
    }

    /// Gets the identifiers of all closed positions.
    pub async fn closed_identifiers(pool: &mut Pool) -> sqlx::Result<Vec<i32>> {
        sqlx::query!("SELECT id FROM exec_positions WHERE open IS NOT TRUE")
            .map(|row| row.id)
            .fetch_all(pool)
            .await
    }

    /// Checks whether voting is open for a given identifier.
    pub async fn voting_is_open(position_id: i32, pool: &mut Pool) -> bool {
        sqlx::query_as!(
            Self,
            "SELECT * FROM exec_positions WHERE id = $1",
            position_id
        )
        .fetch_one(pool)
        .await
        .ok()
        .map(|row| row.open)
        .unwrap_or_default()
    }

    /// Toggles the state of the position, either opening or closing voting.
    pub async fn toggle_state(position_id: i32, pool: &mut Pool) -> sqlx::Result<()> {
        log::info!("Toggling the state of position_id={}", position_id);

        sqlx::query!(
            "UPDATE exec_positions SET open = NOT open WHERE id = $1",
            position_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
