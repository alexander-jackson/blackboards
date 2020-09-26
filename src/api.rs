use rocket::request::Form;
use rocket::response::Redirect;

use crate::forms;
use crate::frontend;
use crate::schema;

use crate::guards::DatabaseConnection;

#[post("/session/register", data = "<data>")]
pub fn register(conn: DatabaseConnection, data: Form<forms::Register>) -> Redirect {
    let data = data.into_inner();

    let request = schema::Request::create(data);
    request.insert(&conn.0).unwrap();

    Redirect::to(uri!(frontend::dashboard))
}

#[get("/session/confirm/<id>")]
pub fn confirm_email(conn: DatabaseConnection, id: i32) -> Redirect {
    schema::Request::verify(id, &conn.0).unwrap();

    Redirect::to(uri!(frontend::dashboard))
}
