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

use rocket_contrib::templates::Template;

pub mod api;
pub mod auth;
pub mod context;
pub mod email;
pub mod forms;
pub mod frontend;
pub mod guards;
pub mod schema;

/// Builds the Rocket object defining the web server.
///
/// Adds the database connection and the template handler to the rocket, along with the routes that
/// are supported and returns the Rocket object ready to be launched.
pub fn build_rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(guards::DatabaseConnection::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                frontend::dashboard,
                frontend::specific_session,
                frontend::attendance,
                frontend::session_attendance,
                api::register,
                api::confirm_email,
                api::record_attendance,
                api::authenticate,
                api::authorised,
            ],
        )
}
