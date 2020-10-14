use crate::schema;

pub type Registrations = ((String, String), Vec<String>);

#[derive(Serialize)]
pub struct Context {
    pub sessions: Vec<schema::Session>,
    pub current: Option<schema::Session>,
    pub message: Option<String>,
    pub registrations: Option<Vec<Registrations>>,
}

#[derive(Serialize)]
pub struct Attendance {
    pub sessions: Vec<schema::Session>,
    pub current: Option<schema::Session>,
}
