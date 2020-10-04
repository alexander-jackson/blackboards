#[derive(Debug, FromForm)]
pub struct Register {
    pub session_id: i32,
    pub warwick_id: i32,
    pub name: String,
}
