use std::fmt;
use std::ops::Deref;

use chrono::{Datelike, LocalResult, TimeZone, Timelike, Utc};
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::database::traits::{DBNormalize, DBNormalizeResult};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DateTime(pub chrono::DateTime<Utc>);

impl DateTime {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(date: chrono::DateTime<Utc>) -> Self {
        DateTime(date.date().and_hms_milli(
            date.hour(),
            date.minute(),
            date.second(),
            date.timestamp_subsec_millis(),
        ))
    }

    pub fn now() -> Self {
        Self::new(Utc::now())
    }

    pub fn current_minute() -> Self {
        let now = Utc::now();
        DateTime(now.date().and_hms(now.hour(), now.minute(), 0))
    }

    pub fn current_hour() -> Self {
        let now = Utc::now();
        DateTime(now.date().and_hms(now.hour(), 0, 0))
    }

    pub fn max_datetime() -> Self {
        Self::new(chrono::MAX_DATETIME)
    }

    // GETTERS ----------------------------------------------------------------

    /// Checks this datetime against now as if it is an expiration.
    pub fn is_expired(&self) -> bool {
        let now = DateTime::default();
        self.0 <= now.0
    }

    /// Checks this datetime against now as if it is an expiration.
    /// The threshold allows to return true if the difference is less or equal than the specified seconds.
    pub fn is_expired_with_threshold(&self, threshold: u64) -> bool {
        self.after_seconds(threshold).is_expired()
    }

    // METHODS ----------------------------------------------------------------

    /// Creates a new DateTime from the current one after `duration` seconds.
    pub fn after_seconds(&self, duration: u64) -> DateTime {
        self.after_seconds_checked(duration as i64).unwrap()
    }

    /// Creates a new DateTime from the current one after `duration` seconds.
    pub fn after_seconds_checked(&self, duration: i64) -> Option<DateTime> {
        self.0
            .checked_add_signed(chrono::Duration::seconds(duration))
            .map(DateTime)
    }

    /// Creates a new DateTime from the current one before `duration` seconds.
    pub fn before_seconds(&self, duration: u64) -> DateTime {
        DateTime(self.0 - chrono::Duration::seconds(duration as i64))
    }

    /// Creates a new DateTime from the current one after `duration` days.
    pub fn after_days(&self, duration: u64) -> DateTime {
        self.after_days_checked(duration as i64).unwrap()
    }

    /// Creates a new DateTime from the current one after `duration` days.
    pub fn after_days_checked(&self, duration: i64) -> Option<DateTime> {
        self.0
            .checked_add_signed(chrono::Duration::days(duration))
            .map(DateTime)
    }

    /// Creates a new DateTime from the current one after `duration` months.
    pub fn after_months_checked(&self, duration: u32) -> Option<DateTime> {
        let mut final_months = match (self.0.year() as i64).checked_mul(12) {
            Some(v) => v,
            None => return None,
        };
        final_months = match final_months.checked_add(self.0.month0() as i64) {
            Some(v) => v,
            None => return None,
        };
        final_months = match final_months.checked_add(duration as i64) {
            Some(v) => v,
            None => return None,
        };

        let year = final_months / 12;
        let month = final_months % 12;

        match Utc
            .ymd_opt(year as i32, month as u32 + 1, self.0.day())
            .map(|v| {
                v.and_hms_milli_opt(
                    self.0.hour(),
                    self.0.minute(),
                    self.0.second(),
                    self.0.timestamp_subsec_millis(),
                )
                .map(DateTime)
            }) {
            LocalResult::Single(v) => v,
            _ => None,
        }
    }

    /// Creates a new DateTime from the current one after `duration` years.
    pub fn after_years_checked(&self, duration: i32) -> Option<DateTime> {
        let mut years = self.0.year();
        years = match years.checked_add(duration) {
            Some(v) => v,
            None => return None,
        };

        match Utc.ymd_opt(years, self.0.month(), self.0.day()).map(|v| {
            v.and_hms_milli_opt(
                self.0.hour(),
                self.0.minute(),
                self.0.second(),
                self.0.timestamp_subsec_millis(),
            )
            .map(DateTime)
        }) {
            LocalResult::Single(v) => v,
            _ => None,
        }
    }

    /// Creates a new DateTime from the current one before `duration` months.
    pub fn before_months(&self, duration: u64) -> DateTime {
        let mut final_months = self.0.year() * 12;
        final_months += self.0.month0() as i32;
        final_months -= duration as i32;

        let year = final_months / 12;
        let month = final_months % 12;

        DateTime(Utc.ymd(year, month as u32 + 1, self.0.day()).and_hms_milli(
            self.0.hour(),
            self.0.minute(),
            self.0.second(),
            self.0.timestamp_subsec_millis(),
        ))
    }

    pub fn min(self, other: DateTime) -> DateTime {
        DateTime(self.0.min(other.0))
    }

