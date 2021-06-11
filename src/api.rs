//! Defines the backend API functions that get called by the frontend.
//!
//! Deals with processing data into the database from forms and returning error messages to the
//! frontend to be displayed.

use std::collections::HashMap;
use std::env;

use chrono::TimeZone;
use itertools::Itertools;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Flash, Redirect};

use crate::auth;
use crate::email;
use crate::forms;
use crate::frontend;
use crate::schema;

use crate::guards::{DatabaseConnection, ElectionAdmin, Generic, Member, SiteAdmin, User};

/// Creates a new session in the database.
#[post("/sessions/create", data = "<data>")]
pub async fn sessions_create(
    _user: User<SiteAdmin>,
    conn: DatabaseConnection,
    data: Form<forms::SessionCreate>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let formatted = format!("{} {}", data.date, data.start_time);

    let datetime = chrono::Local.datetime_from_str(&formatted, "%Y-%m-%d %H:%M");
    let timestamp = datetime.unwrap().timestamp();

    conn.run(move |c| schema::Session::create_and_insert(&c, data.title, timestamp, data.spaces))
        .await
        .unwrap();

    Flash::success(
        Redirect::to(uri!(frontend::manage_sessions)),
        "Successfully created the session!",
    )
}

/// Deletes a session in the database.
#[post("/sessions/delete", data = "<data>")]
pub async fn session_delete(
    _user: User<SiteAdmin>,
    conn: DatabaseConnection,
    data: Form<forms::SessionDelete>,
) -> Flash<Redirect> {
    let data = data.into_inner();

    conn.run(move |c| schema::Session::delete(&c, data.session_id))
        .await
        .unwrap();

    Flash::success(
        Redirect::to(uri!(frontend::manage_sessions)),
        "Successfully deleted the session!",
    )
}

/// Registers a user for a session, confirming their email if needed.
#[post("/session/register", data = "<data>")]
pub async fn register(
    user: User<Generic>,
    conn: DatabaseConnection,
    data: Form<forms::Register>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let registration = schema::Registration::new(data.session_id, user.id, user.name);
    let insertable = registration.clone();
    let session_id = registration.session_id;

    let result = conn.run(move |c| insertable.insert(&c)).await;
    let session = conn
        .run(move |c| schema::Session::find(session_id, &c))
        .await
        .unwrap();

    // Check whether they broke the database
    match result {
        Ok(_) => {
            email::send_confirmation(&registration, &session).await;

            Flash::success(
                Redirect::to(uri!(frontend::sessions)),
                "Successfully registered for the session!",
            )
        }
        Err(_) => Flash::error(
            Redirect::to(uri!(frontend::sessions)),
            "Failed to register for the session, have you already booked one or is it full?",
        ),
    }
}

/// Cancel a user's registration for a session.
#[post("/session/cancel", data = "<data>")]
pub async fn cancel(
    user: User<Generic>,
    conn: DatabaseConnection,
    data: Form<forms::Cancel>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let registration = conn
        .run(move |c| schema::Registration::cancel(user.id, data.session_id, &c))
        .await;

    // Check whether they broke the database
    match registration {
        Ok(_) => Flash::success(
            Redirect::to(uri!(frontend::sessions)),
            "Successfully cancelled the session!",
        ),
        Err(_) => Flash::error(
            Redirect::to(uri!(frontend::sessions)),
            "Failed to cancel the session, try again or let me know if it keeps happening.",
        ),
    }
}

