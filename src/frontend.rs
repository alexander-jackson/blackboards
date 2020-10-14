use std::collections::BTreeMap;

use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::context;
use crate::schema;

use crate::guards::DatabaseConnection;

fn format_registrations(
    unformatted: Vec<(i32, schema::custom_types::DateTime, String, String)>,
) -> Vec<context::Registrations> {
    let mut map: BTreeMap<(i32, schema::custom_types::DateTime, String), Vec<String>> =
        BTreeMap::new();

    for (id, start_time, title, name) in unformatted {
        map.entry((id, start_time, title)).or_default().push(name);
    }

    let mut registrations = Vec::new();

    for (key, value) in map {
        registrations.push(((key.1.to_string(), key.2), value))
    }

    registrations
}

fn get_registrations(conn: &diesel::SqliteConnection) -> Option<Vec<context::Registrations>> {
    let unformatted = schema::Registration::get_registration_list(conn).unwrap();
    let formatted = format_registrations(unformatted);

    match formatted.len() {
        0 => None,
        _ => Some(formatted),
    }
}

#[get("/sessions")]
pub fn dashboard(conn: DatabaseConnection, flash: Option<FlashMessage>) -> Template {
    let sessions = schema::Session::get_results(&conn.0).unwrap();
    let message = flash.map(|f| f.msg().to_string());
    let registrations = get_registrations(&conn.0);

    Template::render(
        "sessions",
        context::Context {
            sessions,
            current: None,
            message,
            registrations,
        },
    )
}

#[get("/sessions/<session_id>")]
pub fn specific_session(conn: DatabaseConnection, session_id: i32) -> Result<Template, Redirect> {
    let sessions = schema::Session::get_results(&conn.0).unwrap();
    let current = schema::Session::find(session_id, &conn.0).ok();
    let registrations = get_registrations(&conn.0);

    Ok(Template::render(
        "sessions",
        context::Context {
            sessions,
            current,
            message: None,
            registrations,
        },
    ))
}

#[get("/attendance")]
pub fn attendance(conn: DatabaseConnection) -> Result<Template, Redirect> {
    let sessions = schema::Session::get_results(&conn.0).unwrap();

    Ok(Template::render(
        "attendance",
        context::Attendance {
            sessions,
            current: None,
        },
    ))
}

#[get("/attendance/<session_id>")]
pub fn session_attendance(conn: DatabaseConnection, session_id: i32) -> Result<Template, Redirect> {
    let sessions = schema::Session::get_results(&conn.0).unwrap();
    let current = schema::Session::find(session_id, &conn.0).ok();

    Ok(Template::render(
        "attendance",
        context::Attendance { sessions, current },
    ))
}
