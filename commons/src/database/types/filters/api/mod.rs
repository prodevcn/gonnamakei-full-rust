use serde::{Deserialize, Serialize};

pub use expressions::*;
pub use fields::*;

use crate::database::AqlBuilder;
use crate::database::documents::APIFilteringStatsConfig;
use crate::database::types::UserType;
use crate::error::AppResult;
use crate::server::requests::PaginatedDocumentField;

mod expressions;
mod fields;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: PaginatedDocumentField")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum APIFilter<T: PaginatedDocumentField> {
    #[serde(rename = "expr")]
    Expression(APIFilterExpression<T>),
    Or(Vec<APIFilter<T>>),
    And(Vec<APIFilter<T>>),
}

impl<T: PaginatedDocumentField> APIFilter<T> {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self, user_type: UserType) -> Result<(), &'static str> {
        match self {
            APIFilter::Expression(v) => {
                v.validate(user_type)?;
            }
            APIFilter::Or(v) => {
                if v.is_empty() {
                    return Err("An 'or' operation cannot be empty");
                }

                for operation in v.iter() {
                    if matches!(*operation, APIFilter::Or(_)) {
                        return Err("Cannot nest two 'or' operations");
                    }

                    operation.validate(user_type)?;
                }
            }
            APIFilter::And(v) => {
                if v.is_empty() {
                    return Err("An 'and' operation cannot be empty");
                }

                for operation in v.iter() {
                    if matches!(*operation, APIFilter::And(_)) {
                        return Err("Cannot nest two 'and' operations");
                    }

                    operation.validate(user_type)?;
                }
            }
        }

        Ok(())
    }

    pub fn normalize(self) -> Self {
        match self {
            APIFilter::Expression(v) => APIFilter::Expression(v),
            APIFilter::Or(v) => {
                let mut v: Vec<_> = v.into_iter().map(|v| v.normalize()).collect();

                if v.len() == 1 {
                    v.remove(0)
                } else {
                    APIFilter::Or(v)
                }
            }
            APIFilter::And(v) => {
                let mut v: Vec<_> = v.into_iter().map(|v| v.normalize()).collect();

                if v.len() == 1 {
                    v.remove(0)
                } else {
                    APIFilter::And(v)
                }
            }
        }
    }

    pub fn calculate_stats(&self, stats: &mut APIFilteringStatsConfig) {
        match self {
            APIFilter::Expression(v) => {
                stats.expression_count += 1;
                v.calculate_stats(stats);
            }
            APIFilter::Or(v) => {
                for filter in v {
                    filter.calculate_stats(stats);
                }
            }
            APIFilter::And(v) => {
                for filter in v {
                    filter.calculate_stats(stats);
                }
            }
        }
    }

    pub fn build_aql(&self, query: &mut String, aql: &mut AqlBuilder) -> AppResult<()> {
        self.build_aql_core(query, aql)
    }

    pub(crate) fn build_aql_core(&self, query: &mut String, aql: &mut AqlBuilder) -> AppResult<()> {
        match self {
            APIFilter::Expression(v) => {
                v.build_aql(query, aql)?;
            }
            APIFilter::Or(v) => {
                query.push('(');
                for (i, value) in v.iter().enumerate() {
                    if i != 0 {
                        query.push_str(" || ");
                    }

                    value.build_aql_core(query, aql)?;
                }
                query.push(')');
            }
            APIFilter::And(v) => {
                query.push('(');
                for (i, value) in v.iter().enumerate() {
                    if i != 0 {
                        query.push_str(" && ");
                    }

                    value.build_aql_core(query, aql)?;
                }
                query.push(')');
            }
        }

        Ok(())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::database::{AQL_DOCUMENT_ID, AqlBuilder};
    use crate::database::documents::ChallengeAPIDocumentField;
    use crate::database::types::FilterOperator;

    use super::*;

    #[test]
    fn test_aql() {
        // Case 1: simple expression
        let expression = APIFilter::Expression(APIFilterExpression {
            left: APIFilterField::Field(ChallengeAPIDocumentField::Name),
            operator: FilterOperator::Equal,
            right: APIFilterField::Field(ChallengeAPIDocumentField::Id),
        });
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        expression.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!("{}.N == {}._key", AQL_DOCUMENT_ID, AQL_DOCUMENT_ID),
            "[1] The query is incorrect"
        );

        // Case 2: or expression
        let expression = APIFilter::Or(vec![
            APIFilter::Expression(APIFilterExpression {
                left: APIFilterField::Field(ChallengeAPIDocumentField::Name),
                operator: FilterOperator::Equal,
                right: APIFilterField::Field(ChallengeAPIDocumentField::Id),
            }),
            APIFilter::Expression(APIFilterExpression {
                left: APIFilterField::Field(ChallengeAPIDocumentField::Name),
                operator: FilterOperator::Equal,
                right: APIFilterField::Field(ChallengeAPIDocumentField::Id),
            }),
        ]);
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        expression.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!(
                "({}.N == {}._key || {}.N == {}._key)",
                AQL_DOCUMENT_ID, AQL_DOCUMENT_ID, AQL_DOCUMENT_ID, AQL_DOCUMENT_ID
            ),
            "[2] The query is incorrect"
        );

        // Case 3: and expression
        let expression = APIFilter::And(vec![
            APIFilter::Expression(APIFilterExpression {
                left: APIFilterField::Field(ChallengeAPIDocumentField::Name),
                operator: FilterOperator::Equal,
                right: APIFilterField::Field(ChallengeAPIDocumentField::Id),
            }),
            APIFilter::Expression(APIFilterExpression {
                left: APIFilterField::Field(ChallengeAPIDocumentField::Name),
                operator: FilterOperator::Equal,
                right: APIFilterField::Field(ChallengeAPIDocumentField::Id),
            }),
        ]);
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        expression.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!(
                "({}.N == {}._key && {}.N == {}._key)",
                AQL_DOCUMENT_ID, AQL_DOCUMENT_ID, AQL_DOCUMENT_ID, AQL_DOCUMENT_ID
            ),
            "[3] The query is incorrect"
        );
    }
}
