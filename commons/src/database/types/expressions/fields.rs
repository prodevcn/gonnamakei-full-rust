use serde::{Deserialize, Serialize};

use crate::database::types::{DBDataType, DBExpression};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum DBExpressionField<T> {
    #[serde(rename = "F")]
    Field(T),
    #[serde(rename = "V")]
    Constant(DBDataType),
    #[serde(rename = "M")]
    Function(DBExpressionFunction<T>),
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct DBExpressionFunction<T> {
    #[serde(rename = "N")]
    pub name: FunctionExpressionKind,
    #[serde(rename = "A")]
    pub args: Vec<DBExpression<T>>,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FunctionExpressionKind {
    // GENERAL ----------------------------------------------------------------
    /// The number of elements in a List.
    /// LENGTH(List) -> Natural
    ///
    /// The number of chars in a String.
    /// LENGTH(String) -> Natural
    ///
    /// The number of keys in an Object.
    /// LENGTH(Object) -> Natural
    #[serde(rename = "Le")]
    Length,

    /// Whether a List contains an element or not.
    /// CONTAINS(List, element: Any) -> Bool
    ///
    /// Whether a String contains a substring or not.
    /// CONTAINS(String, substring: String) -> Bool
    ///
    /// Whether an Object contains a key or not.
    /// CONTAINS(Object, key: String) -> Bool
    #[serde(rename = "Co")]
    Contains,

    /// Gets the nth-element of a List.
    /// NTH(List, index: Natural) -> Any
    ///
    /// Gets the nth-char of a String.
    /// NTH(String, index: Natural) -> String
    #[serde(rename = "Nt")]
    Nth,

    /// Gets the nth-element of a List starting from the end.
    /// NTH_LAST(List, index: Natural) -> Any
    ///
    /// Gets the nth-char of a String starting from the end.
    /// NTH_LAST(String, index: Natural) -> String
    #[serde(rename = "NL")]
    NthLast,

    /// Gets the position of an element inside a List.
    /// INDEX(List, element: Any) -> Integer
    ///
    /// Gets the position of a substring inside a String.
    /// INDEX(String, substring: String) -> Integer
    #[serde(rename = "In")]
    Index,

    /// Gets the position of an element inside a List starting from the end.
    /// LAST_INDEX(List, element: Any) -> Integer
    ///
    /// Gets the position of a substring inside a String starting from the end.
    /// LAST_INDEX(String, substring: String) -> Integer
    #[serde(rename = "LI")]
    LastIndex,

    // LOGIC ------------------------------------------------------------------
    /// Evaluates a condition and returns the specified value.
    /// IF(condition: Bool, then: Any, else: Any) -> Any
    #[serde(rename = "If")]
    If,

    // NUMBER -----------------------------------------------------------------
    /// Gets the absolute value of the number.
    /// ABS(Number) -> Number
    #[serde(rename = "Ab")]
    Abs,

    /// Rounds up the number.
    /// CEIL(Number) -> Number
    #[serde(rename = "Ce")]
    Ceil,

    /// Rounds down the number.
    /// FLOOR(Number) -> Number
    #[serde(rename = "Fl")]
    Floor,

    /// Rounds the number.
    /// ROUND(Number) -> Number
    #[serde(rename = "Ro")]
    Round,

    /// Get the maximum number of both.
    /// MAX(...Number) -> Number
    #[serde(rename = "Ma")]
    Max,

    /// Get the minimum number of both.
    /// MIN(...Number) -> Number
    #[serde(rename = "Mi")]
    Min,

    // STRING -----------------------------------------------------------------
    /// Removes the whitespaces in both sides of a String.
    /// TRIM(String) -> String
    #[serde(rename = "Tr")]
    Trim,

    /// Changes a String to lowercase.
    /// LOWERCASE(String) -> String
    #[serde(rename = "Lo")]
    Lowercase,

    /// Changes a String to uppercase.
    /// UPPERCASE(String) -> String
    #[serde(rename = "Up")]
    Uppercase,

    /// Whether a String starts with a substring or not.
    /// STARTS_WITH(String, substring: String) -> Bool
    #[serde(rename = "SW")]
    StartsWith,

    /// Whether a String ends with a substring or not.
    /// ENDS_WITH(String, substring: String) -> Bool
    #[serde(rename = "EW")]
    EndsWith,

    /// Gets a substring of a String.
    /// SUBSTRING(String, from: Integer, length: Natural) -> String
    #[serde(rename = "Su")]
    Substring,

    // LISTS ------------------------------------------------------------------
    /// The number of unique elements in a List.
    /// COUNT_UNIQUE(List) -> Natural
    #[serde(rename = "CU")]
    CountUnique,

    /// Gets a sub-list of a List.
    /// SLICE(List, from: Integer, length: Natural) -> List
    #[serde(rename = "Sl")]
    Slice,

    // OBJECTS ----------------------------------------------------------------
    /// Gets a key of an object.
    /// GET(Object, key: String) -> Any
    #[serde(rename = "Ge")]
    Get,

    // DATES ------------------------------------------------------------------
    /// Creates a DateTime with the current timestamp.
    #[serde(rename = "DN")]
    Now,

    /// Creates a DateTime from the number of seconds since UTC.
    /// DATE(seconds: Integer) -> Date
    ///
    /// Creates a DateTime from the parts.
    /// DATE(year: Integer, month: Integer, day: Integer) -> Date
    /// DATE(year: Integer, month: Integer, day: Integer, hour: Integer, minutes: Integer, seconds: Integer) -> Date
    #[serde(rename = "DT")]
    DateTime,

    /// Creates a DateTimeMillis from the number of milliseconds since UTC.
    /// DATE(milliseconds: Integer) -> Date
    ///
    /// Creates a DateTimeMillis from the parts.
    /// DATE(year: Integer, month: Integer, day: Integer) -> Date
    /// DATE(year: Integer, month: Integer, day: Integer, hour: Integer, minutes: Integer, seconds: Integer, milliseconds: Integer) -> Date
    #[serde(rename = "DTM")]
    DateTimeMillis,

    /// Creates a Date from the number of milliseconds since UTC.
    /// DATE(days: Integer) -> Date
    ///
    /// Creates a Date from the parts.
    /// DATE(year: Integer, month: Integer, day: Integer) -> Date
    #[serde(rename = "Da")]
    Date,

    /// Gets the milliseconds of the day.
    /// MILLISECONDS(Date) -> Natural
    #[serde(rename = "Ms")]
    Milliseconds,

    /// Gets the seconds of the day.
    /// SECONDS(Date) -> Natural
    #[serde(rename = "S")]
    Seconds,

    /// Gets the minutes of the day.
    /// MINUTES(Date) -> Natural
    #[serde(rename = "M")]
    Minutes,

    /// Gets the hour of the day.
    /// HOUR(Date) -> Natural
    #[serde(rename = "H")]
    Hour,

    /// Gets the day of the month.
    /// MONTH_DAY(Date) -> Natural
    #[serde(rename = "D")]
    Day,

    /// Gets the week of the year.
    /// WEEK(Date) -> Natural
    #[serde(rename = "W")]
    Week,

    /// Gets the week day of the date.
    /// WEEK_DAY(Date) -> Natural
    #[serde(rename = "WD")]
    WeekDay,

    /// Gets the month of the date.
    /// MONTH(Date) -> Natural
    #[serde(rename = "Mo")]
    Month,

    /// Gets the year of the date.
    /// YEAR(Date) -> Integer
    #[serde(rename = "Y")]
    Year,

    // DAY TIME ---------------------------------------------------------------
    /// Creates a Duration.
    /// DISTANCE(seconds: Integer) -> Duration
    #[serde(rename = "DyT")]
    DayTime,

    // TIME DURATION ----------------------------------------------------------
    /// Creates a Duration.
    /// DISTANCE(milliseconds: Integer) -> Duration
    #[serde(rename = "Du")]
    Duration,

    /// Gets the equivalent in milliseconds of the duration.
    /// IN_MILLISECONDS(Duration) -> Integer
    ///
    /// Gets the equivalent in milliseconds of the day time.
    /// IN_MILLISECONDS(DayTime) -> Integer
    #[serde(rename = "IMs")]
    InMilliseconds,

    /// Gets the equivalent in seconds of the duration.
    /// IN_SECONDS(Duration) -> Real
    ///
    /// Gets the equivalent in seconds of the day time.
    /// IN_SECONDS(DayTime) -> Real
    #[serde(rename = "IS")]
    InSeconds,

    /// Gets the equivalent in minutes of the duration.
    /// IN_MINUTES(Duration) -> Real
    ///
    /// Gets the equivalent in seconds of the day time.
    /// IN_SECONDS(DayTime) -> Real
    #[serde(rename = "IM")]
    InMinutes,

    /// Gets the equivalent in hours of the duration.
    /// IN_HOURS(Duration) -> Real
    ///
    /// Gets the equivalent in seconds of the day time.
    /// IN_SECONDS(DayTime) -> Real
    #[serde(rename = "IH")]
    InHours,

    /// Gets the equivalent in days of the duration.
    /// IN_DAYS(Duration) -> Real
    #[serde(rename = "ID")]
    InDays,

    /// Gets the equivalent in months of the duration.
    /// Month = 31 days
    /// IN_MONTHS(Duration) -> Real
    #[serde(rename = "IMo")]
    InMonths,

    /// Gets the equivalent in years of the duration.
    /// Year = 365 days
    /// IN_YEARS(Duration) -> Real
    #[serde(rename = "IY")]
    InYears,

    // TYPE CHECKING ----------------------------------------------------------
    /// Gets the typename of the value.
    /// TYPENAME(Any) -> String
    #[serde(rename = "Ty")]
    Typename,

    /// Casts a value to another type.
    /// AS(value, "type") -> type
    #[serde(rename = "As")]
    As,

    // CONTEXT MODIFICATION ---------------------------------------------------
    /// Gets a variable from the context.
    /// VAR("name") -> Any
    #[serde(rename = "Va")]
    Var,

    /// Sets a variable in the context. Returns value.
    /// SET_VAR("name", Any) -> Any
    #[serde(rename = "SV")]
    SetVar,
}
