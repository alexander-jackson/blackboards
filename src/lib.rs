#![warn(clippy::all)]
#![warn(clippy::pedantic)]
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

mod api;
mod context;
mod email;
mod forms;
mod frontend;
mod guards;
mod schema;

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
            ],
        )
}
