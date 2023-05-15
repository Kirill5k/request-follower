use time::{Duration, OffsetDateTime};
use std::ops;

pub struct FiniteDuration {
    seconds: i64
}

impl FiniteDuration {
    pub fn from_seconds(seconds: i64) -> FiniteDuration {
        FiniteDuration { seconds }
    }

    pub fn from(d: Duration) -> FiniteDuration {
        let seconds = d.whole_seconds();
        FiniteDuration { seconds }
    }

    pub fn between_now_and(other_date: OffsetDateTime) -> FiniteDuration {
        FiniteDuration::from(OffsetDateTime::now_utc() - other_date)
    }

    pub fn to_days(&self) -> i64 {
        self.seconds / 86400
    }

    pub fn days(&self) -> FiniteDuration {
        let diff = self.seconds - (self.to_days() * 86400);
        FiniteDuration {
            seconds: self.seconds - diff
        }
    }

    pub fn to_hours(&self) -> i64 {
        self.seconds / 3600
    }

    pub fn hours(&self) -> FiniteDuration {
        let diff = self.seconds - (self.to_hours() * 3600);
        FiniteDuration {
            seconds: self.seconds - diff
        }
    }

    pub fn to_minutes(&self) -> i64 {
        self.seconds / 60
    }

    pub fn minutes(&self) -> FiniteDuration {
        let diff = self.seconds - (self.to_minutes() * 60);
        FiniteDuration {
            seconds: self.seconds - diff
        }
    }

    pub fn to_string(&self) -> String {
        String::from("")
    }
}

impl ops::Sub<FiniteDuration> for FiniteDuration {
    type Output = FiniteDuration;

    fn sub(self, other: FiniteDuration) -> Self::Output {
        FiniteDuration {
            seconds: (self.seconds - other.seconds).abs()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_duration() {
        let duration = Duration::new(129601, 0);
        let finite_duration = FiniteDuration::from(duration);

        assert_eq!(1, finite_duration.to_days());
        assert_eq!(36, finite_duration.to_hours());
        assert_eq!(2160, finite_duration.to_minutes());
        assert_eq!(129600, finite_duration.minutes().seconds);
        assert_eq!(129600, finite_duration.hours().seconds);
        assert_eq!(86400, finite_duration.days().seconds);
    }

    #[test]
    fn subtract() {
        let fd_1 = FiniteDuration::from_seconds(3160);
        let fd_2 = FiniteDuration::from_seconds(3000);
        let result = fd_1 - fd_2;

        assert_eq!(160, result.seconds)
    }
}
