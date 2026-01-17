//! Custom serde modules for handling API-specific serialization

#![allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde function signatures"
)]

use std::{fmt, str::FromStr as _};

use rust_decimal::{Decimal, prelude::FromPrimitive as _};
use serde::{Deserializer, de};

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
            .single()
            .expect("Valid UTC datetime"))
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

/// Deserialize a `Decimal` from a string, number, or null.
///
/// This function is designed to be robust and can handle various formats:
/// - Standard decimal strings: `"123.45"`
/// - Integers: `123`
/// - Strings with thousands separators (either `.` or `,`): `"1,234.56"`, `"1.234,56"`
/// - Strings with currency symbols: `"$1,234.56"`
/// - Negative numbers represented with a leading minus or parentheses: `"-123.45"`, `"(123.45)"`
/// - Null values, which are deserialized as `Decimal::zero()`.
///
/// It intelligently determines the decimal and thousands separators based on their position.
pub fn deserialize_flexible_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    struct FlexibleDecimalVisitor;

    impl<'de> de::Visitor<'de> for FlexibleDecimalVisitor {
        type Value = Decimal;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or number representing a decimal")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Decimal::from(value))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Decimal::from(value))
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Decimal::from_f64(value)
                .ok_or_else(|| E::custom(format!("invalid float value: {}", value)))
        }

        #[allow(
            clippy::else_if_without_else,
            clippy::string_slice,
            clippy::arithmetic_side_effects,
            reason = "Character filtering logic with safe string operations - indices checked before use"
        )]
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut s = v.trim();
            let is_negative = (s.starts_with('(') && s.ends_with(')')) || s.starts_with('-');
            if s.starts_with('(') && s.ends_with(')') {
                s = &s[1..s.len() - 1];
            }

            // Determine decimal and thousands separators
            let last_dot = s.rfind('.');
            let last_comma = s.rfind(',');

            let mut final_str = String::with_capacity(s.len());

            match (last_dot, last_comma) {
                (Some(dot_pos), Some(comma_pos)) if dot_pos > comma_pos => {
                    for c in s.chars() {
                        if c.is_ascii_digit() {
                            final_str.push(c);
                        } else if c == '.' {
                            final_str.push('.');
                        }
                    }
                }
                (Some(dot_pos), Some(comma_pos)) if comma_pos > dot_pos => {
                    for c in s.chars() {
                        if c.is_ascii_digit() {
                            final_str.push(c);
                        } else if c == ',' {
                            final_str.push('.'); // Convert decimal comma to dot
                        }
                    }
                }
                (None, Some(_)) => {
                    if s.matches(',').count() > 1 || (s.rfind(',').unwrap_or(0) < s.len() - 3) {
                        // Treat all commas as thousands separators
                        for c in s.chars() {
                            if c.is_ascii_digit() {
                                final_str.push(c);
                            }
                        }
                    } else {
                        // Treat the single comma as a decimal separator
                        for c in s.chars() {
                            if c.is_ascii_digit() {
                                final_str.push(c);
                            } else if c == ',' {
                                final_str.push('.');
                            }
                        }
                    }
                }
                (Some(_), None) => {
                    if s.matches('.').count() > 1 || (s.rfind('.').unwrap_or(0) < s.len() - 3) {
                        // Treat all dots as thousands separators
                        for c in s.chars() {
                            if c.is_ascii_digit() {
                                final_str.push(c);
                            }
                        }
                    } else {
                        // Treat the single dot as a decimal separator
                        for c in s.chars() {
                            if c.is_ascii_digit() {
                                final_str.push(c);
                            } else if c == '.' {
                                final_str.push('.');
                            }
                        }
                    }
                }
                (None, None) => {
                    for c in s.chars() {
                        if c.is_ascii_digit() {
                            final_str.push(c);
                        }
                    }
                }
                (Some(_), Some(_)) => unreachable!(),
            }

            // Prepend minus sign if negative
            if is_negative {
                final_str.insert(0, '-');
            }

            // Handle cases like ".50" or ",50" which become ".50"
            if final_str.starts_with('.') {
                final_str.insert(0, '0');
            }

            Decimal::from_str(&final_str).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(FlexibleDecimalVisitor)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "Test code with known-good conversions")]
mod tests {
    use rust_decimal::prelude::FromPrimitive as _;
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct TestBalance {
        #[serde(deserialize_with = "deserialize_flexible_decimal")]
        balance: Decimal,
    }

    #[track_caller]
    fn test_parsing(json_str: &str, expected: Decimal) {
        let result: TestBalance = serde_json::from_str(json_str).unwrap();
        assert_eq!(result.balance, expected);
    }

    #[test]
    fn test_flexible_decimal_parsing() {
        // Simple integer
        test_parsing(r#"{"balance": "1000"}"#, Decimal::from(1000_u64));
        // With currency symbol
        test_parsing(r#"{"balance": "$1000"}"#, Decimal::from(1000_u64));
        // With currency symbol and decimals
        test_parsing(
            r#"{"balance": "$1000.00"}"#,
            Decimal::from_f64(1000.00).unwrap(),
        );
        // With comma as thousands separator
        test_parsing(
            r#"{"balance": "100,000.00"}"#,
            Decimal::from_f64(100_000.00).unwrap(),
        );
        // With dot as thousands separator and comma as decimal
        test_parsing(
            r#"{"balance": "100.000,00"}"#,
            Decimal::from_f64(100_000.00).unwrap(),
        );
        // Multiple thousands separators
        test_parsing(
            r#"{"balance": "1,234,567.89"}"#,
            Decimal::from_f64(1_234_567.89).unwrap(),
        );
        test_parsing(
            r#"{"balance": "1.234.567,89"}"#,
            Decimal::from_f64(1_234_567.89).unwrap(),
        );
        // Different currency symbols
        test_parsing(
            r#"{"balance": "€1.234,56"}"#,
            Decimal::from_f64(1234.56).unwrap(),
        );
        test_parsing(
            r#"{"balance": "£1,234.56"}"#,
            Decimal::from_f64(1234.56).unwrap(),
        );
        // With whitespace
        test_parsing(
            r#"{"balance": "  $ 5,000.50  "}"#,
            Decimal::from_f64(5000.50).unwrap(),
        );
        // Negative values
        test_parsing(
            r#"{"balance": "-123.45"}"#,
            Decimal::from_f64(-123.45).unwrap(),
        );
        test_parsing(
            r#"{"balance": "-$1,234.56"}"#,
            Decimal::from_f64(-1234.56).unwrap(),
        );
        test_parsing(
            r#"{"balance": "($1,234.56)"}"#,
            Decimal::from_f64(-1234.56).unwrap(),
        );
        // No fractional part
        test_parsing(r#"{"balance": "1,000"}"#, Decimal::from(1000_u64));
        test_parsing(r#"{"balance": "1.000"}"#, Decimal::from(1000_u64));
        // Just decimals
        test_parsing(r#"{"balance": ".50"}"#, Decimal::from_f64(0.50).unwrap());
        test_parsing(r#"{"balance": "0.50"}"#, Decimal::from_f64(0.50).unwrap());
        test_parsing(r#"{"balance": ",50"}"#, Decimal::from_f64(0.50).unwrap());
        test_parsing(r#"{"balance": "0,50"}"#, Decimal::from_f64(0.50).unwrap());
    }
}
