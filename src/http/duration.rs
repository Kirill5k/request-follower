use time::{Duration, OffsetDateTime};
use std::ops;

pub struct FiniteDuration {
    seconds: i64
}

impl ops::Sub<FiniteDuration> for &FiniteDuration {
    type Output = FiniteDuration;

    fn sub(self, other: FiniteDuration) -> Self::Output {
        FiniteDuration {
            seconds: (self.seconds - other.seconds).abs()
        }
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

impl FiniteDuration {
    pub fn from_days(days: i64) -> FiniteDuration {
        FiniteDuration {
            seconds: days * 3600 * 24
        }
    }

    pub fn from_hours(hours: i64) -> FiniteDuration {
        FiniteDuration {
            seconds: hours * 3600
        }
    }

    pub fn from_minutes(minutes: i64) -> FiniteDuration {
        FiniteDuration {
            seconds: minutes * 60
        }
    }

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

    pub fn days(&self) -> i64 {
        self.seconds / 86400
    }

    pub fn hours(&self) -> i64 {
        self.seconds / 3600
    }

    pub fn minutes(&self) -> i64 {
        self.seconds / 60
    }

    pub fn to_string(&self) -> String {
        let days = self.days();
        let rem_hours = self - FiniteDuration::from_days(days);

        let days_str = if days > 0 { format!("{}d", days) } else { "".to_string() };

        days_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_duration() {
        let duration = Duration::new(129601, 0);
        let finite_duration = FiniteDuration::from(duration);

        assert_eq!(1, finite_duration.days());
        assert_eq!(36, finite_duration.hours());
        assert_eq!(2160, finite_duration.minutes());
    }

    #[test]
    fn subtract() {
        let fd_1 = FiniteDuration::from_seconds(3160);
        let fd_2 = FiniteDuration::from_seconds(3000);
        let result = fd_1 - fd_2;

        assert_eq!(160, result.seconds)
    }

    #[test]
    fn to_string() {
        let fd = FiniteDuration::from_seconds(129601);

        assert_eq!("1d", fd.to_string())
    }
}
