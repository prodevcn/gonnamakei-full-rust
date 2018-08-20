use serde::{Deserialize, Serialize};

pub use api::*;
pub use expressions::*;
pub use fields::*;

use crate::database::traits::{DBNormalize, DBNormalizeResult};
use crate::server::requests::PaginatedDocumentField;

mod api;
mod expressions;
mod fields;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: PaginatedDocumentField")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum DBFilter<T: PaginatedDocumentField> {
    #[serde(rename = "E")]
    Expression(DBFilterExpression<T>),
    #[serde(rename = "O")]
    Or(Vec<DBFilter<T>>),
    #[serde(rename = "A")]
    And(Vec<DBFilter<T>>),
}

impl<T: PaginatedDocumentField> From<DBFilter<T>> for APIFilter<T> {
    fn from(value: DBFilter<T>) -> Self {
        match value {
            DBFilter::Expression(v) => APIFilter::Expression(v.into()),
            DBFilter::Or(v) => APIFilter::Or(v.into_iter().map(|v| v.into()).collect()),
            DBFilter::And(v) => APIFilter::And(v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl<T: PaginatedDocumentField> From<APIFilter<T>> for DBFilter<T> {
    fn from(value: APIFilter<T>) -> Self {
        match value {
            APIFilter::Expression(v) => DBFilter::Expression(v.into()),
            APIFilter::Or(v) => DBFilter::Or(v.into_iter().map(|v| v.into()).collect()),
            APIFilter::And(v) => DBFilter::And(v.into_iter().map(|v| v.into()).collect()),
        }
    }
}

impl<T: PaginatedDocumentField> DBNormalize for DBFilter<T> {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}
