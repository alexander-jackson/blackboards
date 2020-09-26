use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};

#[database("sqlite_database")]
pub struct DatabaseConnection(diesel::SqliteConnection);

#[derive(Copy, Clone, Debug, Serialize)]
pub struct AuthorisedUser {
    pub id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthorisedUser {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthorisedUser, !> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse().ok())
            .map(|id| AuthorisedUser { id })
            .or_forward(())
    }
}
