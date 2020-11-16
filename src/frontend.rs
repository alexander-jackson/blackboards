//! Handles the routes that return Templates for the user to view.

use std::collections::BTreeMap;

use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::context;
use crate::schema;

use crate::guards::{AuthorisedUser, DatabaseConnection};
use crate::session_window::SessionWindow;

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

fn get_registrations(
    conn: &diesel::SqliteConnection,
    window: SessionWindow,
) -> Option<Vec<context::Registrations>> {
    let unformatted = schema::Registration::get_registration_list(conn, window).unwrap();
    let formatted = format_registrations(unformatted);

    match formatted.len() {
        0 => None,
        _ => Some(formatted),
    }
}

/// Gets the information needed for the general dashboard and renders the template.
#[get("/sessions")]
pub fn dashboard(
    _user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
) -> Template {
    let window = SessionWindow::from_current_time();
    let sessions = schema::Session::get_results_between(&conn.0, window).unwrap();
    let message = flash.map(|f| f.msg().to_string());
    let registrations = get_registrations(&conn.0, window);

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

/// Gets the information needed for the session registration and renders the template.
#[get("/sessions/<session_id>")]
pub fn specific_session(
    _user: AuthorisedUser,
    conn: DatabaseConnection,
    session_id: i32,
) -> Result<Template, Redirect> {
    let window = SessionWindow::from_current_time();
    let sessions = schema::Session::get_results_between(&conn.0, window).unwrap();
    let current = schema::Session::find(session_id, &conn.0).ok();
    let registrations = get_registrations(&conn.0, window);

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

/// Gets the information needed for the attendance recording dashboard and renders the template.
#[get("/attendance")]
pub fn attendance(conn: DatabaseConnection) -> Result<Template, Redirect> {
    let sessions = schema::Session::get_results(&conn.0).unwrap();

    Ok(Template::render(
        "attendance",
        context::Attendance {
            sessions,
            current: None,
            message: None,
        },
    ))
}

/// Gets the information needed for the attendance recording and renders the template.
#[get("/attendance/<session_id>")]
pub fn session_attendance(
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
    session_id: i32,
) -> Result<Template, Redirect> {
    let sessions = schema::Session::get_results(&conn.0).unwrap();
    let current = schema::Session::find(session_id, &conn.0).ok();
    let message = flash.map(|f| f.msg().to_string());

    Ok(Template::render(
        "attendance",
        context::Attendance {
            sessions,
            current,
            message,
        },
    ))
}

/// Displays a small splash page after authenticating.
#[get("/authenticated")]
pub fn authenticated() -> Template {
    Template::render("authenticated", context::get_empty())
}

/// Displays a small splash page after authenticating.
#[get("/bookings")]
pub fn bookings(user: AuthorisedUser, conn: DatabaseConnection) -> Template {
    let window = SessionWindow::from_current_time();
    let sessions = schema::Registration::get_user_bookings(user.id, window, &conn.0).unwrap();

    Template::render(
        "bookings",
        context::Context {
            sessions,
            current: None,
            message: None,
            registrations: None,
        },
    )
}

/// Displays the PB board for people to view.
#[get("/")]
pub fn blackboard(user: AuthorisedUser, conn: DatabaseConnection) -> Template {
    let personal_bests = schema::PersonalBest::get_results(&conn.0).unwrap();
    let user_id = user.id;

    Template::render(
        "blackboard",
        context::Blackboard {
            personal_bests,
            user_id,
        },
    )
}

/// Allows the user to change their personal bests.
#[get("/pbs")]
pub fn personal_bests(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
) -> Template {
    let personal_bests = schema::PersonalBest::find(user.id, &conn.0).unwrap();
    let message = flash.map(|f| f.msg().to_string());

    Template::render(
        "personal_bests",
        context::PersonalBests {
            personal_bests,
            message,
        },
    )
}
