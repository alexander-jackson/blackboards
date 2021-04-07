//! Defines the routes, schema and handlers for the Sessions website.

#![warn(clippy::all)]
#![warn(missing_docs)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use fern::colors::{Color, ColoredLevelConfig};
use rocket::request::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

pub mod api;
pub mod auth;
pub mod context;
pub mod email;
pub mod forms;
pub mod frontend;
pub mod guards;
pub mod schema;
pub mod session_window;

/// Catches 401 error codes for redirecting.
#[catch(401)]
pub async fn unauthorised(req: &Request<'_>) -> Redirect {
    let uri = req.uri().to_string();
    log::debug!("Unauthorised user requested: {}", uri);

    // Encode the uri
    let encoded = base64::encode(&uri);

    Redirect::to(uri!(api::authenticate: encoded))
}

/// Catches 403 error codes for displaying a custom page.
#[catch(403)]
pub async fn forbidden(req: &Request<'_>) -> Template {
    let path = req.uri().path().as_str();

    Template::render("forbidden", context::Forbidden { path })
}

/// Builds the Rocket object defining the web server.
///
/// Adds the database connection and the template handler to the rocket, along with the routes that
/// are supported and returns the Rocket object ready to be launched.
pub fn build_rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(guards::DatabaseConnection::fairing())
        .attach(Template::fairing())
        .register(catchers![unauthorised, forbidden])
        .mount(
            "/",
            routes![
                frontend::sessions,
                frontend::specific_session,
                frontend::bookings,
                frontend::attendance,
                frontend::session_attendance,
                frontend::authenticated,
                frontend::blackboard,
                frontend::personal_bests,
                frontend::taskmaster_leaderboard,
                frontend::taskmaster_edit,
                frontend::elections,
                frontend::election_voting,
                frontend::election_results,
                frontend::election_settings,
                api::register,
                api::cancel,
                api::record_attendance,
                api::authenticate,
                api::authorised,
                api::personal_bests,
                api::logout,
                api::taskmaster_edit,
                api::election_vote,
                api::election_settings_toggle,
            ],
        )
}

/// Setup a logger with custom filters.
pub fn setup_logger_with_filters<Conditions, Name>(conditions: Conditions)
where
    Conditions: IntoIterator<Item = (Name, log::LevelFilter)>,
    Name: Into<String>,
{
    let colours_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    let mut dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{colours_line}[{date}][{target}][{level}]\x1B[0m {message}",
                colours_line = format_args!(
                    "\x1B[{}m",
                    colours_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message,
            ));
        })
        .level(log::LevelFilter::Warn);

    for (module_name, level) in conditions {
        dispatch = dispatch.level_for(module_name.into(), level);
    }

    dispatch
        .chain(std::io::stdout())
        .apply()
        .expect("Failed to initialise the logger");
}
