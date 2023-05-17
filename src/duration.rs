use regex::Regex;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, ops};
use time::OffsetDateTime;

#[derive(Debug, PartialEq)]
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

    pub fn as_days(&self) -> i64 {
        self.seconds / 86400
    }

    pub fn as_hours(&self) -> i64 {
        self.seconds / 3600
    }

    pub fn as_minutes(&self) -> i64 {
        self.seconds / 60
    }

    pub fn to_string(&self) -> String {
        let days = self.as_days();
        let rem_hours = self - FiniteDuration::from_days(days);
        let hours = rem_hours.as_hours();
        let rem_mins = rem_hours - FiniteDuration::from_hours(hours);
        let mins = rem_mins.as_minutes();
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

lazy_static! {
    static ref STRING_REPR_REGEX: Regex = Regex::new(r"^(\d+d)?(\d+h)?(\d+m)?(\d+s)?$").unwrap();
    static ref EXTRACT_DATA_REGEX: Regex =
        Regex::new(r"(?x)((?P<day>\d+)d)?((?P<hour>\d+)h)?((?P<min>\d+)m)?((?P<sec>\d+)s)?")
            .unwrap();
}

impl<'de> Visitor<'de> for FiniteDurationVisitor {
    type Value = FiniteDuration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the form of XXdXXhXXmXXs")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_empty() {
            Err(E::custom("received empty string"))
        } else if !STRING_REPR_REGEX.is_match(v) {
            Err(E::custom(
                "invalid string repr of FiniteDuration. expected format is XXdXXhXXmXXs",
            ))
        } else {
            let duration_seconds = match EXTRACT_DATA_REGEX.captures(v) {
                None => 0,
                Some(extracted_data) => {
                    let n_days = extracted_data
                        .name("day")
                        .map_or(0, |d| d.as_str().parse().unwrap());

                    let n_hours = extracted_data
                        .name("hour")
                        .map_or(0, |h| h.as_str().parse().unwrap());

                    let n_minutes = extracted_data
                        .name("min")
                        .map_or(0, |m| m.as_str().parse().unwrap());

                    let n_seconds = extracted_data
                        .name("sec")
                        .map_or(0, |s| s.as_str().parse().unwrap());

                    n_days * 86400 + n_hours * 3600 + n_minutes * 60 + n_seconds
                }
            };

            Ok(FiniteDuration::from_seconds(duration_seconds))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error as ValueError, StrDeserializer};
    use serde::de::IntoDeserializer;

    #[test]
    fn conversions() {
        let fd = FiniteDuration::from_seconds(129601);

        assert_eq!(fd.as_days(), 1);
        assert_eq!(fd.as_hours(), 36);
        assert_eq!(fd.as_minutes(), 2160);
    }

    #[test]
    fn subtract() {
        let fd_1 = FiniteDuration::from_seconds(3160);
        let fd_2 = FiniteDuration::from_seconds(3000);
        let result = fd_1 - fd_2;

        assert_eq!(result.seconds, 160)
    }

    #[test]
    fn to_string() {
        assert_eq!(FiniteDuration::from_days(2).to_string(), "2d");
        assert_eq!(FiniteDuration::from_hours(36).to_string(), "1d12h");
        assert_eq!(FiniteDuration::from_seconds(129601).to_string(), "1d12h1s");
        assert_eq!(FiniteDuration::from_seconds(0).to_string(), "0s");
    }

    #[test]
    fn deserialize_empty_string() {
        let deserializer: StrDeserializer<ValueError> = "".into_deserializer();
        let error = deserializer
            .deserialize_string(FiniteDurationVisitor)
            .unwrap_err();
        assert_eq!(error.to_string(), "received empty string");
    }

    #[test]
    fn deserialize_invalid_string_repr() {
        let deserializer: StrDeserializer<ValueError> = "foo".into_deserializer();
        let error = deserializer
            .deserialize_string(FiniteDurationVisitor)
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid string repr of FiniteDuration. expected format is XXdXXhXXmXXs"
        );
    }

    #[test]
    fn deserialize_negative_number() {
        let deserializer: StrDeserializer<ValueError> = "-10d".into_deserializer();
        let error = deserializer
            .deserialize_string(FiniteDurationVisitor)
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid string repr of FiniteDuration. expected format is XXdXXhXXmXXs"
        );
    }

    #[test]
    fn deserialize_days() {
        let deserializer: StrDeserializer<ValueError> = "10d".into_deserializer();
        let result = deserializer
            .deserialize_string(FiniteDurationVisitor)
            .unwrap();
        assert_eq!(result, FiniteDuration::from_days(10));
    }

    #[test]
    fn deserialize_hours() {
        let deserializer: StrDeserializer<ValueError> = "2h".into_deserializer();
        let result = deserializer
            .deserialize_string(FiniteDurationVisitor)
            .unwrap();
        assert_eq!(result, FiniteDuration::from_hours(2));
    }

    #[test]
    fn deserialize_full_string_repr() {
        let deserializer: StrDeserializer<ValueError> = "1d12h1m1s".into_deserializer();
        let result = deserializer
            .deserialize_string(FiniteDurationVisitor)
            .unwrap();
        assert_eq!(result, FiniteDuration::from_seconds(129661));
    }
}
