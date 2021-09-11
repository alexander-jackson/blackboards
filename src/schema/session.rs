//! Allows modifications of the `sessions` table in the database.

use rand::Rng;

use crate::schema::{custom_types, Pool};
use crate::session_window::SessionWindow;

/// Represents a session in the database.
#[derive(Clone, Debug, Serialize)]
pub struct Session {
    /// The identifier for the session.
    pub id: i32,
    /// The title for the session.
    pub title: String,
    /// The starting time for the session.
    pub start_time: custom_types::DateTime,
    /// The number of remaining places.
    pub remaining: i32,
}

impl Session {
    /// Creates a new session with a unique database identifier.
    pub async fn new(title: String, start_time: i64, remaining: u32, pool: &mut Pool) -> Self {
        // Generate a new identifier for the session
        let id = loop {
            let potential = rand::thread_rng().gen::<i32>();

            let exists = sqlx::query!("SELECT * FROM sessions WHERE id = $1", potential)
                .fetch_optional(&mut *pool)
                .await
                .unwrap()
                .is_some();

            if !exists {
                break potential;
            }
        };

        Self {
            id,
            title,
            start_time: custom_types::DateTime::new(start_time),
            remaining: remaining as i32,
        }
    }

    /// Inserts the [`Session`] into the database.
    pub async fn insert(&self, pool: &mut Pool) {
        sqlx::query!(
            "INSERT INTO sessions (id, title, start_time, remaining) VALUES ($1, $2, $3, $4)",
            self.id,
            self.title,
            self.start_time.inner(),
            self.remaining
        )
        .execute(pool)
        .await
        .unwrap();
    }

    /// Gets all available sessions currently in the database.
    pub async fn get_results(pool: &mut Pool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.remaining
            FROM sessions
            ORDER BY start_time"#
        )
        .fetch_all(pool)
        .await
    }

    /// Gets all available sessions currently in the database.
    pub async fn get_results_between(
        pool: &mut Pool,
        window: SessionWindow,
    ) -> sqlx::Result<Vec<Self>> {
        log::debug!("Getting all the sessions in window={:?}", window);

        sqlx::query_as!(
            Self,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.remaining
            FROM sessions
            WHERE $1 < start_time AND start_time < $2
            ORDER BY start_time"#,
            window.start,
            window.end,
        )
        .fetch_all(pool)
        .await
    }

    /// Gets all available sessions in the window or after.
    pub async fn get_results_within_and_after(
        pool: &mut Pool,
        window: SessionWindow,
    ) -> sqlx::Result<Vec<Self>> {
        log::debug!("Getting all the sessions after time={}", window.start);

        sqlx::query_as!(
            Self,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.remaining
            FROM sessions
            WHERE $1 < start_time
            ORDER BY start_time"#,
            window.start,
        )
        .fetch_all(pool)
        .await
    }

    /// Finds a session in the database given its identifier.
    pub async fn find(id: i32, pool: &mut Pool) -> sqlx::Result<Option<Self>> {
        log::debug!("Querying the database for session_id={}", id);

        sqlx::query_as!(
            Self,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.remaining
            FROM sessions
            WHERE id = $1"#,
            id,
        )
        .fetch_optional(pool)
        .await
    }

    /// Deletes the session with the given identifier.
    pub async fn delete(id: i32, pool: &mut Pool) -> sqlx::Result<()> {
        log::warn!("Deleting session with id={}, including registrations", id);

        sqlx::query!("DELETE FROM sessions WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Decreases the number of remaining places for a session given its identifier.
    pub async fn decrement_remaining(id: i32, pool: &mut Pool) -> sqlx::Result<()> {
        log::debug!("Decrementing remaining places for session_id={}", id);

        sqlx::query!(
            "UPDATE sessions SET remaining = remaining - 1 WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Increases the number of remaining places for a session given its identifier.
    pub async fn increment_remaining(id: i32, pool: &mut Pool) -> sqlx::Result<()> {
        log::debug!("Incrementing remaining places for session_id={}", id);

        sqlx::query!(
            "UPDATE sessions SET remaining = remaining + 1 WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
