//! Custom serde modules for handling API-specific serialization

#![allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde function signatures"
)]

/// Serialize/deserialize DateTime<Utc> as ISO 8601 format
pub mod naive_date {
    use chrono::{DateTime, NaiveDate, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serialize a DateTime<Utc> as ISO 8601 (YYYY-MM-DDTHH:MM:SSZ)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.to_rfc3339())
    }

    /// Deserialize a DateTime<Utc> from ISO 8601 or YYYY-MM-DD
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Try parsing as full ISO 8601 datetime first
        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            return Ok(dt.with_timezone(&Utc));
        }
        // Fallback to parsing as YYYY-MM-DD date
        let naive_date =
            NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
        // When only a date is given, assume it's the start of the day in UTC.
        Ok(Utc
            .from_local_datetime(
                &naive_date
                    .and_hms_opt(0, 0, 0)
                    .expect("Infallible conversion"),
            )
            .unwrap())
    }
}

/// Serialize/deserialize Duration as seconds (i64)
pub mod duration_from_secs {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(duration.as_secs() as i64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = i64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs as u64))
    }
}