/// Logs the user out and deletes their cookies.
#[get("/logout")]
pub fn logout(_user: User<Generic>, cookies: &CookieJar<'_>) -> &'static str {
    cookies.remove_private(Cookie::named("id"));
    cookies.remove_private(Cookie::named("name"));

    "Logged out"
}

/// Updates a user's personal bests.
#[post("/pbs", data = "<data>")]
pub async fn personal_bests(
    user: User<Member>,
    conn: DatabaseConnection,
    data: Form<forms::PersonalBests>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let result = conn
        .run(move |c| schema::PersonalBest::update(user.id, user.name, data, &c))
        .await;

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
pub async fn record_attendance(
    conn: DatabaseConnection,
    data: Form<forms::Attendance>,
) -> Flash<Redirect> {
    let data = data.into_inner();

    let result = conn
        .run(move |c| schema::Attendance::from(data).insert(&c))
        .await;

    // Record the attendance
    if let Err(e) = result {
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
#[get("/authenticate/<uri>")]
pub async fn authenticate(
    cookies: &CookieJar<'_>,
    conn: DatabaseConnection,
    uri: String,
) -> Redirect {
    // Check whether their cookie is already set
    if cookies.get_private("id").is_some() && cookies.get_private("name").is_some() {
        return Redirect::to(uri!(frontend::sessions));
    }

    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();

    let pair = auth::obtain_request_token(&consumer_key, &consumer_secret, &uri).await;
    let callback = auth::build_callback(&pair.token, &uri);

    // Write the secret to the database
    conn.run(move |c| schema::AuthPair::from(pair).insert(&c).unwrap())
        .await;

    Redirect::to(callback)
}

/// Represents the callback of the website. Users are sent here after signing in through SSO.
///
/// Gets the parameters from the query string and logs them to the terminal before requesting to
/// exchange the request token for an access token. If this succeeds, logs the token and displays
/// it on the frontend to the user.
#[get("/authorised/<uri>?<oauth_token>&<oauth_verifier>")]
pub async fn authorised(
    cookies: &CookieJar<'_>,
    uri: String,
    conn: DatabaseConnection,
    oauth_token: &str,
    oauth_verifier: &str,
) -> Redirect {
    let consumer_key = env::var("CONSUMER_KEY").unwrap();
    let consumer_secret = env::var("CONSUMER_SECRET").unwrap();

    let token = oauth_token.to_owned();
    let auth_pair = conn
        .run(move |c| schema::AuthPair::find(&token, &c).unwrap())
        .await;

    let pair = auth::exchange_request_for_access(
        &consumer_key,
        &consumer_secret,
        &auth_pair.token,
        &auth_pair.secret,
        oauth_verifier,
    )
    .await;

    // Request the user's information
    let user_info =
        auth::request_user_information(&pair.token, &pair.secret, &consumer_key, &consumer_secret)
            .await;

    // Set the user's id and name cookies
    cookies.add_private(Cookie::new("id", user_info.id.to_string()));
    cookies.add_private(Cookie::new("name", user_info.name));

    Redirect::to(uri!(frontend::authenticated: uri))
}

/// Allows users to vote on the election.
#[post("/election/vote/<position_id>", data = "<data>")]
pub async fn election_vote(
    user: User<Member>,
    conn: DatabaseConnection,
    position_id: i32,
    data: Form<HashMap<i32, i32>>,
) -> Flash<Redirect> {
    let data = data.into_inner();
    let redirect = Redirect::to(uri!(frontend::election_voting: position_id));

    let voting_is_open = conn
        .run(move |c| schema::ExecPosition::voting_is_open(position_id, &c))
        .await;

    // Check whether voting for this position is open
    if !voting_is_open {
        // Redirect to the main elections page
        return Flash::error(
            Redirect::to(uri!(frontend::elections)),
            "Voting for this position either hasn't opened yet or has closed.",
        );
    }

    // Check all the votes are unique
    let all_unique = data.values().unique().count() == data.values().count();

    if !all_unique {
        return Flash::error(redirect, "Make sure your votes are unique!");
    }

    // Check that they submitted a full ballot
    let candidates = conn
        .run(move |c| schema::Nomination::for_position_with_names(position_id, &c).unwrap())
        .await;

    if data.values().count() != candidates.len() {
        return Flash::error(
            redirect,
            "Please submit a full ballot, you need to choose an option for each box.",
        );
    }

    // Check whether the user is a candidate
    if candidates.iter().any(|(id, _)| *id == user.id) {
        return Flash::error(
            redirect,
            "You are a candidate for this position, so you cannot vote.",
        );
    }

    // Record the user's votes
    conn.run(move |c| schema::Vote::insert_all(user.id, position_id, &data, &c).unwrap())
        .await;

    Flash::success(redirect, "Successfully recorded your votes!")
}

/// Allows administrators to open and close voting for a position.
#[get("/elections/settings/toggle/<position_id>")]
pub async fn election_settings_toggle(
    _user: User<ElectionAdmin>,
    conn: DatabaseConnection,
    position_id: i32,
) -> Flash<Redirect> {
    conn.run(move |c| schema::ExecPosition::toggle_state(position_id, &c).unwrap())
        .await;

    Flash::success(
        Redirect::to(uri!(frontend::election_settings)),
        "Successfully toggled the state.",
    )
}
