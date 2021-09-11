//! Handles the routes that return Templates for the user to view.

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::str::FromStr;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::Template;
use tallystick::{irv::Tally, Transfer};

use crate::{context, schema};

use crate::guards::{Db, ElectionAdmin, Generic, Member, SiteAdmin, User};
use crate::session_window::SessionWindow;

fn format_registrations(
    unformatted: Vec<schema::registration::SessionRegistration>,
) -> Vec<context::Registrations> {
    let mut map: BTreeMap<(schema::custom_types::DateTime, i32, String), Vec<String>> =
        BTreeMap::new();

    for sr in unformatted {
        map.entry((sr.start_time, sr.session_id, sr.title))
            .or_default()
            .push(sr.name);
    }

    map.into_iter()
        .map(|(key, value)| context::Registrations {
            start_time: key.0,
            title: key.2,
            members: value,
        })
        .collect()
}

async fn get_registrations(
    mut conn: Connection<Db>,
    window: SessionWindow,
) -> Option<Vec<context::Registrations>> {
    let unformatted = schema::Registration::get_registration_list(&mut *conn, window)
        .await
        .unwrap();

    let formatted = format_registrations(unformatted);

    match formatted.len() {
        0 => None,
        _ => Some(formatted),
    }
}

/// Gets the information needed for the sessions page and renders the template.
#[get("/sessions")]
pub async fn sessions(
    user: User<Generic>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let window = SessionWindow::from_current_time();

    let sessions = schema::Session::get_results_between(&mut *conn, window)
        .await
        .unwrap();

    let message = flash.map(context::Message::from);
    let registrations = get_registrations(conn, window).await;
    let is_site_admin = user.is_also::<SiteAdmin>();

    Template::render(
        "sessions",
        context::Context {
            sessions,
            current: None,
            message,
            registrations,
            is_site_admin,
        },
    )
}

/// Allows site administrators to manage the upcoming sessions.
#[get("/sessions/manage")]
pub async fn manage_sessions(
    _user: User<SiteAdmin>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let window = SessionWindow::from_current_time();

    let sessions = schema::Session::get_results_within_and_after(&mut *conn, window)
        .await
        .unwrap();

    let message = flash.map(context::Message::from);

    Template::render(
        "sessions_manage",
        context::ManageSessions {
            sessions,
            current: None,
            message,
        },
    )
}

/// Allows site administrators to manage a specific session.
#[get("/sessions/manage/<session_id>")]
pub async fn manage_specific_session(
    _user: User<SiteAdmin>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
    session_id: i32,
) -> Template {
    let window = SessionWindow::from_current_time();

    let sessions = schema::Session::get_results_within_and_after(&mut *conn, window)
        .await
        .unwrap();

    let current = schema::Session::find(session_id, &mut *conn)
        .await
        .ok()
        .flatten();

    let message = flash.map(context::Message::from);

    Template::render(
        "sessions_manage",
        context::ManageSessions {
            sessions,
            current,
            message,
        },
    )
}

/// Gets the information needed for the session registration and renders the template.
#[get("/sessions/<session_id>")]
pub async fn specific_session(
    user: User<Generic>,
    mut conn: Connection<Db>,
    session_id: i32,
) -> Template {
    let window = SessionWindow::from_current_time();

    let sessions = schema::Session::get_results_between(&mut *conn, window)
        .await
        .unwrap();

    let current = schema::Session::find(session_id, &mut *conn)
        .await
        .ok()
        .flatten();

    let registrations = get_registrations(conn, window).await;
    let is_site_admin = user.is_also::<SiteAdmin>();

    Template::render(
        "sessions",
        context::Context {
            sessions,
            current,
            message: None,
            registrations,
            is_site_admin,
        },
    )
}

/// Gets the information needed for the attendance recording dashboard and renders the template.
#[get("/attendance")]
pub async fn attendance(mut conn: Connection<Db>) -> Template {
    let sessions = schema::Session::get_results(&mut *conn).await.unwrap();

    Template::render(
        "attendance",
        context::Attendance {
            sessions,
            current: None,
            message: None,
        },
    )
}

