//! Defines the routes, schema and handlers for the Sessions website.

#![warn(clippy::all)]
#![warn(missing_docs)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::env;

use rocket::fs::FileServer;
use rocket::{figment::providers::Env, request::Request};
use rocket::{figment::Figment, response::Redirect, Config};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

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
    tracing::debug!(%uri, "Caught a 401 Unauthorized response");

    // Encode the uri
    let encoded = base64::encode(&uri);

    Redirect::to(uri!(api::authenticate(encoded)))
}

/// Catches 403 error codes for displaying a custom page.
#[catch(403)]
pub async fn forbidden(req: &Request<'_>) -> Template {
    let path = req.uri().path().as_str();
    tracing::warn!(%path, "Caught a 403 Forbidden response");

    Template::render("forbidden", context::Forbidden { path })
}

/// Builds the configuration for the Rocket instance.
fn config_from_env() -> Figment {
    let mut databases = HashMap::new();
    let mut urls = HashMap::new();

    let database_url =
        env::var("DATABASE_URL").expect("Failed to find `DATABASE_URL` in the environment");

    urls.insert("url", database_url);
    databases.insert("blackboards", urls);

    Figment::from(Config::default())
        .merge(Env::prefixed("ROCKET_").global())
        .merge(("log_level", "off"))
        .merge(("databases", databases))
}

/// Builds the Rocket object defining the web server.
///
/// Adds the database connection and the template handler to the rocket, along with the routes that
/// are supported and returns the Rocket object ready to be launched.
pub fn build_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::custom(config_from_env())
        .attach(guards::Db::init())
        .attach(Template::fairing())
        .register("/", catchers![unauthorised, forbidden])
        .mount(
            "/assets",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")),
        )
        .mount(
            "/",
            routes![
                frontend::sessions,
                frontend::manage_sessions,
                frontend::manage_specific_session,
                frontend::specific_session,
                frontend::bookings,
                frontend::attendance,
                frontend::session_attendance,
                frontend::authenticated,
                frontend::blackboard,
                frontend::personal_bests,
                frontend::elections,
                frontend::election_voting,
                frontend::election_results,
                frontend::election_settings,
                api::sessions_create,
                api::session_delete,
                api::register,
                api::cancel,
                api::record_attendance,
                api::authenticate,
                api::authorised,
                api::personal_bests,
                api::logout,
                api::election_vote,
                api::election_settings_toggle,
            ],
        )
}
