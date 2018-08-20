use std::fmt::Write;

use serde::{Deserialize, Serialize};

use crate::database::{AQL_DOCUMENT_ID, AqlBuilder};
use crate::database::types::{DBDataType, UserType};
use crate::database::types::filters::api::APIFilteringStatsConfig;
use crate::error::AppResult;
use crate::server::requests::PaginatedDocumentField;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: PaginatedDocumentField")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "T", content = "V")]
pub enum APIFilterField<T: PaginatedDocumentField> {
    #[serde(rename = "field")]
    Field(T),
    #[serde(rename = "value")]
    Constant(DBDataType),
    #[serde(rename = "func")]
    Function(APIFilterFieldFunction<T>),
}

impl<T: PaginatedDocumentField> APIFilterField<T> {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self, user_type: UserType) -> Result<(), &'static str> {
        match self {
            APIFilterField::Field(v) => {
                if v.is_valid_for_filtering(user_type) {
                    Ok(())
                } else {
                    Err("The field is not valid for filtering")
                }
            }
            APIFilterField::Constant(_) => Ok(()),
            APIFilterField::Function(v) => v.validate(user_type),
        }
    }

    pub fn calculate_stats(&self, stats: &mut APIFilteringStatsConfig) {
        match self {
            APIFilterField::Field(_) => {
                stats.field_count += 1;
            }
            APIFilterField::Constant(_) => {
                stats.const_count += 1;
            }
            APIFilterField::Function(v) => {
                stats.function_count += 1;
                v.calculate_stats(stats);
            }
        }
    }

    pub fn build_aql(&self, query: &mut String, aql: &mut AqlBuilder) -> AppResult<()> {
        match self {
            APIFilterField::Field(v) => {
                query
                    .write_fmt(format_args!("{}.{}", AQL_DOCUMENT_ID, v.path_to_value()))
                    .unwrap();
            }
            APIFilterField::Constant(v) => {
                let param = v.as_aql_param();
                let index = aql.add_variable_as_json(param)?;
                query.push_str(index);
            }
            APIFilterField::Function(v) => {
                v.build_aql(query, aql)?;
            }
        }

        Ok(())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(bound = "T: PaginatedDocumentField")]
pub struct APIFilterFieldFunction<T: PaginatedDocumentField> {
    pub name: APIFilterFieldFunctionKind,
    pub args: Vec<APIFilterField<T>>,
}

impl<T: PaginatedDocumentField> APIFilterFieldFunction<T> {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self, user_type: UserType) -> Result<(), &'static str> {
        self.name.validate(&self.args, user_type)
    }

    pub fn calculate_stats(&self, stats: &mut APIFilteringStatsConfig) {
        for arg in &self.args {
            arg.calculate_stats(stats);
        }
    }

    pub fn build_aql(&self, query: &mut String, aql: &mut AqlBuilder) -> AppResult<()> {
        match self.name {
            APIFilterFieldFunctionKind::Length => query.push_str("LENGTH"),
            APIFilterFieldFunctionKind::Nth => query.push_str("NTH"),
            APIFilterFieldFunctionKind::IndexOf => query.push_str("POSITION"),
            APIFilterFieldFunctionKind::ListContains => query.push_str("POSITION"),
            APIFilterFieldFunctionKind::HasKey => query.push_str("HAS"),
            APIFilterFieldFunctionKind::Not => query.push_str("!TO_BOOL"),
            APIFilterFieldFunctionKind::Trim => query.push_str("TRIM"),
            APIFilterFieldFunctionKind::Lowercase => query.push_str("LOWER"),
            APIFilterFieldFunctionKind::Uppercase => query.push_str("UPPER"),
            APIFilterFieldFunctionKind::StartsWith => {
                let mut args = self.args.iter();
                let arg0 = args.next().unwrap();
                let arg1 = args.next().unwrap();

                query.push_str("(LEFT(");
                arg0.build_aql(query, aql)?;
                query.push_str(",LENGTH(");
                arg1.build_aql(query, aql)?;
                query.push_str(")) == ");
                arg1.build_aql(query, aql)?;
                query.push(')');

                return Ok(());
            }
            APIFilterFieldFunctionKind::EndsWith => {
                let mut args = self.args.iter();
                let arg0 = args.next().unwrap();
                let arg1 = args.next().unwrap();

                query.push_str("(RIGHT(");
                arg0.build_aql(query, aql)?;
                query.push_str(",LENGTH(");
                arg1.build_aql(query, aql)?;
                query.push_str(")) == ");
                arg1.build_aql(query, aql)?;
                query.push(')');

                return Ok(());
            }
            APIFilterFieldFunctionKind::Like => query.push_str("LIKE"),
            APIFilterFieldFunctionKind::Regex => query.push_str("REGEX_TEST"),
            APIFilterFieldFunctionKind::CharAt => query.push_str("SUBSTRING"),
            APIFilterFieldFunctionKind::Contains => query.push_str("CONTAINS"),
            APIFilterFieldFunctionKind::Typename => query.push_str("TYPENAME"),
        }

        query.push('(');

        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 {
                query.push(',');
            }
            arg.build_aql(query, aql)?;
        }

        // Add additional args.
        match self.name {
            APIFilterFieldFunctionKind::IndexOf => {
                query.push_str(",true");
            }
            APIFilterFieldFunctionKind::CharAt => {
                query.push_str(",1");
            }
            _ => {}
        }

        query.push(')');
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum APIFilterFieldFunctionKind {
    // GENERAL
    /// For arrays, strings and objects.
    Length,

    // ARRAYS
    Nth,
    IndexOf,
    ListContains,

    // OBJECTS
    HasKey,

    // LOGIC
    Not,

    // STRING
    Trim,
    Lowercase,
    Uppercase,
    StartsWith,
    EndsWith,
    Like,
    Regex,
    CharAt,
    Contains,

    // TYPE CHECKING
    Typename,
}

