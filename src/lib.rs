//! Defines the routes, schema and handlers for the Sessions website.

#![warn(clippy::all)]
#![warn(missing_docs)]

use std::net::{Ipv4Addr, SocketAddrV4};

use axum::extract::FromRef;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Extension, Router, Server};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use tera::Tera;
use testing::{HandlerContext, RedirectLayer};
use tower_cookies::CookieManagerLayer;

pub mod api;
pub mod auth;
pub mod context;
pub mod email;
pub mod forms;
pub mod frontend;
pub mod guards;
pub mod persistence;
pub mod schema;
pub mod session_window;

// #[catch(403)]
// pub async fn forbidden(req: &Request<'_>) -> Template {
//     let path = req.uri().path().as_str();
//     tracing::warn!(%path, "Caught a 403 Forbidden response");
//
//     Template::render("forbidden", context::Forbidden { path })
// }

fn unauthorised(context: &HandlerContext) -> Response {
    // Encode the uri
    let encoded = base64::encode(context.uri.path());
    let redirect = format!("/authenticate/{encoded}");

    Redirect::to(&redirect).into_response()
}

/// Wraps the inner `Tera` object with additional utility methods.
#[derive(Clone)]
pub struct Templates {
    inner: Tera,
}

impl Templates {
    fn new(path: &str) -> Self {
        Self {
            inner: Tera::new(path).unwrap(),
        }
    }

    fn render_with<S: Serialize>(&self, template: &str, context: &S) -> Html<String> {
        let context = tera::Context::from_serialize(context).unwrap();
        let template = format!("{}.html.tera", template);

        self.inner.render(&template, &context).unwrap().into()
    }
}

/// The state of the application.
#[derive(Clone)]
pub struct State {
    flash_config: axum_flash::Config,
    templates: Templates,
}

impl FromRef<State> for axum_flash::Config {
    fn from_ref(state: &State) -> axum_flash::Config {
        state.flash_config.clone()
    }
}

impl FromRef<State> for Templates {
    fn from_ref(state: &State) -> Templates {
        state.templates.clone()
    }
}

/// Builds the router, binds it to the server and handles incoming requests.
pub async fn build_router(pool: Pool<Postgres>) {
    let key = axum_flash::Key::generate();
    let flash_config = axum_flash::Config::new(key);

    let state = State {
        flash_config,
        templates: Templates::new("templates/*"),
    };

    let redirect_layer = RedirectLayer::new(StatusCode::UNAUTHORIZED, unauthorised);

    let router = Router::new()
        .route("/", get(frontend::blackboard))
        .route("/pbs", get(frontend::personal_bests))
        .route("/sessions", get(frontend::sessions))
        .route("/sessions/manage", get(frontend::manage_sessions))
        .route("/sessions/create", post(api::sessions_create))
        .route("/elections", get(frontend::elections))
        .route("/elections/settings", get(frontend::election_settings))
        .route("/elections/results", get(frontend::election_results))
        .route("/bookings", get(frontend::bookings))
        .route("/attendance", get(frontend::attendance))
        .route("/authenticate/:redirect", get(api::authenticate))
        .layer(CookieManagerLayer::new())
        .layer(Extension(pool))
        .layer(redirect_layer)
        .with_state(state);

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4000).into();
    let server = Server::bind(&addr).serve(router.into_make_service());

    server.await.unwrap();
}

// pub fn build_rocket(config: Figment) -> rocket::Rocket<rocket::Build> {
//     rocket::custom(config)
//         .attach(guards::Db::init())
//         .attach(Template::fairing())
//         .register("/", catchers![unauthorised, forbidden])
//         .mount(
//             "/assets",
//             FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")),
//         )
//         .mount(
//             "/",
//             routes![
//                 frontend::sessions,
//                 frontend::manage_sessions,
//                 frontend::manage_specific_session,
//                 frontend::specific_session,
//                 frontend::bookings,
//                 frontend::attendance,
//                 frontend::session_attendance,
//                 frontend::authenticated,
//                 frontend::blackboard,
//                 frontend::personal_bests,
//                 frontend::elections,
//                 frontend::election_voting,
//                 frontend::election_results,
//                 frontend::election_settings,
//                 api::sessions_create,
//                 api::session_delete,
//                 api::register,
//                 api::cancel,
//                 api::record_attendance,
//                 api::authenticate,
//                 api::authorised,
//                 api::personal_bests,
//                 api::logout,
//                 api::election_vote,
//                 api::election_settings_toggle,
//             ],
//         )
// }
