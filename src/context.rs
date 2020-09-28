use crate::schema;

#[derive(Serialize)]
pub struct Context {
    pub sessions: Vec<schema::Session>,
    pub current: Option<schema::Session>,
    pub message: Option<String>,
}
