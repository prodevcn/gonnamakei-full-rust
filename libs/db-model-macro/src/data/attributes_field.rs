use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, Type};

use crate::utils::{
    get_simple_name_from_meta, process_bool_literal, process_enum_literal, process_only_attribute,
    process_string_literal,
};

pub const DB_ATTR_ATTRIBUTE: &str = "db_attr";
pub const API_ATTR_ATTRIBUTE: &str = "api_attr";
pub const SKIP_IN_DB_ATTRIBUTE: &str = "skip_in_db";
pub const SKIP_IN_API_ATTRIBUTE: &str = "skip_in_api";
pub const SKIP_NORMALIZE_ATTRIBUTE: &str = "skip_normalize";
pub const DB_NAME_ATTRIBUTE: &str = "db_name";
pub const INNER_MODEL_ATTRIBUTE: &str = "inner_model";
pub static INNER_MODEL_ATTRIBUTE_NAMES: &[&str] = &["data", "struct", "enum"];
pub static INNER_MODEL_ATTRIBUTE_VALUES: &[InnerModelKind] = &[
    InnerModelKind::Data,
    InnerModelKind::Struct,
    InnerModelKind::Enum,
];
pub const API_INNER_TYPE_ATTRIBUTE: &str = "api_inner_type";
pub const API_SENSIBLE_INFO_ATTRIBUTE: &str = "api_sensible_info";
pub const API_SKIP_MAP_TO_NULL_ATTRIBUTE: &str = "api_skip_map_to_null";

#[derive(Default)]
pub struct FieldAttributes {
    pub db: Vec<TokenStream>,
    pub api: Vec<TokenStream>,
    pub skip_in_db: bool,
    pub skip_in_api: bool,
    pub skip_normalize: bool,
    pub db_name: Option<String>,
    pub inner_model: InnerModelKind,
    pub api_inner_type: Option<Type>,
    pub api_sensible_info: bool,
    pub api_skip_map_to_null: bool,
}

impl FieldAttributes {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn from_attributes(
        attributes: &[Attribute],
        in_enum: bool,
    ) -> Result<FieldAttributes, syn::Error> {
        let mut result = FieldAttributes::default();

        if in_enum {
            result.inner_model = InnerModelKind::Struct;
        }

        // Read every attribute, i.e. #[...]
        for attribute in attributes {
            // Transform the attribute as meta, i.e. removing the brackets.
            let meta = attribute.parse_meta()?;

            // Get the name.
            let name = match get_simple_name_from_meta(&meta) {
                Some(v) => v,
                None => {
                    result.db.push(attribute.to_token_stream());
                    result.api.push(attribute.to_token_stream());
                    continue;
                }
            };
            let name = name.as_str();

            match name {
                DB_ATTR_ATTRIBUTE => {
                    result.db.push(process_only_attribute(&meta, name)?);
                }
                API_ATTR_ATTRIBUTE => {
                    result.api.push(process_only_attribute(&meta, name)?);
                }
                SKIP_IN_DB_ATTRIBUTE => {
                    result.skip_in_db = process_bool_literal(&meta, name, Some(true))?;
                }
                SKIP_IN_API_ATTRIBUTE => {
                    result.skip_in_api = process_bool_literal(&meta, name, Some(true))?;
                }
                SKIP_NORMALIZE_ATTRIBUTE => {
                    result.skip_normalize = process_bool_literal(&meta, name, Some(true))?;
                }
                DB_NAME_ATTRIBUTE => {
                    result.db_name = Some(process_string_literal(&meta, name, None)?);
                }
                INNER_MODEL_ATTRIBUTE => {
                    result.inner_model = process_enum_literal(
                        &meta,
                        INNER_MODEL_ATTRIBUTE_NAMES,
                        INNER_MODEL_ATTRIBUTE_VALUES,
                        name,
                        None,
                    )?;
                }
                API_INNER_TYPE_ATTRIBUTE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.api_inner_type = Some(syn::parse_str(&value)?);
                }
                API_SENSIBLE_INFO_ATTRIBUTE => {
                    result.api_sensible_info = process_bool_literal(&meta, name, Some(true))?;
                }
                API_SKIP_MAP_TO_NULL_ATTRIBUTE => {
                    result.api_skip_map_to_null = process_bool_literal(&meta, name, Some(true))?;
                }
                _ => {
                    result.db.push(attribute.to_token_stream());
                    result.api.push(attribute.to_token_stream());
                    continue;
                }
            }
        }

        Ok(result)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InnerModelKind {
    Data,
    Struct,
    Enum,
}

impl Default for InnerModelKind {
    fn default() -> Self {
        Self::Data
    }
}
