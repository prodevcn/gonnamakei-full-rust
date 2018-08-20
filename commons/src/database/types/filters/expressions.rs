use serde::{Deserialize, Serialize};

use crate::database::types::{APIFilterExpression, DBFilterField};
use crate::server::requests::PaginatedDocumentField;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: PaginatedDocumentField")]
pub struct DBFilterExpression<T: PaginatedDocumentField> {
    #[serde(rename = "L")]
    pub left: DBFilterField<T>,
    #[serde(rename = "O")]
    pub operator: FilterOperator,
    #[serde(rename = "R")]
    pub right: DBFilterField<T>,
}

impl<T: PaginatedDocumentField> From<DBFilterExpression<T>> for APIFilterExpression<T> {
    fn from(value: DBFilterExpression<T>) -> Self {
        Self {
            left: value.left.into(),
            operator: value.operator,
            right: value.right.into(),
        }
    }
}

impl<T: PaginatedDocumentField> From<APIFilterExpression<T>> for DBFilterExpression<T> {
    fn from(value: APIFilterExpression<T>) -> Self {
        Self {
            left: value.left.into(),
            operator: value.operator,
            right: value.right.into(),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum FilterOperator {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterOrEqualThan,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessOrEqualThan,
    #[serde(rename = "in")]
    InArray,
    #[serde(rename = "!in")]
    NotInArray,
    #[serde(rename = "like")]
    LikePattern,
    #[serde(rename = "!like")]
    NotLikePattern,
    #[serde(rename = "regex")]
    LikeRegex,
    #[serde(rename = "!regex")]
    NotLikeRegex,
    // ARRAY
    #[serde(rename = "[all]")]
    AllArray(Box<FilterOperator>),
    #[serde(rename = "[any]")]
    AnyArray(Box<FilterOperator>),
    #[serde(rename = "[none]")]
    NoneArray(Box<FilterOperator>),
}
