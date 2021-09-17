//! Allows modifications of the `registrations` table in the database.

use crate::schema::{custom_types, Pool, Session};
use crate::session_window::SessionWindow;

/// Represents a row in the `registrations` table.
#[derive(Clone, Debug, Serialize)]
pub struct Registration {
    /// The identifier for the session
    pub session_id: i32,
    /// The user's Warwick ID
    pub warwick_id: i32,
    /// The user's name
    pub name: String,
}

/// Represents a registration for a particular session, for further processing.
#[derive(Clone, Debug, Serialize)]
pub struct SessionRegistration {
    /// The identifier for the session.
    pub session_id: i32,
    /// The starting time for the session.
    pub start_time: custom_types::DateTime,
    /// The title for the session.
    pub title: String,
    /// The name of the user who has registered.
    pub name: String,
}

impl Registration {
    /// Creates a new [`Registration`] instance.
    pub fn new(session_id: i32, warwick_id: i32, name: String) -> Self {
        Self {
            session_id,
            warwick_id,
            name,
        }
    }

    /// Inserts the [`Registration`] into the database.
    ///
    /// This fails if the session has no remaining places, and sends the user a confirmation email
    /// upon success. It also decrements the number of remaining places for the given session.
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        // Ensure the session has spaces
        match Session::find(self.session_id, &mut *pool).await? {
            None => return Err(sqlx::Error::RowNotFound),
            Some(session) if session.remaining == 0 => return Err(sqlx::Error::RowNotFound),
            _ => (),
        }

        tracing::info!(?self, "Registering a user for a session");

        // Insert the registration
        sqlx::query!(
            "INSERT INTO registrations (session_id, warwick_id, name) VALUES ($1, $2, $3)",
            self.session_id,
            self.warwick_id,
            self.name
        )
        .execute(&mut *pool)
        .await?;

        Session::decrement_remaining(self.session_id, pool).await
    }

    /// Deletes a user's registration from the database if it exists.
    pub async fn cancel(warwick_id: i32, session_id: i32, pool: &mut Pool) -> sqlx::Result<()> {
        tracing::info!(%session_id, %warwick_id, "Cancelling a registration for a session");

        Session::increment_remaining(session_id, pool).await?;

        sqlx::query!(
            "DELETE FROM registrations WHERE session_id = $1 AND warwick_id = $2",
            session_id,
            warwick_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Gets the session data and names of those registered for all sessions in the database.
    pub async fn get_registration_list(
        pool: &mut Pool,
        window: SessionWindow,
    ) -> sqlx::Result<Vec<SessionRegistration>> {
        sqlx::query_as!(
            SessionRegistration,
            r#"
            SELECT
                sessions.id AS session_id,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.title AS title,
                registrations.name AS name
            FROM registrations
            INNER JOIN sessions
            ON registrations.session_id = sessions.id
            WHERE $1 < sessions.start_time AND sessions.start_time < $2
            ORDER BY sessions.start_time, sessions.title
            "#,
            window.start,
            window.end,
        )
        .fetch_all(pool)
        .await

        // let columns = (
        //     sessions::dsl::id,
        //     sessions::dsl::start_time,
        //     sessions::dsl::title,
        //     registrations::dsl::name,
        // );
        // let ordering = (sessions::dsl::start_time, sessions::dsl::id);
        // let filter = sessions::dsl::start_time
        //     .gt(window.start)
        //     .and(sessions::dsl::start_time.lt(window.end));

        // registrations::dsl::registrations
        //     .inner_join(sessions::dsl::sessions)
        //     .select(columns)
        //     .filter(filter)
        //     .order_by(ordering)
        //     .load(conn)
    }

    /// Gets all the sessions that a user has booked.
    pub async fn get_user_bookings(
        id: i32,
        window: SessionWindow,
        pool: &mut Pool,
    ) -> sqlx::Result<Vec<Session>> {
        sqlx::query_as!(
            Session,
            r#"
            SELECT
                sessions.id,
                sessions.title,
                sessions.start_time AS "start_time: custom_types::DateTime",
                sessions.remaining
            FROM registrations
            INNER JOIN sessions ON registrations.session_id = sessions.id
            WHERE $1 < sessions.start_time AND sessions.start_time < $2 AND registrations.warwick_id = $3
            ORDER BY sessions.start_time, sessions.title
            "#,
            window.start,
            window.end,
            id,
        )
            .fetch_all(pool)
            .await

        // let columns = sessions::all_columns;
        // let ordering = (sessions::dsl::start_time, sessions::dsl::id);
        // let filter = sessions::dsl::start_time
        //     .gt(window.start)
        //     .and(sessions::dsl::start_time.lt(window.end))
        //     .and(registrations::dsl::warwick_id.eq(id));

        // registrations::dsl::registrations
        //     .inner_join(sessions::dsl::sessions)
        //     .select(columns)
        //     .filter(filter)
        //     .order_by(ordering)
        //     .load(conn)
    }
}
