//! Stores the custom datatypes required for the schema to work.

use std::fmt;
use std::io::Write;

use diesel::backend::Backend;
use diesel::serialize::Output;
use diesel::sql_types::BigInt;
use diesel::types::{FromSql, ToSql};

/// Represents a custom datetime, to be stored as BigInt in SQL and formatted otherwise.
#[derive(Debug, AsExpression, Eq, PartialEq, Ord, PartialOrd)]
pub struct DateTime {
    value: i64,
}

impl DateTime {
    /// Creates a new [`DateTime`] from a timestamp.
    pub fn new(timestamp: i64) -> Self {
        Self { value: timestamp }
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.value, 0);
        let formatted = datetime.format("%a %d %h, %H:%M").to_string();
        serializer.serialize_str(&formatted)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Mon 08 Oct, 12:15
        let datetime = chrono::NaiveDateTime::from_timestamp(self.value, 0);
        write!(f, "{}", datetime.format("%a %d %h, %H:%M"))
    }
}

impl<DB> ToSql<BigInt, DB> for DateTime
where
    DB: Backend,
    i64: ToSql<BigInt, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> diesel::serialize::Result {
        self.value.to_sql(out)
    }
}

impl<DB> FromSql<BigInt, DB> for DateTime
where
    DB: Backend,
    i64: FromSql<BigInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        Ok(Self::new(i64::from_sql(bytes)?))
    }
}

impl diesel::expression::Expression for DateTime {
    type SqlType = diesel::sql_types::BigInt;
}

impl<DB, ST> diesel::Queryable<ST, DB> for DateTime
where
    DB: Backend,
    i64: diesel::Queryable<ST, DB>,
{
    type Row = <i64 as diesel::Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        DateTime::new(i64::build(row))
    }
}