impl APIFilterFieldFunctionKind {
    // METHODS ----------------------------------------------------------------

    pub fn validate<T: PaginatedDocumentField>(
        &self,
        args: &[APIFilterField<T>],
        user_type: UserType,
    ) -> Result<(), &'static str> {
        match self {
            APIFilterFieldFunctionKind::Length => {
                if args.len() != 1 {
                    return Err("The 'length' function requires 1 argument");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'length' function must be a string or field");
                }
            }
            APIFilterFieldFunctionKind::Nth => {
                if args.len() != 2 {
                    return Err("The 'nth' function only accept one extra argument");
                }

                if !matches!(args[0], APIFilterField::Field(_)) {
                    return Err("The args[0] of 'nth' function must be a field");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::Natural(_))
                ) {
                    return Err("The args[1] of 'nth' function must be a natural number");
                }
            }
            APIFilterFieldFunctionKind::IndexOf => {
                if args.len() != 2 {
                    return Err("The 'indexOf' function only accept one extra argument");
                }

                if !matches!(args[0], APIFilterField::Field(_)) {
                    return Err("The args[0] of 'indexOf' function must be a field");
                }

                // args[1] = any
            }
            APIFilterFieldFunctionKind::ListContains => {
                if args.len() != 2 {
                    return Err("The 'listContains' function only accept one extra argument");
                }

                if !matches!(args[0], APIFilterField::Field(_)) {
                    return Err("The args[0] of 'listContains' function must be a field");
                }

                // args[1] = any
            }
            APIFilterFieldFunctionKind::HasKey => {
                if args.len() != 2 {
                    return Err("The 'hasKey' function only accept one extra argument");
                }

                if !matches!(args[0], APIFilterField::Field(_)) {
                    return Err("The args[0] of 'hasKey' function must be a field");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'hasKey' function must be a string");
                }
            }
            APIFilterFieldFunctionKind::Not => {
                if args.len() != 1 {
                    return Err("The 'not' function requires 1 argument");
                }

                // args[0] = any
            }
            APIFilterFieldFunctionKind::Trim => {
                if args.len() != 1 {
                    return Err("The 'trim' function requires 1 argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'trim' function must be a string");
                }
            }
            APIFilterFieldFunctionKind::Lowercase => {
                if args.len() != 1 {
                    return Err("The 'lowercase' function requires 1 argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'lowercase' function must be a string");
                }
            }
            APIFilterFieldFunctionKind::Uppercase => {
                if args.len() != 1 {
                    return Err("The 'uppercase' function requires 1 argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'uppercase' function must be a string");
                }
            }
            APIFilterFieldFunctionKind::StartsWith => {
                if args.len() != 2 {
                    return Err("The 'startsWith' function only accept one extra argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'startsWith' function must be a string");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'startsWith' function must be a string");
                }
            }
            APIFilterFieldFunctionKind::EndsWith => {
                if args.len() != 2 {
                    return Err("The 'endsWith' function only accept one extra argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'endsWith' function must be a string");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'endsWith' function must be a string");
                }
            }
            APIFilterFieldFunctionKind::Like => {
                if args.len() != 3 {
                    return Err("The 'like' function only accept one extra argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[0] of 'like' function must be a string");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'like' function must be a string");
                }

                if !matches!(
                    args[2],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::Bool(_))
                ) {
                    return Err("The args[2] of 'like' function must be a bool");
                }
            }
            APIFilterFieldFunctionKind::Regex => {
                if args.len() != 3 {
                    return Err("The 'regex' function only accept one extra argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[0] of 'regex' function must be a string");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'regex' function must be a string");
                }

                if !matches!(
                    args[2],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::Bool(_))
                ) {
                    return Err("The args[2] of 'regex' function must be a bool");
                }
            }
            APIFilterFieldFunctionKind::CharAt => {
                if args.len() != 2 {
                    return Err("The 'charAt' function only accept one extra argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[0] of 'charAt' function must be a string");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::Natural(_))
                ) {
                    return Err("The args[1] of 'charAt' function must be a natural number");
                }
            }
            APIFilterFieldFunctionKind::Contains => {
                if args.len() != 2 {
                    return Err("The 'contains' function only accept one extra argument");
                }

                if !matches!(
                    args[0],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[0] of 'contains' function must be a string");
                }

                if !matches!(
                    args[1],
                    APIFilterField::Field(_) | APIFilterField::Constant(DBDataType::String(_))
                ) {
                    return Err("The args[1] of 'contains' function must be a string");
                }
            }
            APIFilterFieldFunctionKind::Typename => {
                if args.len() != 1 {
                    return Err("The 'typename' function requires 1 argument");
                }

                // args[0] = any
            }
        }

        // Check at least one field is a field.
        let mut at_least_one_field_is_field = false;
        for arg in args {
            arg.validate(user_type)?;

            at_least_one_field_is_field |= !matches!(arg, APIFilterField::Constant(_));
        }

        if !at_least_one_field_is_field {
            return Err("Functions require that at least one argument is a field");
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
    use crate::database::types::DBDataType;

    use super::*;

    #[test]
    fn test_aql() {
        // Case 1: simple field
        let field =
            APIFilterField::<ChallengeAPIDocumentField>::Field(ChallengeAPIDocumentField::Id);
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        field.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!("{}._key", AQL_DOCUMENT_ID),
            "[1] The query is incorrect"
        );

        // Case 2: complex field
        let field = APIFilterField::<ChallengeAPIDocumentField>::Field(
            ChallengeAPIDocumentField::Name,
        );
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        field.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!("{}.N", AQL_DOCUMENT_ID),
            "[2] The query is incorrect"
        );

        // Case 3: data field
        let field = APIFilterField::<ChallengeAPIDocumentField>::Constant(DBDataType::Bool(true));
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        field.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(query, "@v0", "[3] The query is incorrect");

        // Case 4: function
        let field = APIFilterField::<ChallengeAPIDocumentField>::Function(APIFilterFieldFunction {
            name: APIFilterFieldFunctionKind::Contains,
            args: vec![
                APIFilterField::Field(ChallengeAPIDocumentField::Name),
                APIFilterField::Constant(DBDataType::String(arcstr::literal!("test"))),
            ],
        });
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        field.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!("CONTAINS({}.N,@v0)", AQL_DOCUMENT_ID),
            "[4] The query is incorrect"
        );

        // Case 5: function complex
        let field = APIFilterField::<ChallengeAPIDocumentField>::Function(APIFilterFieldFunction {
            name: APIFilterFieldFunctionKind::StartsWith,
            args: vec![
                APIFilterField::Field(ChallengeAPIDocumentField::Name),
                APIFilterField::Constant(DBDataType::String(arcstr::literal!("test"))),
            ],
        });
        let mut query = String::new();
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, "Collection");
        field.build_aql(&mut query, &mut aql).unwrap();

        assert_eq!(
            query,
            format!("(LEFT({}.N,LENGTH(@v0)) == @v1)", AQL_DOCUMENT_ID),
            "[5] The query is incorrect"
        );
    }
}
