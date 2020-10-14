use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rand::Rng;

use crate::email;
use crate::forms;

pub mod custom_types;

table! {
    sessions {
        id -> Integer,
        title -> Text,
        start_time -> BigInt,
        remaining -> Integer,
    }
}

table! {
    verified_emails (warwick_id) {
        warwick_id -> Integer,
        name -> Text,
    }
}

table! {
    requests (warwick_id) {
        session_id -> Integer,
        warwick_id -> Integer,
        name -> Text,
        identifier -> Integer,
    }
}

table! {
    registrations (warwick_id) {
        session_id -> Integer,
        warwick_id -> Integer,
        name -> Text,
    }
}

table! {
    attendances (session_id, warwick_id) {
        session_id -> Integer,
        warwick_id -> Integer,
    }
}

joinable!(registrations -> sessions (session_id));
allow_tables_to_appear_in_same_query!(registrations, sessions);

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Session {
    pub id: i32,
    pub title: String,
    pub start_time: custom_types::DateTime,
    pub remaining: i32,
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct VerifiedEmail {
    pub warwick_id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Request {
    pub session_id: i32,
    pub warwick_id: i32,
    pub name: String,
    pub identifier: i32,
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Registration {
    pub session_id: i32,
    pub warwick_id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Attendance {
    pub session_id: i32,
    pub warwick_id: i32,
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

impl VerifiedEmail {
    pub fn create(warwick_id: i32, name: String) -> Self {
        Self { warwick_id, name }
    }

    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(verified_emails::table)
            .values(self)
            .execute(conn)
    }

    pub fn find(warwick_id: i32, conn: &diesel::SqliteConnection) -> QueryResult<Self> {
        verified_emails::dsl::verified_emails
            .find(warwick_id)
            .first::<Self>(conn)
    }

    pub fn exists(warwick_id: i32, conn: &diesel::SqliteConnection) -> bool {
        Self::find(warwick_id, conn).is_ok()
    }
}

impl Request {
    pub fn create(data: forms::Register) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id.0,
            name: data.name,
            identifier: rand::thread_rng().gen::<i32>().abs(),
        }
    }

    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Ensure the session has spaces
        let session = Session::find(self.session_id, conn)?;

        if session.remaining == 0 {
            return Err(diesel::result::Error::NotFound);
        }

        // Email the user
        email::confirm_email_address(&self, &session);

        diesel::insert_into(requests::table)
            .values(self)
            .execute(conn)
    }

    pub fn verify(identifier: i32, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        // Find the request
        let request: Self = requests::dsl::requests
            .filter(requests::dsl::identifier.eq(&identifier))
            .first(conn)?;

        let registration = Registration::create(request);
        registration.insert(conn)?;

        // Add the user to the confirmed emails
        let verification =
            VerifiedEmail::create(registration.warwick_id, registration.name.clone());
        verification.insert(conn)?;

        Request::delete(identifier, conn)
    }

    pub fn delete(identifier: i32, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::delete(requests::dsl::requests.filter(requests::dsl::identifier.eq(&identifier)))
            .execute(conn)
    }
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

        email::send_confirmation_email(&self, &session);

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

impl Attendance {
    pub fn create(data: forms::Attendance) -> Self {
        Self {
            session_id: data.session_id,
            warwick_id: data.warwick_id.0,
        }
    }

    pub fn insert(&self, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(attendances::table)
            .values(self)
            .execute(conn)
    }
}
