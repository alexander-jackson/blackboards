use diesel::{QueryDsl, QueryResult, RunQueryDsl};

table! {
    verified_emails (warwick_id) {
        warwick_id -> Integer,
        name -> Text,
    }
}

#[derive(Debug, Insertable, Queryable, Serialize)]
pub struct VerifiedEmail {
    pub warwick_id: i32,
    pub name: String,
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
