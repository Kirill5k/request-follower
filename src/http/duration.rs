use time::{Duration, OffsetDateTime};

pub struct FiniteDuration {
    seconds: i64,
    minutes: i64,
    hours: i64,
    days: i64
}

impl FiniteDuration {
    pub fn from(d: Duration) -> FiniteDuration {
        let seconds = d.whole_seconds();
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        FiniteDuration {
            seconds,
            minutes,
            hours,
            days
        }
    }

    pub fn between_now_and(other_date: OffsetDateTime) -> FiniteDuration {
        from(OffsetDateTime::now_utc() - other_date)
    }
}
