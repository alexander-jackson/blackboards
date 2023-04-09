//! Allows modifications of the `attendances` table in the database.

use serde::Serialize;

use crate::forms;
use crate::schema::Pool;

/// Represents a row in the `attendances` table.
#[derive(Copy, Clone, Debug, Serialize)]
pub struct Attendance {
    /// The identifier for the session.
    pub session_id: i32,
    /// The user's Warwick ID.
    pub warwick_id: i32,
}

impl Attendance {
    /// Inserts the data into the appropriate table.
    pub async fn insert(&self, pool: &mut Pool) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO attendances (session_id, warwick_id) VALUES ($1, $2)",
            self.session_id,
            self.warwick_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl From<forms::Attendance> for Attendance {
    /// Creates a new [`Attendance`] struct from the form data.
    fn from(data: forms::Attendance) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id.0,
        }
    }
}
