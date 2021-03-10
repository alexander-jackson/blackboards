//! Defines the backend API functions that get called by the frontend.
//!
//! Deals with processing data into the database from forms and returning error messages to the
//! frontend to be displayed.

use std::env;

use itertools::Itertools;
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
    let result = schema::PersonalBest::update(user, data, &conn.0);

    // Check whether they broke the database
    match result {
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
#[get("/authorised?<oauth_token>&<oauth_verifier>")]
pub fn authorised(
    mut cookies: Cookies,
    conn: DatabaseConnection,
    oauth_token: &RawStr,
    oauth_verifier: &RawStr,
) -> Redirect {
    let request_token = oauth_token.as_str();
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

/// Allows the Taskmaster leaderboard to be edited.
#[post("/taskmaster/edit", data = "<data>")]
pub fn taskmaster_edit(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    data: Form<forms::TaskmasterUpdate>,
) -> Redirect {
    if user.is_taskmaster_admin() {
        schema::TaskmasterEntry::update_all(&data.leaderboard, &conn.0).unwrap();
    }

    Redirect::to(uri!(frontend::taskmaster_leaderboard))
}

/// Allows users to vote on the election.
#[post("/election/vote/<position_id>", data = "<data>")]
pub fn election_vote(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    position_id: i32,
    data: Form<forms::RawMap<i32, i32>>,
) -> Flash<Redirect> {
    let data = data.into_inner().into_inner();
    let redirect = Redirect::to(uri!(frontend::election_voting: position_id));

    // Check whether voting for this position is open
    if !schema::ExecPosition::voting_is_open(position_id, &conn.0) {
        // Redirect to the main elections page
        return Flash::error(
            Redirect::to(uri!(frontend::elections)),
            "Voting for this position either hasn't opened yet or has closed.",
        );
    }

    if !user.is_barbell_member() {
        // Redirect to the main elections page
        return Flash::error(
            Redirect::to(uri!(frontend::elections)),
            "You are not a Barbell member, so you cannot vote in this election.",
        );
    }

    // Check all the votes are unique
    let all_unique = data.values().unique().count() == data.values().count();

    if !all_unique {
        return Flash::error(redirect, "Make sure your votes are unique!");
    }

    // Check that they submitted a full ballot
    let candidates = schema::Nomination::for_position_with_names(position_id, &conn.0).unwrap();

    if data.values().count() != candidates.len() {
        return Flash::error(
            redirect,
            "Please submit a full ballot, you need to choose an option for each box.",
        );
    }

    // Check whether the user is a candidate
    if candidates.iter().find(|(id, _)| *id == user.id).is_some() {
        return Flash::error(
            redirect,
            "You are a candidate for this position, so you cannot vote.",
        );
    }

    // Record the user's votes
    schema::Vote::insert_all(user.id, position_id, &data, &conn.0).unwrap();

    Flash::success(redirect, "Successfully recorded your votes!")
}

/// Allows administrators to open and close voting for a position.
#[get("/elections/settings/toggle/<position_id>")]
pub fn election_settings_toggle(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    position_id: i32,
) -> Flash<Redirect> {
    // Check whether the user can make this change
    if !user.is_election_admin() {
        return Flash::error(
            Redirect::to(uri!(frontend::elections)),
            "You do not have permission to toggle election voting.",
        );
    }

    schema::ExecPosition::toggle_state(position_id, &conn.0).unwrap();

    Flash::success(
        Redirect::to(uri!(frontend::election_settings)),
        "Successfully toggled the state.",
    )
}
