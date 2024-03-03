use core::fmt;
use serde::{de, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Nothing;

// Based on chrono's serde implementation (https://github.com/chronotope/chrono/blob/0aa46ddbf021df6aae12da729e0df6f486947768/src/datetime/serde.rs)

// TODO: this snippet was taken was chrono's main branch and is unrelased at the moment

/// Create a custom `de::Error` with `SerdeError::InvalidTimestamp`.
pub(crate) fn invalid_ts<E, T>(value: T) -> E
where
    E: de::Error,
    T: fmt::Display,
{
    E::custom(SerdeError::InvalidTimestamp(value))
}

enum SerdeError<T: fmt::Display> {
    InvalidTimestamp(T),
}

impl<T: fmt::Display> fmt::Display for SerdeError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SerdeError::InvalidTimestamp(ts) => {
                write!(f, "value is not a legal timestamp: {}", ts)
            }
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct IsoDateVisitor;

/// Ser/de to/from timestamps in microseconds
///
/// Intended for use with `serde`'s `with` attribute.
///
/// # Example:
///
/// ```rust
/// # use chrono::{DateTime, Utc, NaiveDate};
/// # use serde::{Deserialize, Serialize};
/// use supermarket::serde::iso_date;
/// #[derive(Deserialize, Serialize)]
/// struct S {
///     #[serde(with = "iso_date")]
///     date: DateTime<Utc>,
/// }
///
/// let date = NaiveDate::from_ymd_opt(2018, 5, 17)
///     .unwrap()
///     .and_hms_micro_opt(0, 0, 0, 0)
///     .unwrap()
///     .and_local_timezone(Utc)
///     .unwrap();
/// let my_s = S { date: date.clone() };
///
/// let as_string = serde_json::to_string(&my_s)?;
/// assert_eq!(as_string, r#"{"date":"2018-05-17"}"#);
/// let my_s: S = serde_json::from_str(&as_string)?;
/// assert_eq!(my_s.date, date);
/// # Ok::<(), serde_json::Error>(())
/// ```
pub mod iso_date {
    use chrono::prelude::{DateTime, Utc};
    use chrono::NaiveDate;
    // use chrono::serde::invalid_ts;
    use core::fmt;
    use serde::{de, ser};

    use super::invalid_ts;
    use super::IsoDateVisitor;

    /// Serialize a UTC datetime into an ISO 8601 date string
    ///
    /// Intended for use with `serde`s `serialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{DateTime, Utc, NaiveDate};
    /// # use serde::Serialize;
    /// use supermarket::serde::iso_date::serialize as to_iso_date;
    /// #[derive(Serialize)]
    /// struct S {
    ///     #[serde(serialize_with = "to_iso_date")]
    ///     date: DateTime<Utc>,
    /// }
    ///
    /// let my_s = S {
    ///     date: NaiveDate::from_ymd_opt(2018, 5, 17)
    ///         .unwrap()
    ///         .and_hms_micro_opt(02, 04, 59, 918355)
    ///         .unwrap()
    ///         .and_local_timezone(Utc)
    ///         .unwrap(),
    /// };
    /// let as_string = serde_json::to_string(&my_s)?;
    /// assert_eq!(as_string, r#"{"date":"2018-05-17"}"#);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", dt.format("%F")))
    }

    /// Deserialize a `DateTime` from an ISO 8601 date string
    ///
    /// Intended for use with `serde`s `deserialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{DateTime, TimeZone, Utc};
    /// # use serde::Deserialize;
    /// use supermarket::serde::iso_date::deserialize as from_iso_date;
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// struct S {
    ///     #[serde(deserialize_with = "from_iso_date")]
    ///     date: DateTime<Utc>,
    /// }
    ///
    /// let my_s: S = serde_json::from_str(r#"{"date":"2018-05-17"}"#)?;
    /// assert_eq!(my_s, S { date: Utc.with_ymd_and_hms(2018, 5, 17, 0, 0, 0).unwrap() });
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(IsoDateVisitor)
    }

    impl<'de> de::Visitor<'de> for IsoDateVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an ISO 8601 date string")
        }

        /// Deserialize an ISO 8601 date string
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            NaiveDate::parse_from_str(value, "%F")
                .map_err(|_| invalid_ts(value))?
                .and_hms_micro_opt(0, 0, 0, 0)
                .ok_or_else(|| invalid_ts(value))?
                .and_local_timezone(Utc)
                .single()
                .ok_or_else(|| invalid_ts(value))

            // DateTime::parse_from_str(value, "%F")
            //     .map(|d| d.with_timezone(&Utc))
            //     .map_err(|_| invalid_ts(value))
        }
    }
}