/// Gets the information needed for the attendance recording and renders the template.
#[get("/attendance/<session_id>")]
pub async fn session_attendance(
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
    session_id: i32,
) -> Template {
    let sessions = schema::Session::get_results(&mut *conn).await.unwrap();
    let current = schema::Session::find(session_id, &mut *conn)
        .await
        .ok()
        .flatten();

    let message = flash.map(context::Message::from);

    Template::render(
        "attendance",
        context::Attendance {
            sessions,
            current,
            message,
        },
    )
}

/// Displays a small splash page after authenticating.
#[get("/authenticated/<uri>")]
pub fn authenticated(uri: &str) -> Template {
    // Decode the uri
    let bytes = base64::decode(&uri).unwrap();
    let uri = String::from_utf8(bytes).unwrap();
    log::debug!("Authenticated user, redirecting to: {}", uri);

    Template::render("authenticated", context::Authenticated { uri })
}

/// Displays a small splash page after authenticating.
#[get("/bookings")]
pub async fn bookings(user: User<Member>, mut conn: Connection<Db>) -> Template {
    let is_site_admin = user.is_also::<SiteAdmin>();

    let window = SessionWindow::from_current_time();
    let sessions = schema::Registration::get_user_bookings(user.id, window, &mut *conn)
        .await
        .unwrap();

    Template::render(
        "bookings",
        context::Context {
            sessions,
            current: None,
            message: None,
            registrations: None,
            is_site_admin,
        },
    )
}

/// Displays the PB board for people to view.
#[get("/")]
pub async fn blackboard(
    user: Option<User<Generic>>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let (pl, wl) = schema::PersonalBest::get_results(&mut *conn).await.unwrap();

    let user_id = user.map(|user| user.id);
    let message = flash.map(context::Message::from);

    Template::render(
        "blackboard",
        context::Blackboard {
            pl,
            wl,
            user_id,
            message,
        },
    )
}

/// Allows the user to change their personal bests.
#[get("/pbs")]
pub async fn personal_bests(
    user: User<Member>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let personal_bests = schema::PersonalBest::find(user.id, &user.name, &mut *conn)
        .await
        .unwrap();

    let message = flash.map(context::Message::from);
    let warning = personal_bests.check_for_show_without_values();

    Template::render(
        "personal_bests",
        context::PersonalBests {
            personal_bests,
            warning,
            message,
        },
    )
}

/// Shows the elections board.
#[get("/elections")]
pub async fn elections(
    user: User<Generic>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let exec_positions = schema::ExecPosition::get_results(&mut *conn).await.unwrap();

    let message = flash.map(context::Message::from);

    Template::render(
        "elections",
        context::Elections {
            exec_positions,
            message,
            admin: user.is_also::<ElectionAdmin>(),
        },
    )
}

/// Gets the information needed for the session registration and renders the template.
#[get("/elections/voting/<position_id>")]
pub async fn election_voting(
    user: User<Member>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
    position_id: i32,
) -> Result<Template, Flash<Redirect>> {
    // Check whether voting for this position is open
    let voting_is_open = schema::ExecPosition::voting_is_open(position_id, &mut *conn).await;

    if !voting_is_open {
        // Redirect to the main elections page
        return Err(Flash::error(
            Redirect::to(uri!(elections)),
            "Voting for this position either hasn't opened yet or has closed.",
        ));
    }

    let position_title = schema::ExecPosition::get_title(position_id, &mut *conn)
        .await
        .unwrap();

    let mut nominations = schema::Nomination::for_position_with_names(position_id, &mut *conn)
        .await
        .unwrap();

    let message = flash.map(context::Message::from);

    let current_ballot = schema::Vote::get_current_ballot(user.id, position_id, &mut *conn)
        .await
        .unwrap();

    // Randomly shuffle the nominations for each person
    let mut rng = rand::thread_rng();
    nominations.shuffle(&mut rng);

    Ok(Template::render(
        "election_voting",
        context::Voting {
            position_id,
            position_title,
            nominations,
            current_ballot,
            message,
        },
    ))
}