    pub fn max(self, other: DateTime) -> DateTime {
        DateTime(self.0.max(other.0))
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.timestamp_millis())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateTimeVisitor;
        impl<'de> Visitor<'de> for DateTimeVisitor {
            type Value = DateTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between -2^63 and 2^63")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DateTime::new(Utc::timestamp_millis(&Utc, value)))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DateTime::new(Utc::timestamp_millis(&Utc, value as i64)))
            }
        }

        deserializer.deserialize_i64(DateTimeVisitor)
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(v: chrono::DateTime<Utc>) -> Self {
        DateTime::new(v)
    }
}

impl DBNormalize for DateTime {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Date(pub chrono::Date<Utc>);

impl Date {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(date: chrono::Date<Utc>) -> Self {
        Self(date)
    }

    pub fn today() -> Self {
        Self(Utc::today())
    }

    pub fn current_month() -> Self {
        let today = Self::today();
        Self(Utc.ymd(today.year(), today.month(), 1))
    }

    pub fn zero_month() -> Self {
        Self(Utc.ymd(0, 1, 1))
    }

    // GETTERS ----------------------------------------------------------------

    /// Checks this datetime against now as if it is an expiration.
    pub fn is_expired(&self) -> bool {
        let now = Date::today();
        self.0 <= now.0
    }

    pub fn months_since_zero_month(&self) -> u32 {
        let zero_month = Self::zero_month();
        (self.0.year() as u32 * 12 + self.0.month0())
            - (zero_month.0.year() as u32 * 12 + zero_month.0.month0())
    }

    // METHODS ----------------------------------------------------------------

    pub fn before_years(&self, years: u32) -> Date {
        Date(Utc.ymd(self.0.year() - years as i32, self.0.month(), self.0.day()))
    }

    pub fn after_days(&self, duration: u64) -> Date {
        Date(self.0 + chrono::Duration::days(duration as i64))
    }

    pub fn after_months(&self, months: u32) -> Date {
        let mut final_months = self.0.year() * 12;
        final_months += self.0.month0() as i32;
        final_months += months as i32;

        let year = final_months / 12;
        let month = final_months % 12;

        Date(Utc.ymd(year, month as u32 + 1, self.0.day()))
    }

    pub fn before_months(&self, months: u32) -> Date {
        let mut final_months = self.0.year() * 12;
        final_months += self.0.month0() as i32;
        final_months -= months as i32;

        let year = final_months / 12;
        let month = final_months % 12;

        Date(Utc.ymd(year, month as u32 + 1, self.0.day()))
    }

    pub fn to_date_time(&self) -> DateTime {
        DateTime::new(self.0.and_hms(0, 0, 0))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0.num_days_from_ce())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;
        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Date;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between -2^63 and 2^63")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Date(chrono::Date::from_utc(
                    chrono::NaiveDate::from_num_days_from_ce(value as i32),
                    Utc,
                )))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Date(chrono::Date::from_utc(
                    chrono::NaiveDate::from_num_days_from_ce(value as i32),
                    Utc,
                )))
            }
        }

        deserializer.deserialize_i64(DateVisitor)
    }
}

impl Deref for Date {
    type Target = chrono::Date<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<chrono::Date<Utc>> for Date {
    fn from(v: chrono::Date<Utc>) -> Self {
        Date(v)
    }
}

impl DBNormalize for Date {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::today()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DayTime(pub chrono::NaiveTime);

impl Serialize for DayTime {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.0.num_seconds_from_midnight())
    }
}

impl<'de> Deserialize<'de> for DayTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeVisitor;
        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = DayTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between -2^63 and 2^63")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DayTime(chrono::NaiveTime::from_num_seconds_from_midnight(
                    value as u32,
                    0,
                )))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DayTime(chrono::NaiveTime::from_num_seconds_from_midnight(
                    value as u32,
                    0,
                )))
            }
        }

        deserializer.deserialize_u32(TimeVisitor)
    }
}

impl Deref for DayTime {
    type Target = chrono::NaiveTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<chrono::NaiveTime> for DayTime {
    fn from(v: chrono::NaiveTime) -> Self {
        DayTime(v)
    }
}

impl DBNormalize for DayTime {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

impl Default for DayTime {
    fn default() -> Self {
        Self(chrono::NaiveTime::from_hms(0, 0, 0))
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DurationMillis(u64);

impl Serialize for DurationMillis {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for DurationMillis {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeVisitor;
        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = DurationMillis;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between -2^63 and 2^63")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DurationMillis(value as u64))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DurationMillis(value))
            }
        }

        deserializer.deserialize_u64(TimeVisitor)
    }
}

impl Deref for DurationMillis {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for DurationMillis {
    fn from(v: u8) -> Self {
        DurationMillis(v as u64)
    }
}

impl From<u16> for DurationMillis {
    fn from(v: u16) -> Self {
        DurationMillis(v as u64)
    }
}

impl From<u32> for DurationMillis {
    fn from(v: u32) -> Self {
        DurationMillis(v as u64)
    }
}

impl From<u64> for DurationMillis {
    fn from(v: u64) -> Self {
        DurationMillis(v)
    }
}

impl DBNormalize for DurationMillis {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

impl Default for DurationMillis {
    fn default() -> Self {
        Self(0)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DBExpiration(pub chrono::DateTime<Utc>);

impl DBExpiration {
    // GETTERS ----------------------------------------------------------------

    /// Checks this datetime against now as if it is an expiration.
    pub fn is_expired(&self) -> bool {
        let now = DateTime::now();
        self.0 <= now.0
    }
}

impl Serialize for DBExpiration {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}

impl<'de> Deserialize<'de> for DBExpiration {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateTimeVisitor;
        impl<'de> Visitor<'de> for DateTimeVisitor {
            type Value = DBExpiration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between -2^63 and 2^63")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DateTime::new(Utc::timestamp(&Utc, value, 0)).into())
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DateTime::new(Utc::timestamp(&Utc, value as i64, 0)).into())
            }
        }

