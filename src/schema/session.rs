use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::schema::custom_types;

table! {
    sessions {
        id -> Integer,
        title -> Text,
        start_time -> BigInt,
        remaining -> Integer,
    }
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Session {
    pub id: i32,
    pub title: String,
    pub start_time: custom_types::DateTime,
    pub remaining: i32,
}

impl Session {
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        sessions::dsl::sessions
            .order_by(sessions::dsl::start_time.asc())
            .get_results::<Self>(conn)
    }

    pub fn find(id: i32, conn: &diesel::SqliteConnection) -> QueryResult<Self> {
        sessions::dsl::sessions.find(id).first::<Session>(conn)
    }

    pub fn decrement_remaining(id: i32, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        let current = Self::find(id, conn)?.remaining;

        diesel::update(sessions::dsl::sessions.filter(sessions::dsl::id.eq(&id)))
            .set(sessions::dsl::remaining.eq(current - 1))
            .execute(conn)
    }
}
