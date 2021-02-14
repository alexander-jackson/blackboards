//! Allows modifications of the `taskmaster_entry` table in the database.

use std::str::FromStr;

use diesel::{QueryResult, RunQueryDsl};

table! {
    /// Represents the schema for `taskmaster_entries`.
    taskmaster_entries (name) {
        /// The user's name.
        name -> Text,
        /// The number of points that the user has
        score -> Integer,
    }
}

/// Represents a row in the `personal_bests` table.
#[derive(Debug, Default, Insertable, Queryable, Serialize)]
#[table_name = "taskmaster_entries"]
pub struct TaskmasterEntry {
    /// The user's name
    pub name: String,
    /// The number of points that the user has
    pub score: i32,
}

impl TaskmasterEntry {
    /// Inserts the [`TaskmasterEntry`] into the database.
    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(taskmaster_entries::table)
            .values(self)
            .execute(conn)
    }

    /// Gets all personal bests currently in the database.
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        taskmaster_entries::dsl::taskmaster_entries.get_results::<Self>(conn)
    }

    /// Updates the entire leaderboard based on a CSV structure.
    pub fn update_all(leaderboard: &str, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Parse the new contents
        let new_state: Vec<_> = leaderboard
            .trim()
            .lines()
            .map(|line| {
                let mut iter = line.split(',');
                TaskmasterEntry {
                    name: String::from(iter.next().unwrap()),
                    score: i32::from_str(iter.next().unwrap()).unwrap(),
                }
            })
            .collect();

        // Delete the contents first
        diesel::delete(taskmaster_entries::dsl::taskmaster_entries).execute(conn)?;

        // Update with the new contents
        diesel::insert_into(taskmaster_entries::dsl::taskmaster_entries)
            .values(new_state)
            .execute(conn)
    }
}
