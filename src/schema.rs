use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::forms;

table! {
    sessions {
        id -> Integer,
        title -> Text,
        start_time -> Text,
        remaining -> Integer,
    }
}

table! {
    registrations (email) {
        session_id -> Integer,
        email -> Text,
        name -> Text,
    }
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Session {
    pub id: i32,
    pub title: String,
    pub start_time: String,
    pub remaining: i32,
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Registration {
    pub session_id: i32,
    pub email: String,
    pub name: String,
}

impl Session {
    pub fn get_results(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        sessions::dsl::sessions.get_results::<Self>(conn)
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

impl Registration {
    pub fn create(data: forms::Register) -> Self {
        Self {
            session_id: data.session_id,
            email: data.email.0,
            name: data.name,
        }
    }

    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Ensure the session has spaces
        let remaining = Session::find(self.session_id, conn)?.remaining;

        if remaining == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        diesel::insert_into(registrations::table)
            .values(self)
            .execute(conn)?;

        Session::decrement_remaining(self.session_id, conn)
    }

    pub fn count(session_id: i32, conn: &diesel::SqliteConnection) -> QueryResult<i64> {
        registrations::dsl::registrations
            .filter(registrations::dsl::session_id.eq(&session_id))
            .count()
            .get_result(conn)
    }
}
