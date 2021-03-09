//! Stores the expected structure of various forms for the user to submit.

use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use either::Either;
use rocket::http::RawStr;
use rocket::request::{FormItem, FormItems, FromForm, FromFormValue};

/// Defines a custom struct that can only contain a valid Warwick ID.
#[derive(Copy, Clone, Debug)]
pub struct WarwickId(pub i32);

impl<'v> FromFormValue<'v> for WarwickId {
    type Error = &'v RawStr;

    /// Converts a form value into a `WarwickId` if it matches the length.
    fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
        if !(form_value.chars().all(char::is_numeric) && form_value.len() == 7) {
            return Err(form_value);
        }

        Ok(Self(form_value.parse::<i32>().unwrap()))
    }
}

/// Custom type for recording votes from a form.
#[derive(Debug)]
pub struct RawMap<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> RawMap<K, V> {
    /// Gets a reference to the underlying [`HashMap`].
    pub fn into_inner(self) -> HashMap<K, V> {
        self.inner
    }
}

impl<'f, K, V> FromForm<'f> for RawMap<K, V>
where
    K: FromStr + Hash + Eq,
    V: FromStr,
{
    type Error = Either<<K as FromStr>::Err, <V as FromStr>::Err>;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        let convert = |item: FormItem<'f>| -> Result<(K, V), Self::Error> {
            let key = item.key.url_decode().unwrap();
            let value = item.value.url_decode().unwrap();

            Ok((
                K::from_str(&key).map_err(Either::Left)?,
                V::from_str(&value).map_err(Either::Right)?,
            ))
        };

        let inner = items.into_iter().map(convert).collect::<Result<_, _>>()?;

        Ok(Self { inner })
    }
}

/// Defines the contents of the registration form for a session.
#[derive(Debug, FromForm)]
pub struct Register {
    /// The identifier for the session.
    pub session_id: i32,
}

/// Defines the contents of the cancellation form for a session.
#[derive(Debug, FromForm)]
pub struct Cancel {
    /// The identifier for the session.
    pub session_id: i32,
}

/// Defines the contents of the attendance form for a session.
#[derive(Copy, Clone, Debug, FromForm)]
pub struct Attendance {
    /// The identifier for the session.
    pub session_id: i32,
    /// The user's Warwick ID.
    pub warwick_id: WarwickId,
}

/// Defines the contents of the personal bests form.
#[derive(Clone, Debug, FromForm)]
pub struct PersonalBests {
    /// The user's best squat.
    pub squat: Option<f32>,
    /// The user's best bench.
    pub bench: Option<f32>,
    /// The user's best deadlift.
    pub deadlift: Option<f32>,
    /// The user's best snatch.
    pub snatch: Option<f32>,
    /// The user's best clean and jerk.
    pub clean_and_jerk: Option<f32>,
    /// Whether to display the user on the PL board.
    pub show_pl: bool,
    /// Whether to display the user on the WL board.
    pub show_wl: bool,
}

/// Defines the contents of a taskmaster board update.
#[derive(Debug, FromForm)]
pub struct TaskmasterUpdate {
    /// The CSV representing the new board state.
    pub leaderboard: String,
}
