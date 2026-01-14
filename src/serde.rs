//! Custom serde modules for handling API-specific serialization

/// Serialize/deserialize NaiveDate as YYYY-MM-DD format
pub(crate) mod naive_date {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serialize a NaiveDate as YYYY-MM-DD
    pub(crate) fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
    }

    /// Deserialize a NaiveDate from YYYY-MM-DD
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

/// Serialize/deserialize Option<NaiveDate> as YYYY-MM-DD format
pub(crate) mod naive_date_option {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serialize an Option<NaiveDate> as YYYY-MM-DD
    pub(crate) fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_str(&d.format("%Y-%m-%d").to_string()),
            None => serializer.serialize_none(),
        }
    }

    /// Deserialize an Option<NaiveDate> from YYYY-MM-DD
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let date =
                    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}

/// Serialize/deserialize Duration as seconds (i64)
pub(crate) mod duration_from_secs {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub(crate) fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(duration.as_secs() as i64)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = i64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs as u64))
    }
}