        deserializer.deserialize_i64(DateTimeVisitor)
    }
}

impl Deref for DBExpiration {
    type Target = chrono::DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DateTime> for DBExpiration {
    fn from(v: DateTime) -> Self {
        DBExpiration(v.0.date().and_hms(v.0.hour(), v.0.minute(), v.0.second()))
    }
}

impl From<DBExpiration> for DateTime {
    fn from(v: DBExpiration) -> Self {
        DateTime::new(v.0)
    }
}

impl DBNormalize for DBExpiration {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

impl Default for DBExpiration {
    fn default() -> Self {
        DateTime::now().into()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn datetime() {
        let date = DateTime(Utc.ymd(1970, 12, 7).and_hms_milli(5, 23, 30, 500));
        let str_date = serde_json::to_string(&date).unwrap();

        assert_eq!("29395410500", str_date);
        assert_eq!(date, serde_json::from_str(str_date.as_str()).unwrap());
    }

    #[test]
    fn date() {
        let date = Date(Utc.ymd(1970, 12, 7));
        let str_date = serde_json::to_string(&date).unwrap();

        assert_eq!("719503", str_date);
        assert_eq!(date, serde_json::from_str(str_date.as_str()).unwrap());
    }

    #[test]
    fn day_time() {
        let day_time = DayTime(chrono::NaiveTime::from_hms(2, 23, 55));
        let str_day_time = serde_json::to_string(&day_time).unwrap();

        assert_eq!("8635", str_day_time);
        assert_eq!(
            day_time,
            serde_json::from_str(str_day_time.as_str()).unwrap()
        );
    }

    #[test]
    fn time_duration() {
        let time_duration = DurationMillis(555687);
        let str_time_duration = serde_json::to_string(&time_duration).unwrap();

        assert_eq!("555687", str_time_duration);
        assert_eq!(
            time_duration,
            serde_json::from_str(str_time_duration.as_str()).unwrap()
        );
    }

    #[test]
    fn datetime_after_months() {
        let original_date = DateTime(Utc.ymd(2021, 12, 1).and_hms(0, 0, 0));
        let final_date = original_date.after_months_checked(1).unwrap();

        assert_eq!(final_date.0.year(), 2022, "The year is incorrect");
        assert_eq!(final_date.0.month(), 1, "The month is incorrect");

        let original_date = DateTime(Utc.ymd(2021, 5, 1).and_hms(0, 0, 0));
        let final_date = original_date.after_months_checked(20).unwrap();

        assert_eq!(final_date.0.year(), 2023, "The year is incorrect");
        assert_eq!(final_date.0.month(), 1, "The month is incorrect");
    }

    #[test]
    fn datetime_before_months() {
        let original_date = DateTime(Utc.ymd(2021, 1, 1).and_hms(0, 0, 0));
        let final_date = original_date.before_months(1);

        assert_eq!(final_date.0.year(), 2020, "The year is incorrect");
        assert_eq!(final_date.0.month(), 12, "The month is incorrect");

        let original_date = DateTime(Utc.ymd(2021, 5, 1).and_hms(0, 0, 0));
        let final_date = original_date.before_months(20);

        assert_eq!(final_date.0.year(), 2019, "The year is incorrect");
        assert_eq!(final_date.0.month(), 9, "The month is incorrect");
    }

    #[test]
    fn date_after_months() {
        let original_date = Date(Utc.ymd(2021, 12, 1));
        let final_date = original_date.after_months(1);

        assert_eq!(final_date.0.year(), 2022, "The year is incorrect");
        assert_eq!(final_date.0.month(), 1, "The month is incorrect");

        let original_date = Date(Utc.ymd(2021, 5, 1));
        let final_date = original_date.after_months(20);

        assert_eq!(final_date.0.year(), 2023, "The year is incorrect");
        assert_eq!(final_date.0.month(), 1, "The month is incorrect");
    }

    #[test]
    fn date_before_months() {
        let original_date = Date(Utc.ymd(2021, 1, 1));
        let final_date = original_date.before_months(1);

        assert_eq!(final_date.0.year(), 2020, "The year is incorrect");
        assert_eq!(final_date.0.month(), 12, "The month is incorrect");

        let original_date = Date(Utc.ymd(2021, 5, 1));
        let final_date = original_date.before_months(20);

        assert_eq!(final_date.0.year(), 2019, "The year is incorrect");
        assert_eq!(final_date.0.month(), 9, "The month is incorrect");
    }
}