/// Resolves the winners of a tie based on a presidential vote ballot.
fn resolve_ties<'a, 'b>(
    winners: Vec<(i32, &'a str, usize)>,
    num_winners: usize,
    ballot: &'b [i32],
) -> Vec<(i32, &'a str, usize)> {
    // The given ballot is ordered, so we only need to use it for tie breaks
    let mut resolved = Vec::new();

    for (_, group) in &winners.into_iter().group_by(|k| k.2) {
        if num_winners == resolved.len() {
            break;
        }

        let mut group: Vec<_> = group.collect();
        let remaining = num_winners - resolved.len();

        // If we can include everyone here, do so
        if group.len() <= remaining {
            resolved.extend(group.into_iter());
            continue;
        }

        // Pick from here in order of preference
        for _ in 0..remaining {
            for id in ballot {
                if let Some(pos) = group.iter().position(|x| x.0 == *id) {
                    resolved.push(group.remove(pos));
                }

                if num_winners == resolved.len() {
                    return resolved;
                }
            }
        }
    }

    resolved
}

/// Counts all the ballots for a given position.
fn count_position_ballots<'a>(
    position_id: i32,
    votes: &mut Vec<schema::Vote>,
    positions: &'a BTreeMap<i32, schema::ExecPosition>,
    nominees: &'a HashMap<i32, String>,
) -> context::ElectionResult<'a> {
    if votes.is_empty() {
        return context::ElectionResult {
            position_id,
            title: positions[&position_id].title.clone(),
            winners: Vec::new(),
            voter_count: 0,
        };
    }

    // Sort the votes by `warwick_id` and then `ranking`
    votes.sort_by(|a, b| {
        a.warwick_id
            .cmp(&b.warwick_id)
            .then(a.ranking.cmp(&b.ranking))
    });

    let map = votes
        .iter_mut()
        .map(|v| (v.warwick_id, v.candidate_id))
        .into_group_map();

    let voter_count = map.len();
    let collected: Vec<_> = map.values().map(Vec::clone).collect();

    let num_winners = positions[&position_id].num_winners as usize;
    let mut tally: Tally<i32, usize> = Tally::new(Transfer::Meek);

    for vote in collected {
        tally.add_ref(&vote);
    }

    // Get the (candidate, rank) pairs
    let ranked = tally.tally_ranked();

    // Iterate once to find the rank of the last winner
    let last_winner_rank = ranked.get(num_winners - 1).map_or(usize::MAX, |r| r.rank);

    // Find all people with this rank or less
    let mut winners: Vec<_> = ranked
        .iter()
        .filter_map(|rc| {
            (rc.rank <= last_winner_rank)
                .then(|| (rc.candidate, nominees[&rc.candidate].as_str(), rc.rank))
        })
        .collect();

    // Resolve any ties
    if winners.len() > num_winners {
        // Get the identifier of the president and resolve the ties
        let id = i32::from_str(&env::var("PRESIDENT_ID").unwrap()).unwrap();
        winners = resolve_ties(winners, num_winners, &map[&id]);
    }

    let title = positions[&position_id].title.clone();

    log::debug!(
        "Voting has decided that {:?} has/have won the nomination for: {}",
        winners,
        title
    );

    context::ElectionResult {
        position_id,
        title,
        winners,
        voter_count,
    }
}

/// Calculates the results of the elections won so far.
#[get("/elections/results")]
pub async fn election_results(_user: User<ElectionAdmin>, mut conn: Connection<Db>) -> Template {
    // Get all the available positions
    let positions: BTreeMap<i32, schema::ExecPosition> =
        schema::ExecPosition::get_results(&mut *conn)
            .await
            .unwrap()
            .into_iter()
            .map(|pos| (pos.id, pos))
            .collect();

    // Map all the nominees from `warwick_id` -> `name`
    let nominees: HashMap<_, _> = schema::Candidate::get_results(&mut *conn)
        .await
        .unwrap()
        .into_iter()
        .map(|n| (n.warwick_id, n.name))
        .collect();

    // Pull all the votes so far
    let votes = schema::Vote::get_results(&mut *conn).await.unwrap();

    // Sort all the votes by position they are voting for
    let mut by_position: BTreeMap<i32, Vec<schema::Vote>> =
        positions.keys().map(|k| (*k, Vec::new())).collect();

    for vote in votes {
        by_position.get_mut(&vote.position_id).unwrap().push(vote);
    }

    let results: Vec<_> = by_position
        .iter_mut()
        .map(|(id, votes)| count_position_ballots(*id, votes, &positions, &nominees))
        .collect();

    let closed_positions = schema::ExecPosition::closed_identifiers(&mut *conn)
        .await
        .unwrap();

    log::trace!(
        "Only the following positions are closed, ignoring results for all others: {:?}",
        closed_positions
    );

    // All ties should have been resolved by the presidential vote, so elect users
    let all_winners: Vec<_> = results
        .iter()
        .filter_map(|r| {
            closed_positions
                .contains(&r.position_id)
                .then(|| &r.winners)
        })
        .flatten()
        .map(|w| w.0)
        .collect();

    schema::Candidate::mark_elected(&all_winners, &mut *conn)
        .await
        .unwrap();

    Template::render("election_results", context::ElectionResults { results })
}

