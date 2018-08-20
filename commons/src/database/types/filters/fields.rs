use serde::{Deserialize, Serialize};

use crate::database::types::{
    APIFilterField, APIFilterFieldFunction, APIFilterFieldFunctionKind, DBDataType,
};
use crate::server::requests::PaginatedDocumentField;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: PaginatedDocumentField")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum DBFilterField<T: PaginatedDocumentField> {
    #[serde(rename = "F")]
    Field(T),
    #[serde(rename = "V")]
    Constant(DBDataType),
    #[serde(rename = "F")]
    Function(DBFilterFieldFunction<T>),
}

impl<T: PaginatedDocumentField> From<DBFilterField<T>> for APIFilterField<T> {
    fn from(value: DBFilterField<T>) -> Self {
        match value {
            DBFilterField::Field(v) => APIFilterField::Field(v),
            DBFilterField::Constant(v) => APIFilterField::Constant(v),
            DBFilterField::Function(v) => APIFilterField::Function(v.into()),
        }
    }
}

impl<T: PaginatedDocumentField> From<APIFilterField<T>> for DBFilterField<T> {
    fn from(value: APIFilterField<T>) -> Self {
        match value {
            APIFilterField::Field(v) => DBFilterField::Field(v),
            APIFilterField::Constant(v) => DBFilterField::Constant(v),
            APIFilterField::Function(v) => DBFilterField::Function(v.into()),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(bound = "T: PaginatedDocumentField")]
pub struct DBFilterFieldFunction<T: PaginatedDocumentField> {
    #[serde(rename = "N")]
    pub name: FilterFieldFunctionKind,
    #[serde(rename = "A")]
    pub args: Vec<DBFilterField<T>>,
}

impl<T: PaginatedDocumentField> From<DBFilterFieldFunction<T>> for APIFilterFieldFunction<T> {
    fn from(value: DBFilterFieldFunction<T>) -> Self {
        Self {
            name: value.name.into(),
            args: value.args.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl<T: PaginatedDocumentField> From<APIFilterFieldFunction<T>> for DBFilterFieldFunction<T> {
    fn from(value: APIFilterFieldFunction<T>) -> Self {
        Self {
            name: value.name.into(),
            args: value.args.into_iter().map(|v| v.into()).collect(),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterFieldFunctionKind {
    // GENERAL
    /// For arrays, strings and objects.
    #[serde(rename = "L")]
    Length,

    // ARRAYS
    #[serde(rename = "N")]
    Nth,
    #[serde(rename = "I")]
    IndexOf,
    #[serde(rename = "C")]
    ListContains,

    // OBJECTS
    #[serde(rename = "K")]
    HasKey,

    // LOGIC
    #[serde(rename = "O")]
    Not,

    // STRING
    #[serde(rename = "T")]
    Trim,
    #[serde(rename = "L")]
    Lowercase,
    #[serde(rename = "U")]
    Uppercase,
    #[serde(rename = "S")]
    StartsWith,
    #[serde(rename = "E")]
    EndsWith,
    #[serde(rename = "K")]
    Like,
    #[serde(rename = "X")]
    Regex,
    #[serde(rename = "A")]
    CharAt,
    #[serde(rename = "B")]
    Contains,

    // TYPE CHECKING
    #[serde(rename = "Y")]
    Typename,
}

impl From<FilterFieldFunctionKind> for APIFilterFieldFunctionKind {
    fn from(value: FilterFieldFunctionKind) -> Self {
        match value {
            FilterFieldFunctionKind::Length => APIFilterFieldFunctionKind::Length,
            FilterFieldFunctionKind::Nth => APIFilterFieldFunctionKind::Nth,
            FilterFieldFunctionKind::IndexOf => APIFilterFieldFunctionKind::IndexOf,
            FilterFieldFunctionKind::ListContains => APIFilterFieldFunctionKind::ListContains,
            FilterFieldFunctionKind::HasKey => APIFilterFieldFunctionKind::HasKey,
            FilterFieldFunctionKind::Not => APIFilterFieldFunctionKind::Not,
            FilterFieldFunctionKind::Trim => APIFilterFieldFunctionKind::Trim,
            FilterFieldFunctionKind::Lowercase => APIFilterFieldFunctionKind::Lowercase,
            FilterFieldFunctionKind::Uppercase => APIFilterFieldFunctionKind::Uppercase,
            FilterFieldFunctionKind::StartsWith => APIFilterFieldFunctionKind::StartsWith,
            FilterFieldFunctionKind::EndsWith => APIFilterFieldFunctionKind::EndsWith,
            FilterFieldFunctionKind::Like => APIFilterFieldFunctionKind::Like,
            FilterFieldFunctionKind::Regex => APIFilterFieldFunctionKind::Regex,
            FilterFieldFunctionKind::CharAt => APIFilterFieldFunctionKind::CharAt,
            FilterFieldFunctionKind::Contains => APIFilterFieldFunctionKind::Contains,
            FilterFieldFunctionKind::Typename => APIFilterFieldFunctionKind::Typename,
        }
    }
}

impl From<APIFilterFieldFunctionKind> for FilterFieldFunctionKind {
    fn from(value: APIFilterFieldFunctionKind) -> Self {
        match value {
            APIFilterFieldFunctionKind::Length => FilterFieldFunctionKind::Length,
            APIFilterFieldFunctionKind::Nth => FilterFieldFunctionKind::Nth,
            APIFilterFieldFunctionKind::IndexOf => FilterFieldFunctionKind::IndexOf,
            APIFilterFieldFunctionKind::ListContains => FilterFieldFunctionKind::ListContains,
            APIFilterFieldFunctionKind::HasKey => FilterFieldFunctionKind::HasKey,
            APIFilterFieldFunctionKind::Not => FilterFieldFunctionKind::Not,
            APIFilterFieldFunctionKind::Trim => FilterFieldFunctionKind::Trim,
            APIFilterFieldFunctionKind::Lowercase => FilterFieldFunctionKind::Lowercase,
            APIFilterFieldFunctionKind::Uppercase => FilterFieldFunctionKind::Uppercase,
            APIFilterFieldFunctionKind::StartsWith => FilterFieldFunctionKind::StartsWith,
            APIFilterFieldFunctionKind::EndsWith => FilterFieldFunctionKind::EndsWith,
            APIFilterFieldFunctionKind::Like => FilterFieldFunctionKind::Like,
            APIFilterFieldFunctionKind::Regex => FilterFieldFunctionKind::Regex,
            APIFilterFieldFunctionKind::CharAt => FilterFieldFunctionKind::CharAt,
            APIFilterFieldFunctionKind::Contains => FilterFieldFunctionKind::Contains,
            APIFilterFieldFunctionKind::Typename => FilterFieldFunctionKind::Typename,
        }
    }
}
