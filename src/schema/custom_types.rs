//! Stores the custom datatypes required for the schema to work.

use std::fmt;

use chrono::TimeZone;

/// Represents a custom datetime, to be stored as BigInt in SQL and formatted otherwise.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, DieselNewType)]
pub struct DateTime(i64);

impl DateTime {
    /// Creates a new [`DateTime`] from a timestamp.
    pub fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let formatted = self.to_string();
        serializer.serialize_str(&formatted)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Mon 08 Oct, 12:15
        let datetime = chrono::Local.timestamp(self.0, 0);
        write!(f, "{}", datetime.format("%a %d %h, %H:%M"))
    }
}
