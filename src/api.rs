use rocket::request::Form;
use rocket::response::Redirect;

use crate::forms;
use crate::frontend;
use crate::schema;

use crate::guards::DatabaseConnection;

#[post("/session/register", data = "<data>")]
pub fn register(conn: DatabaseConnection, data: Form<forms::Register>) -> Redirect {
    let data = data.into_inner();

    let registration = schema::Registration::create(data);
    registration.insert(&conn.0);

    Redirect::to(uri!(frontend::dashboard))
}
