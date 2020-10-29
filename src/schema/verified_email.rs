//! Allows modifications of the `verified_emails` table in the database.

use diesel::{QueryDsl, QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `verified_emails`.
    verified_emails (warwick_id) {
        /// The user's Warwick ID.
        warwick_id -> Integer,
        /// The user's name.
        name -> Text,
    }
}

/// Represents a row in the `verified_emails` table.
#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct VerifiedEmail {
    /// The user's Warwick ID
    pub warwick_id: i32,
    /// The user's name
    pub name: String,
}

impl VerifiedEmail {
    /// Creates a new [`VerifiedEmail`] given a Warwick ID and a name.
    pub fn create(warwick_id: i32, name: String) -> Self {
        Self { warwick_id, name }
    }

    /// Inserts the row into the database.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(verified_emails::table)
            .values(self)
            .execute(conn)
    }

    /// Finds a [`VerifiedEmail`] given a Warwick ID.
    pub fn find(warwick_id: i32, conn: &diesel::SqliteConnection) -> QueryResult<Self> {
        verified_emails::dsl::verified_emails
            .find(warwick_id)
            .first::<Self>(conn)
    }

    /// Checks if a row with a given Warwick ID exists in the database.
    pub fn exists(warwick_id: i32, conn: &diesel::SqliteConnection) -> bool {
        Self::find(warwick_id, conn).is_ok()
    }
}
