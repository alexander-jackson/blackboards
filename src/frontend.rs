use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::context;
use crate::schema;

use crate::guards::DatabaseConnection;

#[get("/sessions")]
pub fn dashboard(conn: DatabaseConnection, flash: Option<FlashMessage>) -> Template {
    let sessions = schema::Session::get_results(&conn.0).unwrap();

    let message = flash.map(|f| f.msg().to_string());

    Template::render(
        "sessions",
        context::Context {
            sessions,
            current: None,
            message,
        },
    )
}

#[get("/sessions/<session_id>")]
pub fn specific_session(conn: DatabaseConnection, session_id: i32) -> Result<Template, Redirect> {
    let sessions = schema::Session::get_results(&conn.0).unwrap();
    let current = schema::Session::find(session_id, &conn.0).ok();

    Ok(Template::render(
        "sessions",
        context::Context {
            sessions,
            current,
            message: None,
        },
    ))
}
