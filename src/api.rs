use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use crate::forms;
use crate::frontend;
use crate::schema;

use crate::guards::DatabaseConnection;

#[post("/session/register", data = "<data>")]
pub fn register(conn: DatabaseConnection, data: Form<forms::Register>) -> Flash<Redirect> {
    let data = data.into_inner();

    // Check whether the user needs to be verified
    if !schema::VerifiedEmail::exists(data.warwick_id.0, &conn.0) {
        let request = schema::Request::create(data);
        request.insert(&conn.0).unwrap();

        return Flash::success(
            Redirect::to(uri!(frontend::dashboard)),
            "Successfully registered for the session, check your email to confirm it!",
        );
    }

    // User already has a verified email
    let registration = schema::Registration::create_from_verified(data);

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

#[get("/session/confirm/<id>")]
pub fn confirm_email(conn: DatabaseConnection, id: i32) -> Flash<Redirect> {
    schema::Request::verify(id, &conn.0).unwrap();

    Flash::success(
        Redirect::to(uri!(frontend::dashboard)),
        "Thanks for confirming your email, see you at the session!",
    )
}

#[post("/attendance/record", data = "<data>")]
pub fn record_attendance(conn: DatabaseConnection, data: Form<forms::Attendance>) -> Redirect {
    let data = data.into_inner();

    // Record the attendance
    schema::Attendance::create(data).insert(&conn.0).unwrap();

    Redirect::to(uri!(
        frontend::session_attendance: session_id = data.session_id
    ))
}
