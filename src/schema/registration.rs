use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::email;
use crate::forms;
use crate::schema::{custom_types, sessions, Request, Session};

table! {
    registrations (warwick_id) {
        session_id -> Integer,
        warwick_id -> Integer,
        name -> Text,
    }
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Registration {
    pub session_id: i32,
    pub warwick_id: i32,
    pub name: String,
}

impl Registration {
    pub fn create(data: Request) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id,
            name: data.name,
        }
    }

    pub fn create_from_verified(data: forms::Register) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id.0,
            name: data.name,
        }
    }

    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Ensure the session has spaces
        let session = Session::find(self.session_id, conn)?;

        if session.remaining == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        diesel::insert_into(registrations::table)
            .values(self)
            .execute(conn)?;

        email::send_confirmation(&self, &session);

        Session::decrement_remaining(self.session_id, conn)
    }

    pub fn count(session_id: i32, conn: &diesel::SqliteConnection) -> QueryResult<i64> {
        registrations::dsl::registrations
            .filter(registrations::dsl::session_id.eq(&session_id))
            .count()
            .get_result(conn)
    }

    pub fn get_registration_list(
        conn: &diesel::SqliteConnection,
    ) -> QueryResult<Vec<(i32, custom_types::DateTime, String, String)>> {
        let columns = (
            sessions::dsl::id,
            sessions::dsl::start_time,
            sessions::dsl::title,
            registrations::dsl::name,
        );
        let ordering = (sessions::dsl::start_time, sessions::dsl::id);

        registrations::dsl::registrations
            .inner_join(sessions::dsl::sessions)
            .select(columns)
            .order_by(ordering)
            .load(conn)
    }
}
