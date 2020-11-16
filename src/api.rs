//! Defines the backend API functions that get called by the frontend.
//!
//! Deals with processing data into the database from forms and returning error messages to the
//! frontend to be displayed.

use std::env;

use rocket::http::{Cookie, Cookies, RawStr};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use crate::auth;
use crate::forms;
use crate::frontend;
use crate::schema;

use crate::guards::{AuthorisedUser, DatabaseConnection};

/// Registers a user for a session, confirming their email if needed.
#[post("/session/register", data = "<data>")]
pub fn register(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    data: Form<forms::Register>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let registration = schema::Registration::from((user, data));

    // Check whether they broke the database
    match registration.insert(&conn.0) {
        Ok(_) => Flash::success(
            Redirect::to(uri!(frontend::dashboard)),
            "Successfully registered for the session!",
        ),
        Err(_) => Flash::error(
            Redirect::to(uri!(frontend::dashboard)),
            "Failed to register for the session, have you already booked one?",
        ),
    }
}

/// Cancel a user's registration for a session.
#[post("/session/cancel", data = "<data>")]
pub fn cancel(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    data: Form<forms::Cancel>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let registration = schema::Registration::cancel(user.id, data.session_id, &conn.0);

    // Check whether they broke the database
    match registration {
        Ok(_) => Flash::success(
            Redirect::to(uri!(frontend::dashboard)),
            "Successfully cancelled the session!",
        ),
        Err(_) => Flash::error(
            Redirect::to(uri!(frontend::dashboard)),
            "Failed to cancel the session, try again or let me know if it keeps happening.",
        ),
    }
}

/// Logs the user out and deletes their cookies.
#[get("/logout")]
pub fn logout(_user: AuthorisedUser, mut cookies: Cookies) -> &'static str {
    cookies.remove_private(Cookie::named("id"));
    cookies.remove_private(Cookie::named("name"));

    "Logged out"
}

/// Updates a user's personal bests.
#[post("/pbs", data = "<data>")]
pub fn personal_bests(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    data: Form<forms::PersonalBests>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let original = schema::PersonalBest::find(user.id, &conn.0).unwrap();

    // Check whether they broke the database
    match original.update(user, data, &conn.0) {
        Ok(_) => Flash::success(
            Redirect::to(uri!(frontend::personal_bests)),
            "Successfully updated your PBs!",
        ),
        Err(_) => Flash::error(
            Redirect::to(uri!(frontend::personal_bests)),
            "Failed to update the PBs, try again or let me know if it keeps happening.",
        ),
    }
}

/// Records the attendance for a given Warwick ID at a session.
#[post("/attendance/record", data = "<data>")]
pub fn record_attendance(
    conn: DatabaseConnection,
    data: Form<forms::Attendance>,
) -> Flash<Redirect> {
    let data = data.into_inner();

    // Record the attendance
    if let Err(e) = schema::Attendance::from(data).insert(&conn.0) {
        use diesel::result::{DatabaseErrorKind, Error};

        let redirect = Redirect::to(uri!(frontend::session_attendance: data.session_id));
        let msg = match e {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => format!(
                "Attendance for {} has already been recorded for this session",
                data.warwick_id.0
            ),
            _ => String::from("Something happened in the database incorrectly"),
        };

        return Flash::error(redirect, msg);
    }

    Flash::success(
        Redirect::to(uri!(frontend::session_attendance: data.session_id)),
        format!("Recorded attendance for ID: {}", data.warwick_id.0),
    )
}

/// Begins the OAuth1 authentication process.
#[get("/authenticate")]
pub fn authenticate(mut cookies: Cookies, conn: DatabaseConnection) -> Redirect {
    // Check whether their cookie is already set
    if cookies.get_private("id").is_some() && cookies.get_private("name").is_some() {
        return Redirect::to(uri!(frontend::dashboard));
    }

    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();

    let pair = auth::obtain_request_token(&consumer_key, &consumer_secret);
    let callback = auth::build_callback(&pair.token);

    // Write the secret to the database
    schema::AuthPair::from(pair).insert(&conn.0).unwrap();

    Redirect::to(callback)
}

/// Represents the callback of the website. Users are sent here after signing in through SSO.
///
/// Gets the parameters from the query string and logs them to the terminal before requesting to
/// exchange the request token for an access token. If this succeeds, logs the token and displays
/// it on the frontend to the user.
#[get("/authorised?<oauth_token>&<user_id>&<oauth_verifier>")]
pub fn authorised(
    mut cookies: Cookies,
    conn: DatabaseConnection,
    oauth_token: &RawStr,
    user_id: &RawStr,
    oauth_verifier: &RawStr,
) -> Redirect {
    let request_token = oauth_token.as_str();
    let user_id = user_id.as_str();
    let oauth_verifier = oauth_verifier.as_str();

    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();
    let auth_pair = schema::AuthPair::find(request_token, &conn.0).unwrap();

    let pair = auth::exchange_request_for_access(
        &consumer_key,
        &consumer_secret,
        request_token,
        &auth_pair.secret,
        oauth_verifier,
    );

    // Request the user's information
    let user_info =
        auth::request_user_information(&pair.token, &pair.secret, &consumer_key, &consumer_secret);

    // Set the user's id and name cookies
    cookies.add_private(Cookie::new("id", user_info.id.to_string()));
    cookies.add_private(Cookie::new("name", user_info.name));

    Redirect::to(uri!(frontend::authenticated))
}
