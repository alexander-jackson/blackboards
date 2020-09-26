use regex::Regex;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug)]
pub struct WarwickEmail(pub String);

impl<'v> FromFormValue<'v> for WarwickEmail {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<WarwickEmail, &'v RawStr> {
        lazy_static! {
            static ref RE: Regex = Regex::new("warwick.ac.uk").unwrap();
        }

        if RE.is_match(form_value) {
            Ok(WarwickEmail(form_value.to_string().replace("%40", "@")))
        } else {
            Err(form_value)
        }
    }
}

#[derive(Debug, FromForm)]
pub struct Register {
    pub session_id: i32,
    pub email: WarwickEmail,
    pub name: String,
}
