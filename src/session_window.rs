//! Contains the [`SessionWindow`] type, that defines what sessions should be shown when.

use chrono::offset::Local;
use chrono::{DateTime, Datelike, Duration, Weekday};

/// Represents the sessions that should be shown.
#[derive(Copy, Clone, Debug)]
pub struct SessionWindow {
    /// The timestamp of the start of the window
    pub start: i64,
    /// The timestamp of the end of the window
    pub end: i64,
}

impl SessionWindow {
    /// Gets the window for the current time.
    pub fn from_current_time() -> Self {
        let now = Local::now() + Duration::hours(6);
        Self::from_time(now)
    }

    fn from_time(time: DateTime<Local>) -> Self {
        let one_week_prior = time - Duration::weeks(1);

        let last_sunday = (0..7)
            .map(|i| one_week_prior + Duration::days(i))
            .find(|d| d.weekday() == Weekday::Sun)
            .unwrap();

        let start = last_sunday.date_naive().and_hms_opt(18, 0, 0).unwrap();
        let end = start + Duration::weeks(1);

        Self {
            start: start.timestamp(),
            end: end.timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::TimeZone;

    #[test]
    fn basic_time_inference() {
        let time = Local.ymd(2020, 11, 16).and_hms(10, 28, 0);
        let window = SessionWindow::from_time(time);

        let start = Local.ymd(2020, 11, 15).and_hms(18, 0, 0);
        let end = Local.ymd(2020, 11, 22).and_hms(18, 0, 0);

        assert_eq!(start.timestamp(), window.start);
        assert_eq!(end.timestamp(), window.end);
    }

    #[test]
    fn start_of_week() {
        let time = Local.ymd(2020, 11, 16).and_hms(0, 0, 0);
        let window = SessionWindow::from_time(time);

        let start = Local.ymd(2020, 11, 15).and_hms(18, 0, 0);
        let end = Local.ymd(2020, 11, 22).and_hms(18, 0, 0);

        assert_eq!(start.timestamp(), window.start);
        assert_eq!(end.timestamp(), window.end);
    }

    #[test]
    fn end_of_week() {
        let time = Local.ymd(2020, 11, 22).and_hms(23, 59, 59);
        let window = SessionWindow::from_time(time);

        let start = Local.ymd(2020, 11, 15).and_hms(18, 0, 0);
        let end = Local.ymd(2020, 11, 22).and_hms(18, 0, 0);

        assert_eq!(start.timestamp(), window.start);
        assert_eq!(end.timestamp(), window.end);
    }

    #[test]
    fn start_of_year() {
        let time = Local.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let window = SessionWindow::from_time(time);

        let start = Local.ymd(2019, 12, 29).and_hms(18, 0, 0);
        let end = Local.ymd(2020, 1, 5).and_hms(18, 0, 0);

        assert_eq!(start.timestamp(), window.start);
        assert_eq!(end.timestamp(), window.end);
    }

    #[test]
    fn end_of_year() {
        let time = Local.ymd(2020, 12, 31).and_hms(0, 0, 0);
        let window = SessionWindow::from_time(time);

        let start = Local.ymd(2020, 12, 27).and_hms(18, 0, 0);
        let end = Local.ymd(2021, 1, 3).and_hms(18, 0, 0);

        assert_eq!(start.timestamp(), window.start);
        assert_eq!(end.timestamp(), window.end);
    }
}
