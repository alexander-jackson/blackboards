//! Allows modifications of the `sessions` table in the database.

use rand::Rng;

use crate::context;
use crate::schema::{custom_types, Pool};
use crate::session_window::SessionWindow;

/// Represents a session in the database.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Session {
    /// The identifier for the session.
    pub id: i32,
    /// The title for the session.
    pub title: String,
    /// The starting time for the session.
    pub start_time: custom_types::DateTime,
    /// The original number of spaces in the session.
    pub spaces: i32,
}

impl Session {
    /// Creates a new session with a unique database identifier.
    pub async fn new(title: String, start_time: i64, spaces: u32, pool: &mut Pool) -> Self {
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
            spaces: spaces as i32,
        }
    }

    /// Inserts the [`Session`] into the database.
    pub async fn insert(&self, pool: &mut Pool) {
        tracing::info!(?self, "Inserting a new session into the database");

        sqlx::query!(
            "INSERT INTO sessions (id, title, start_time, spaces) VALUES ($1, $2, $3, $4)",
            self.id,
            self.title,
            self.start_time.inner(),
            self.spaces,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    /// Gets all available sessions currently in the database.
    pub async fn get_results(pool: &mut Pool) -> sqlx::Result<Vec<context::Session>> {
        sqlx::query_as!(
            context::Session,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.spaces - (
                    SELECT COUNT(*)
                    FROM registrations
                    WHERE sessions.id = registrations.session_id
                ) AS remaining_spaces
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
    ) -> sqlx::Result<Vec<context::Session>> {
        tracing::debug!(?window, "Getting all the sessions in a window");

        sqlx::query_as!(
            context::Session,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.spaces - (
                    SELECT COUNT(*)
                    FROM registrations
                    WHERE sessions.id = registrations.session_id
                ) AS remaining_spaces
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
    ) -> sqlx::Result<Vec<context::Session>> {
        tracing::debug!(time = %window.start, "Getting all the sessions after a certain time");

        sqlx::query_as!(
            context::Session,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.spaces - (
                    SELECT COUNT(*)
                    FROM registrations
                    WHERE sessions.id = registrations.session_id
                ) AS remaining_spaces
            FROM sessions
            WHERE $1 < start_time
            ORDER BY start_time"#,
            window.start,
        )
        .fetch_all(pool)
        .await
    }

    /// Finds a session in the database given its identifier.
    pub async fn find(id: i32, pool: &mut Pool) -> sqlx::Result<Option<context::Session>> {
        tracing::debug!(%id, "Querying the database for a specific session");

        sqlx::query_as!(
            context::Session,
            r#"SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.spaces - (
                    SELECT COUNT(*)
                    FROM registrations
                    WHERE sessions.id = registrations.session_id
                ) AS remaining_spaces
            FROM sessions
            WHERE id = $1"#,
            id,
        )
        .fetch_optional(pool)
        .await
    }

    /// Deletes the session with the given identifier.
    pub async fn delete(id: i32, pool: &mut Pool) -> sqlx::Result<()> {
        tracing::warn!(%id, "Deleting a specific session, including its registrations");

        sqlx::query!("DELETE FROM sessions WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Checks whether the session is full or not.
    pub async fn is_full(id: i32, pool: &mut Pool) -> sqlx::Result<bool> {
        sqlx::query!(
            r#"
            SELECT spaces - (
                SELECT COUNT(*)
                FROM registrations
                WHERE registrations.session_id = sessions.id
            ) AS remaining
            FROM sessions
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(pool)
        .await
        .map(|record| record.remaining == Some(0))
    }
}
