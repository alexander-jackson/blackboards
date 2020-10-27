use diesel::{QueryResult, RunQueryDsl};

use crate::forms;

table! {
    attendances (session_id, warwick_id) {
        session_id -> Integer,
        warwick_id -> Integer,
    }
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct Attendance {
    pub session_id: i32,
    pub warwick_id: i32,
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
