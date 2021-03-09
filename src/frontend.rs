//! Handles the routes that return Templates for the user to view.

use std::collections::{BTreeMap, HashMap};

use rand::seq::SliceRandom;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use tallystick::{irv::Tally, Transfer};

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
    conn: &diesel::PgConnection,
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
    let message = flash.map(context::Message::from);
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
    let message = flash.map(context::Message::from);

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
    let (pl, wl) = schema::PersonalBest::get_results(&conn.0).unwrap();
    let user_id = user.id;

    Template::render("blackboard", context::Blackboard { pl, wl, user_id })
}

/// Allows the user to change their personal bests.
#[get("/pbs")]
pub fn personal_bests(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
) -> Template {
    let personal_bests = schema::PersonalBest::find(user, &conn.0).unwrap();
    let message = flash.map(context::Message::from);

    Template::render(
        "personal_bests",
        context::PersonalBests {
            personal_bests,
            message,
        },
    )
}

/// Displays the state of the Taskmaster leaderboard.
#[get("/taskmaster")]
pub fn taskmaster_leaderboard(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
) -> Template {
    let leaderboard = schema::TaskmasterEntry::get_results(&conn.0).unwrap();
    let message = flash.map(context::Message::from);

    Template::render(
        "taskmaster_leaderboard",
        context::TaskmasterLeaderboard {
            leaderboard,
            admin: user.is_taskmaster_admin(),
            message,
        },
    )
}

/// Allows authorised users to edit the Taskmaster leaderboard.
#[get("/taskmaster/edit")]
pub fn taskmaster_edit(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
) -> Result<Template, Redirect> {
    if !user.is_taskmaster_admin() {
        return Err(Redirect::to(uri!(taskmaster_leaderboard)));
    }

    let leaderboard = schema::TaskmasterEntry::get_results(&conn.0).unwrap();
    let message = flash.map(context::Message::from);

    let leaderboard_csv: String = leaderboard
        .iter()
        .map(|entry| format!("{},{}", entry.name, entry.score))
        .collect::<Vec<String>>()
        .join("\n");

    let template = Template::render(
        "taskmaster_edit",
        context::TaskmasterEdit {
            leaderboard_csv,
            message,
        },
    );

    Ok(template)
}

/// Shows the elections board.
#[get("/elections")]
pub fn elections(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
) -> Result<Template, Redirect> {
    let exec_positions = schema::ExecPosition::get_results(&conn.0).unwrap();
    let message = flash.map(context::Message::from);

    Ok(Template::render(
        "elections",
        context::Elections {
            exec_positions,
            message,
            admin: user.is_election_admin(),
        },
    ))
}

/// Gets the information needed for the session registration and renders the template.
#[get("/elections/voting/<position_id>")]
pub fn election_voting(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
    position_id: i32,
) -> Result<Template, Flash<Redirect>> {
    // Check whether voting for this position is open
    if !schema::ExecPosition::voting_is_open(position_id, &conn.0) {
        // Redirect to the main elections page
        return Err(Flash::error(
            Redirect::to(uri!(elections)),
            "Voting for this position either hasn't opened yet or has closed.",
        ));
    }

    if !user.is_barbell_member() {
        // Redirect to the main elections page
        return Err(Flash::error(
            Redirect::to(uri!(elections)),
            "You are not a Barbell member, so you cannot vote in this election.",
        ));
    }

    let mut nominations =
        schema::Nomination::for_position_with_names(position_id, &conn.0).unwrap();
    let message = flash.map(context::Message::from);
    let current_ballot = schema::Vote::get_current_ballot(user.id, position_id, &conn.0).unwrap();

    // Randomly shuffle the nominations for each person
    let mut rng = rand::thread_rng();
    nominations.shuffle(&mut rng);

    Ok(Template::render(
        "election_voting",
        context::Voting {
            position_id,
            nominations,
            current_ballot,
            message,
        },
    ))
}

/// Calculates the results of the elections won so far.
#[get("/elections/results")]
pub fn election_results(
    user: AuthorisedUser,
    conn: DatabaseConnection,
) -> Result<Template, Flash<Redirect>> {
    if !user.is_election_admin() {
        return Err(Flash::error(
            Redirect::to(uri!(elections)),
            "You aren't an admin, so you cannot see the results.",
        ));
    }

    // Get all the available positions
    let positions: BTreeMap<i32, schema::ExecPosition> = schema::ExecPosition::get_results(&conn.0)
        .unwrap()
        .into_iter()
        .map(|pos| (pos.id, pos))
        .collect();

    // Map all the nominees from `warwick_id` -> `name`
    let nominees: HashMap<_, _> = schema::Candidate::get_results(&conn.0)
        .unwrap()
        .into_iter()
        .map(|n| (n.warwick_id, n.name))
        .collect();

    // Pull all the votes so far
    let votes = schema::Vote::get_results(&conn.0).unwrap();

    // Sort all the votes by position they are voting for
    let mut by_position: BTreeMap<i32, Vec<schema::Vote>> =
        positions.keys().map(|k| (*k, Vec::new())).collect();

    for vote in votes {
        by_position.get_mut(&vote.position_id).unwrap().push(vote);
    }

    let results: Vec<_> = by_position
        .iter_mut()
        .map(|(position_id, votes)| {
            // Sort the votes by `warwick_id` and then `ranking`
            votes.sort_by(|a, b| {
                a.warwick_id
                    .cmp(&b.warwick_id)
                    .then(a.ranking.cmp(&b.ranking))
            });

            let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

            for vote in votes {
                map.entry(vote.warwick_id)
                    .or_default()
                    .push(vote.candidate_id);
            }

            let voter_count = map.len();
            let collected: Vec<_> = map.values().map(Vec::clone).collect();

            let num_winners = positions[position_id].num_winners as usize;
            let mut tally: Tally<i32, usize> = Tally::new(Transfer::Meek);

            for vote in &collected {
                tally.add_ref(vote);
            }

            // Get the (candidate, rank) pairs
            let ranked = tally.tally_ranked();

            // Iterate once to find the rank of the last winner
            let last_winner_rank = ranked
                .get(num_winners - 1)
                .map(|r| r.1)
                .unwrap_or(usize::MAX);

            // Find all people with this rank or less
            let winners: Vec<_> = ranked
                .iter()
                .filter_map(|(c, r)| (*r <= last_winner_rank).then(|| nominees[c].as_str()))
                .collect();

            context::ElectionResult {
                title: positions[position_id].title.clone(),
                winners,
                voter_count,
            }
        })
        .collect();

    Ok(Template::render(
        "election_results",
        context::ElectionResults { results },
    ))
}

/// Shows the elections settings page.
#[get("/elections/settings")]
pub fn election_settings(
    user: AuthorisedUser,
    conn: DatabaseConnection,
    flash: Option<FlashMessage>,
) -> Result<Template, Flash<Redirect>> {
    if !user.is_election_admin() {
        return Err(Flash::error(
            Redirect::to(uri!(elections)),
            "You aren't an admin, so you cannot see the settings.",
        ));
    }

    let exec_positions = schema::ExecPosition::get_results(&conn.0).unwrap();
    let message = flash.map(context::Message::from);

    Ok(Template::render(
        "election_settings",
        context::Elections {
            exec_positions,
            message,
            admin: user.is_election_admin(),
        },
    ))
}
