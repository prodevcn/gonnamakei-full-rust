use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Attribute;

use crate::utils::{get_simple_name_from_meta, process_bool_literal, process_only_attribute};

pub const DB_ATTR_ATTRIBUTE: &str = "db_attr";
pub const API_ATTR_ATTRIBUTE: &str = "api_attr";
pub const SKIP_DEFAULT_ATTRIBUTE: &str = "skip_default";

#[derive(Default)]
pub struct StructAttributes {
    pub db: Vec<TokenStream>,
    pub api: Vec<TokenStream>,
    pub skip_default: bool,
}

impl StructAttributes {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn from_attributes(attributes: &[Attribute]) -> Result<StructAttributes, syn::Error> {
        let mut result = StructAttributes::default();

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
                SKIP_DEFAULT_ATTRIBUTE => {
                    result.skip_default = process_bool_literal(&meta, name, Some(true))?;
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
