use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::{ops, fmt};
use time::{OffsetDateTime};

#[derive(Debug)]
pub struct FiniteDuration {
    seconds: i64,
}

impl ops::Sub<FiniteDuration> for &FiniteDuration {
    type Output = FiniteDuration;

    fn sub(self, other: FiniteDuration) -> Self::Output {
        FiniteDuration {
            seconds: (self.seconds - other.seconds).abs(),
        }
    }
}

impl ops::Sub<FiniteDuration> for FiniteDuration {
    type Output = FiniteDuration;

    fn sub(self, other: FiniteDuration) -> Self::Output {
        FiniteDuration {
            seconds: (self.seconds - other.seconds).abs(),
        }
    }
}

impl FiniteDuration {
    pub fn from_days(days: i64) -> FiniteDuration {
        FiniteDuration {
            seconds: days * 3600 * 24,
        }
    }

    pub fn from_hours(hours: i64) -> FiniteDuration {
        FiniteDuration {
            seconds: hours * 3600,
        }
    }

    pub fn from_minutes(minutes: i64) -> FiniteDuration {
        FiniteDuration {
            seconds: minutes * 60,
        }
    }

    pub fn from_seconds(seconds: i64) -> FiniteDuration {
        FiniteDuration { seconds }
    }

    pub fn between_now_and(other_date: OffsetDateTime) -> FiniteDuration {
        let seconds = (OffsetDateTime::now_utc() - other_date).whole_seconds();
        FiniteDuration::from_seconds(seconds)
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
        let hours = rem_hours.hours();
        let rem_mins = rem_hours - FiniteDuration::from_hours(hours);
        let mins = rem_mins.minutes();
        let rem_secs = rem_mins - FiniteDuration::from_seconds(mins);
        let seconds = rem_secs.seconds;

        let mut str_repr: String = String::new();
        if days > 0 {
            str_repr.push_str(format!("{}d", days).as_str());
        }
        if hours > 0 {
            str_repr.push_str(format!("{}h", hours).as_str());
        }
        if mins > 0 {
            str_repr.push_str(format!("{}m", mins).as_str());
        }
        if seconds > 0 {
            str_repr.push_str(format!("{}s", seconds).as_str());
        }
        if str_repr.is_empty() {
            String::from("0s")
        } else {
            str_repr
        }
    }
}

impl Serialize for FiniteDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for FiniteDuration {
    fn deserialize<D>(deserializer: D) -> Result<FiniteDuration, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_string(FiniteDurationVisitor)
    }
}

struct FiniteDurationVisitor;

impl<'de> Visitor<'de> for FiniteDurationVisitor {
    type Value = FiniteDuration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the form of XXdXXhXXmXXs")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        Err(E::custom(format!("tried to deserialize {v}, however this function is not yet implemented")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let fd = FiniteDuration::from_seconds(129601);

        assert_eq!(1, fd.days());
        assert_eq!(36, fd.hours());
        assert_eq!(2160, fd.minutes());
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
        assert_eq!("2d", FiniteDuration::from_days(2).to_string());
        assert_eq!("1d12h", FiniteDuration::from_hours(36).to_string());
        assert_eq!("1d12h1s", FiniteDuration::from_seconds(129601).to_string());
        assert_eq!("0s", FiniteDuration::from_seconds(0).to_string());
    }
}
