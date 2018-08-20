use std::collections::HashMap;

use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::database::types::{Date, DateTime, DayTime, DurationMillis};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "T", content = "V")]
pub enum DBDataType {
    // PRIMITIVES -------------------------------------------------------------
    #[serde(rename = "B")]
    Bool(bool),
    #[serde(rename = "Nu")]
    Natural(u64),
    #[serde(rename = "Ni")]
    Integer(i64),
    #[serde(rename = "Nf")]
    Real(f64),
    #[serde(rename = "S")]
    String(ArcStr),

    // COMPLEX ----------------------------------------------------------------
    #[serde(rename = "L")]
    List(Vec<DBDataType>),
    #[serde(rename = "O")]
    Object(HashMap<ArcStr, DBDataType>),

    // CUSTOM -----------------------------------------------------------------
    #[serde(rename = "DT")]
    DateTime(DateTime),
    #[serde(rename = "D")]
    Date(Date),
    #[serde(rename = "T")]
    DayTime(DayTime),
    #[serde(rename = "TD")]
    TimeDuration(DurationMillis),
}

impl DBDataType {
    // METHODS ----------------------------------------------------------------

    pub fn as_aql_param(&self) -> Value {
        match self {
            // PRIMITIVES -------------------------------------------------------------
            DBDataType::Bool(v) => serde_json::to_value(v).unwrap(),
            DBDataType::Natural(v) => serde_json::to_value(v).unwrap(),
            DBDataType::Integer(v) => serde_json::to_value(v).unwrap(),
            DBDataType::Real(v) => serde_json::to_value(v).unwrap(),
            DBDataType::String(v) => serde_json::to_value(v).unwrap(),

            // COMPLEX ----------------------------------------------------------------
            DBDataType::List(v) => serde_json::to_value(v).unwrap(),
            DBDataType::Object(v) => serde_json::to_value(v).unwrap(),

            // CUSTOM -----------------------------------------------------------------
            DBDataType::DateTime(v) => serde_json::to_value(v).unwrap(),
            DBDataType::Date(v) => serde_json::to_value(v).unwrap(),
            DBDataType::DayTime(v) => serde_json::to_value(v).unwrap(),
            DBDataType::TimeDuration(v) => serde_json::to_value(v).unwrap(),
        }
    }

    pub fn get_variant(&self) -> DBDataTypeVariant {
        match self {
            // PRIMITIVES -------------------------------------------------------------
            DBDataType::Bool(_) => DBDataTypeVariant::Bool,
            DBDataType::Natural(_) => DBDataTypeVariant::Natural,
            DBDataType::Integer(_) => DBDataTypeVariant::Integer,
            DBDataType::Real(_) => DBDataTypeVariant::Real,
            DBDataType::String(_) => DBDataTypeVariant::String,

            // COMPLEX ----------------------------------------------------------------
            DBDataType::List(_) => DBDataTypeVariant::List,
            DBDataType::Object(_) => DBDataTypeVariant::Object,

            // CUSTOM -----------------------------------------------------------------
            DBDataType::DateTime(_) => DBDataTypeVariant::DateTime,
            DBDataType::Date(_) => DBDataTypeVariant::Date,
            DBDataType::DayTime(_) => DBDataTypeVariant::DayTime,
            DBDataType::TimeDuration(_) => DBDataTypeVariant::TimeDuration,
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DBDataTypeVariant {
    // PRIMITIVES -------------------------------------------------------------
    #[serde(rename = "B")]
    Bool,
    #[serde(rename = "Nu")]
    Natural,
    #[serde(rename = "Ni")]
    Integer,
    #[serde(rename = "Nf")]
    Real,
    #[serde(rename = "S")]
    String,

    // COMPLEX ----------------------------------------------------------------
    #[serde(rename = "L")]
    List,
    #[serde(rename = "O")]
    Object,

    // CUSTOM -----------------------------------------------------------------
    #[serde(rename = "DT")]
    DateTime,
    #[serde(rename = "D")]
    Date,
    #[serde(rename = "T")]
    DayTime,
    #[serde(rename = "TD")]
    TimeDuration,
}