/// Ser/de to/from optional ISO 8601 date string
///
/// Intended for use with `serde`'s `with` attribute.
///
/// # Example:
///
/// ```rust
/// # use chrono::{DateTime, Utc, NaiveDate};
/// # use serde::{Deserialize, Serialize};
/// use supermarket::serde::iso_date_option;
/// #[derive(Deserialize, Serialize)]
/// struct S {
///     #[serde(with = "iso_date_option")]
///     date: Option<DateTime<Utc>>,
/// }
///
/// let date = Some(
///     NaiveDate::from_ymd_opt(2018, 5, 17)
///         .unwrap()
///         .and_hms_micro_opt(0, 0, 0, 0)
///         .unwrap()
///         .and_local_timezone(Utc)
///         .unwrap(),
/// );
/// let my_s = S { date: date.clone() };
///
/// let as_string = serde_json::to_string(&my_s)?;
/// assert_eq!(as_string, r#"{"date":"2018-05-17"}"#);
/// let my_s: S = serde_json::from_str(&as_string)?;
/// assert_eq!(my_s.date, date);
/// # Ok::<(), serde_json::Error>(())
/// ```
pub mod iso_date_option {
    use chrono::prelude::{DateTime, Utc};
    use core::fmt;
    use serde::{de, ser};

    use super::IsoDateVisitor;

    /// Serialize a UTC datetime into an ISO 8601 date string or none
    ///
    /// Intended for use with `serde`s `serialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{DateTime, Utc, NaiveDate};
    /// # use serde::Serialize;
    /// use supermarket::serde::iso_date_option::serialize as to_iso_date_option;
    /// #[derive(Serialize)]
    /// struct S {
    ///     #[serde(serialize_with = "to_iso_date_option")]
    ///     date: Option<DateTime<Utc>>,
    /// }
    ///
    /// let my_s = S {
    ///     date: Some(
    ///         NaiveDate::from_ymd_opt(2018, 5, 17)
    ///             .unwrap()
    ///             .and_hms_micro_opt(0, 0, 0, 0)
    ///             .unwrap()
    ///             .and_local_timezone(Utc)
    ///             .unwrap(),
    ///     ),
    /// };
    /// let as_string = serde_json::to_string(&my_s)?;
    /// assert_eq!(as_string, r#"{"date":"2018-05-17"}"#);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn serialize<S>(opt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *opt {
            Some(ref dt) => serializer.serialize_some(&format!("{}", dt.format("%F"))),
            None => serializer.serialize_none(),
        }
    }

    /// Deserialize a `DateTime` from an ISO 8601 date string or none
    ///
    /// Intended for use with `serde`s `deserialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{DateTime, TimeZone, Utc};
    /// # use serde::Deserialize;
    /// use supermarket::serde::iso_date_option::deserialize as from_iso_date_option;
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// struct S {
    ///     #[serde(deserialize_with = "from_iso_date_option")]
    ///     date: Option<DateTime<Utc>>,
    /// }
    ///
    /// let my_s: S = serde_json::from_str(r#"{"date":"2018-05-17"}"#)?;
    /// assert_eq!(my_s, S { date: Utc.with_ymd_and_hms(2018, 5, 17, 0, 0, 0).single() });
    ///
    /// let my_s: S = serde_json::from_str(r#"{"date":null}"#)?;
    /// assert_eq!(my_s, S { date: None });
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn deserialize<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_option(OptionIsoDateVisitor)
    }

    struct OptionIsoDateVisitor;

    impl<'de> de::Visitor<'de> for OptionIsoDateVisitor {
        type Value = Option<DateTime<Utc>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an ISO 8601 date or none")
        }

        /// Deserialize an ISO 8601 date string
        fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            d.deserialize_str(IsoDateVisitor).map(Some)
        }

        /// Deserialize an ISO 8601 date string
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        /// Deserialize an ISO 8601 date string
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::{DateTime, Utc};

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct DateTimeTest {
        #[serde(with = "iso_date_option")]
        pub iso_date: Option<DateTime<Utc>>,
    }

    #[test]
    fn iso_date() {}
}
