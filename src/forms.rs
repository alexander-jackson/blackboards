use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug)]
pub struct WarwickId(pub i32);

impl<'v> FromFormValue<'v> for WarwickId {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
        if !(form_value.chars().all(|c| c.is_numeric()) && form_value.len() == 7) {
            return Err(form_value);
        }

        Ok(Self(form_value.parse::<i32>().unwrap()))
    }
}

#[derive(Debug, FromForm)]
pub struct Register {
    pub session_id: i32,
    pub warwick_id: WarwickId,
    pub name: String,
}