/// Shows the elections settings page.
#[get("/elections/settings")]
pub async fn election_settings(
    _user: User<ElectionAdmin>,
    mut conn: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Template {
    let exec_positions = schema::ExecPosition::get_results(&mut *conn).await.unwrap();

    let message = flash.map(context::Message::from);

    Template::render(
        "election_settings",
        context::Elections {
            exec_positions,
            message,
            admin: true,
        },
    )
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use crate::context::ElectionResult;
    use crate::frontend::{count_position_ballots, resolve_ties};
    use crate::schema::{ExecPosition, Vote};

    #[test]
    fn ties_are_resolved_correctly() {
        let tie = vec![(1, "Candidate A", 2), (2, "Candidate B", 2)];
        let num_winners = 1;
        let ballot = [2, 1];

        let resolved = resolve_ties(tie, num_winners, &ballot);
        let expected = vec![(2, "Candidate B", 2)];

        assert_eq!(resolved, expected);
    }

    #[test]
    fn ties_are_resolved_correctly_with_multiple_winners() {
        let tie = vec![
            (1, "Candidate A", 2),
            (2, "Candidate B", 2),
            (3, "Candidate C", 3),
        ];
        let num_winners = 2;
        let ballot = [2, 1, 3];

        let resolved = resolve_ties(tie, num_winners, &ballot);
        let expected = vec![(1, "Candidate A", 2), (2, "Candidate B", 2)];

        assert_eq!(resolved, expected);
    }

    #[test]
    fn low_ranked_candidates_cannot_win_in_high_ranked_ties() {
        let tie = vec![(1, "Candidate A", 2), (2, "Candidate B", 2)];
        let num_winners = 1;
        let ballot = [3, 2, 1];

        let resolved = resolve_ties(tie, num_winners, &ballot);
        let expected = vec![(2, "Candidate B", 2)];

        assert_eq!(resolved, expected);
    }

    #[test]
    fn position_ballots_are_calculated_correctly() {
        let position_id = 1;
        let mut votes = vec![(1, 1, 2, 1), (1, 1, 3, 2)]
            .into_iter()
            .map(Vote::from)
            .collect();

        let mut positions = BTreeMap::new();
        positions.insert(
            1,
            ExecPosition {
                id: 1,
                title: String::from("pos"),
                num_winners: 1,
                open: true,
            },
        );

        let mut nominees = HashMap::new();
        nominees.insert(1, String::from("Candidate 1"));
        nominees.insert(2, String::from("Candidate 2"));
        nominees.insert(3, String::from("Candidate 3"));

        let result = count_position_ballots(position_id, &mut votes, &positions, &nominees);
        let expected = ElectionResult {
            position_id: 1,
            title: String::from("pos"),
            winners: vec![(2, "Candidate 2", 0)],
            voter_count: 1,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn multiple_ballots_are_resolved_correctly() {
        let position_id = 1;
        let mut votes = vec![(1, 1, 2, 1), (1, 1, 3, 2), (1, 2, 3, 1), (1, 3, 3, 1)]
            .into_iter()
            .map(Vote::from)
            .collect();

        let mut positions = BTreeMap::new();
        positions.insert(
            1,
            ExecPosition {
                id: 1,
                title: String::from("pos"),
                num_winners: 1,
                open: true,
            },
        );

        let mut nominees = HashMap::new();
        nominees.insert(1, String::from("Candidate 1"));
        nominees.insert(2, String::from("Candidate 2"));
        nominees.insert(3, String::from("Candidate 3"));

        let result = count_position_ballots(position_id, &mut votes, &positions, &nominees);
        let expected = ElectionResult {
            position_id: 1,
            title: String::from("pos"),
            winners: vec![(3, "Candidate 3", 0)],
            voter_count: 3,
        };

        assert_eq!(result, expected);
    }
}
