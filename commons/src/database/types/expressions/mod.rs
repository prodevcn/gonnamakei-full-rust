use serde::{Deserialize, Serialize};

pub use fields::*;

use crate::database::traits::{DBNormalize, DBNormalizeResult};

mod fields;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum DBExpression<T> {
    #[serde(rename = "E")]
    Expression(DBExpressionField<T>),
    #[serde(rename = "U")]
    Unitary(DBUnitaryExpression<T>),
    #[serde(rename = "B")]
    Binary(DBBinaryExpression<T>),
}

impl<T> DBNormalize for DBExpression<T> {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DBUnitaryExpression<T> {
    #[serde(rename = "O")]
    pub operand: DBExpressionField<T>,
    #[serde(rename = "K")]
    pub operator: DBExpressionUnitaryOperator,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DBBinaryExpression<T> {
    #[serde(rename = "O")]
    pub operands: Vec<DBExpression<T>>,
    #[serde(rename = "K")]
    pub operator: DBExpressionBinaryOperator,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum DBExpressionUnitaryOperator {
    #[serde(rename = "!")]
    Not,
    #[serde(rename = "-")]
    Minus,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum DBExpressionBinaryOperator {
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Sub,
    #[serde(rename = "*")]
    Multiply,
    #[serde(rename = "/")]
    Divide,
    #[serde(rename = "%")]
    Remainder,
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
    #[serde(rename = "&")]
    LogicAnd,
    #[serde(rename = "|")]
    LogicOr,
    #[serde(rename = "^")]
    LogicXor,
    #[serde(rename = "&&")]
    ConditionalAnd,
    #[serde(rename = "||")]
    ConditionalOr,

    /// This operator is applied to any kind of expression in both sides, evaluation them
    /// from left to right discarding their results and returning only the last one.
    /// This is usually used to set variables: SET_VAR("name", value) -> expression
    #[serde(rename = "->")]
    Implication,
}
