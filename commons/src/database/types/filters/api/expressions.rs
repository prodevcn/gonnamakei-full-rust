use serde::{Deserialize, Serialize};

use crate::database::AqlBuilder;
use crate::database::documents::APIFilteringStatsConfig;
use crate::database::types::{APIFilterField, DBDataType, FilterOperator, UserType};
use crate::error::AppResult;
use crate::server::requests::PaginatedDocumentField;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: PaginatedDocumentField")]
pub struct APIFilterExpression<T: PaginatedDocumentField> {
    pub left: APIFilterField<T>,
    pub operator: FilterOperator,
    pub right: APIFilterField<T>,
}

impl<T: PaginatedDocumentField> APIFilterExpression<T> {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self, user_type: UserType) -> Result<(), &'static str> {
        self.left.validate(user_type)?;
        self.right.validate(user_type)?;

        match (&self.left, &self.right) {
            (APIFilterField::Constant(_), APIFilterField::Constant(_)) => {
                Err("Cannot compare two constant values")
            }
            (APIFilterField::Constant(v), _) => self.operator.validate_left(v),
            (_, APIFilterField::Constant(v)) => self.operator.validate_right(v),
            _ => {
                // Both are fields or methods so they can be compared.
                Ok(())
            }
        }
    }

    pub fn calculate_stats(&self, stats: &mut APIFilteringStatsConfig) {
        self.left.calculate_stats(stats);
        self.right.calculate_stats(stats);
    }

    pub fn build_aql(&self, query: &mut String, aql: &mut AqlBuilder) -> AppResult<()> {
        self.left.build_aql(query, aql)?;
        query.push(' ');
        self.operator.build_aql(query);
        query.push(' ');
        self.right.build_aql(query, aql)?;
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

impl FilterOperator {
    // METHODS ----------------------------------------------------------------

    pub fn validate_left(&self, argument: &DBDataType) -> Result<(), &'static str> {
        match self {
            FilterOperator::LikePattern => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The 'like' operator requires that its left operand be a String");
                }
            }
            FilterOperator::NotLikePattern => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The '!like' operator requires that its left operand be a String");
                }
            }
            FilterOperator::LikeRegex => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The 'regex' operator requires that its left operand be a String");
                }
            }
            FilterOperator::NotLikeRegex => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The '!regex' operator requires that its left operand be a String");
                }
            }
            FilterOperator::AllArray(_) => {
                return Err("The '[all]' operator requires that its left operand be a field");
            }
            FilterOperator::AnyArray(_) => {
                return Err("The '[any]' operator requires that its left operand be a field");
            }
            FilterOperator::NoneArray(_) => {
                return Err("The '[none]' operator requires that its left operand be a field");
            }
            _ => {
                // Accepts any
            }
        }

        Ok(())
    }

    pub fn validate_right(&self, argument: &DBDataType) -> Result<(), &'static str> {
        match self {
            FilterOperator::InArray => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The 'in' operator requires that its right operand be a List");
                }
            }
            FilterOperator::NotInArray => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The '!in' operator requires that its right operand be a List");
                }
            }
            FilterOperator::LikePattern => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The 'like' operator requires that its right operand be a String");
                }
            }
            FilterOperator::NotLikePattern => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The '!like' operator requires that its right operand be a String");
                }
            }
            FilterOperator::LikeRegex => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err("The 'regex' operator requires that its right operand be a String");
                }
            }
            FilterOperator::NotLikeRegex => {
                if !matches!(argument, DBDataType::String(_)) {
                    return Err(
                        "The '!regex' operator requires that its right operand be a String",
                    );
                }
            }
            FilterOperator::AllArray(v) => {
                if matches!(
                    **v,
                    FilterOperator::AllArray(_)
                        | FilterOperator::AnyArray(_)
                        | FilterOperator::NoneArray(_)
                ) {
                    return Err("Cannot nest two array operations");
                }
            }
            FilterOperator::AnyArray(v) => {
                if matches!(
                    **v,
                    FilterOperator::AllArray(_)
                        | FilterOperator::AnyArray(_)
                        | FilterOperator::NoneArray(_)
                ) {
                    return Err("Cannot nest two array operations");
                }
            }
            FilterOperator::NoneArray(v) => {
                if matches!(
                    **v,
                    FilterOperator::AllArray(_)
                        | FilterOperator::AnyArray(_)
                        | FilterOperator::NoneArray(_)
                ) {
                    return Err("Cannot nest two array operations");
                }
            }
            _ => {
                // Accepts any
            }
        }

        Ok(())
    }

    pub fn build_aql(&self, query: &mut String) {
        match self {
            FilterOperator::Equal => query.push_str("=="),
            FilterOperator::NotEqual => query.push_str("!="),
            FilterOperator::GreaterThan => query.push('>'),
            FilterOperator::GreaterOrEqualThan => query.push_str(">="),
            FilterOperator::LessThan => query.push('<'),
            FilterOperator::LessOrEqualThan => query.push_str("<="),
            FilterOperator::InArray => query.push_str("IN"),
            FilterOperator::NotInArray => query.push_str("NOT IN"),
            FilterOperator::LikePattern => query.push_str("LIKE"),
            FilterOperator::NotLikePattern => query.push_str("NOT LIKE"),
            FilterOperator::LikeRegex => query.push_str("=~"),
            FilterOperator::NotLikeRegex => query.push_str("!~"),
            FilterOperator::AllArray(v) => {
                query.push_str("ALL ");
                v.build_aql(query);
            }
            FilterOperator::AnyArray(v) => {
                query.push_str("ANY ");
                v.build_aql(query);
            }
            FilterOperator::NoneArray(v) => {
                query.push_str("NONE ");
                v.build_aql(query);
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::database::AQL_DOCUMENT_ID;
    use crate::database::documents::ChallengeAPIDocumentField;

    use super::*;

    #[test]
    fn test_aql() {
        // Case 1: simple operator
        let expression = APIFilterExpression {
            left: APIFilterField::Field(ChallengeAPIDocumentField::Name),
            operator: FilterOperator::Equal,
            right: APIFilterField::Field(ChallengeAPIDocumentField::Id),
        };
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        expression.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!("{}.N == {}._key", AQL_DOCUMENT_ID, AQL_DOCUMENT_ID),
            "[1] The query is incorrect"
        );

        // Case 2: complex operator
        let expression = APIFilterExpression {
            left: APIFilterField::Field(ChallengeAPIDocumentField::Name),
            operator: FilterOperator::AllArray(Box::new(FilterOperator::NotInArray)),
            right: APIFilterField::Field(ChallengeAPIDocumentField::Id),
        };
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        expression.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!("{}.N ALL NOT IN {}._key", AQL_DOCUMENT_ID, AQL_DOCUMENT_ID),
            "[2] The query is incorrect"
        );
    }
}
