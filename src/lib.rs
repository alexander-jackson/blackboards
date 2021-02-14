//! Defines the routes, schema and handlers for the Sessions website.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![feature(never_type)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

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
pub fn unauthorised() -> Redirect {
    Redirect::to(uri!(api::authenticate))
}

/// Builds the Rocket object defining the web server.
///
/// Adds the database connection and the template handler to the rocket, along with the routes that
/// are supported and returns the Rocket object ready to be launched.
pub fn build_rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(guards::DatabaseConnection::fairing())
        .attach(Template::fairing())
        .register(catchers![unauthorised])
        .mount(
            "/",
            routes![
                frontend::dashboard,
                frontend::specific_session,
                frontend::bookings,
                frontend::attendance,
                frontend::session_attendance,
                frontend::authenticated,
                frontend::blackboard,
                frontend::personal_bests,
                frontend::taskmaster_leaderboard,
                frontend::taskmaster_edit,
                api::register,
                api::cancel,
                api::record_attendance,
                api::authenticate,
                api::authorised,
                api::personal_bests,
                api::logout,
                api::taskmaster_edit,
            ],
        )
}
