use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rand::Rng;

use crate::email;
use crate::forms;
use crate::schema::{Registration, Session, VerifiedEmail};

table! {
    requests (warwick_id) {
        session_id -> Integer,
        warwick_id -> Integer,
        name -> Text,
        identifier -> Integer,
    }
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Request {
    pub session_id: i32,
    pub warwick_id: i32,
    pub name: String,
    pub identifier: i32,
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
        email::confirm_address(&self, &session);

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
        let verification = VerifiedEmail::create(registration.warwick_id, registration.name);
        verification.insert(conn)?;

        Request::delete(identifier, conn)
    }

    pub fn delete(identifier: i32, conn: &diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::delete(requests::dsl::requests.filter(requests::dsl::identifier.eq(&identifier)))
            .execute(conn)
    }
}
