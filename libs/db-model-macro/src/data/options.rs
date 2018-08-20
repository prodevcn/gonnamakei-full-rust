use proc_macro2::Ident;
use quote::format_ident;
use syn::{Attribute, Path};

use crate::errors::Error;
use crate::utils::{
    get_simple_name_from_meta, process_bool_literal, process_enum_literal, process_string_literal,
};

pub const BUILD_API_ATTRIBUTE: &str = "build_api";
pub const SKIP_IMPL_ATTRIBUTE: &str = "skip_impl";
pub const SKIP_FIELDS_ATTRIBUTE: &str = "skip_fields";
pub const SYNC_LEVEL_ATTRIBUTE: &str = "sync_level";
pub static SYNC_LEVEL_ATTRIBUTE_NAMES: &[&str] =
    &["document", "collection", "document_and_collection"];
pub const SYNC_COLLECTION_KEY_METHOD_ATTRIBUTE: &str = "sync_collection_key_method";
pub const COLLECTION_NAME_ATTRIBUTE: &str = "collection_name";
pub const COLLECTION_KIND_ATTRIBUTE: &str = "collection_kind";
pub const REPLACE_NORMALIZE: &str = "replace_normalize";
pub const REPLACE_NORMALIZE_FIELDS: &str = "replace_normalize_fields";
pub const REPLACE_FILTER_ATTRIBUTE: &str = "replace_filter";
pub const API_NAME_ATTRIBUTE: &str = "api_name";
pub const API_PAGINATED_ATTRIBUTE: &str = "api_paginated";
pub const API_MIN_ROWS_PER_PAGE: &str = "api_min_rows_per_page";
pub const API_MAX_ROWS_PER_PAGE: &str = "api_max_rows_per_page";

#[derive(Debug, Default)]
pub struct ModelOptions {
    pub build_api: bool,
    pub skip_impl: bool,
    pub skip_fields: bool,
    pub serialize_fields: bool,
    pub sync_level: SyncLevelType,
    pub sync_collection_key_method: Option<Ident>,
    pub collection_name: Option<Ident>,
    pub collection_kind: Option<Ident>,
    pub replace_normalize: Option<Path>,
    pub replace_normalize_fields: Option<Path>,
    pub replace_filter: Option<Path>,
    pub api_name: Option<Ident>,
    pub api_paginated: bool,
    pub api_min_rows_per_page: Option<Path>,
    pub api_max_rows_per_page: Option<Path>,
}

impl ModelOptions {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn from_attributes(attributes: &[Attribute]) -> Result<ModelOptions, syn::Error> {
        let mut result = ModelOptions::default();
        #[allow(clippy::never_loop)]
        // Read every attribute, i.e. #[...]
        for attribute in attributes {
            // Transform the attribute as meta, i.e. removing the brackets.
            let meta = attribute.parse_meta()?;

            // Get the name.
            let name = match get_simple_name_from_meta(&meta) {
                Some(v) => v,
                None => return Err(Error::UnexpectedMacroOption.with_tokens(attribute)),
            };
            let name = name.as_str();

            match name {
                BUILD_API_ATTRIBUTE => {
                    result.build_api = process_bool_literal(&meta, name, Some(true))?;
                }
                SKIP_IMPL_ATTRIBUTE => {
                    result.skip_impl = process_bool_literal(&meta, name, Some(true))?;
                }
                SKIP_FIELDS_ATTRIBUTE => {
                    result.skip_fields = process_bool_literal(&meta, name, Some(true))?;
                }
                SYNC_LEVEL_ATTRIBUTE => {
                    static ENUM_LIST_VALUES: &[SyncLevelType] = &[
                        SyncLevelType::OnlyDocument,
                        SyncLevelType::OnlyCollection,
                        SyncLevelType::DocumentAndCollection,
                    ];

                    result.sync_level = process_enum_literal(
                        &meta,
                        SYNC_LEVEL_ATTRIBUTE_NAMES,
                        ENUM_LIST_VALUES,
                        name,
                        Some(SyncLevelType::DocumentAndCollection),
                    )?;
                }
                SYNC_COLLECTION_KEY_METHOD_ATTRIBUTE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.sync_collection_key_method = Some(format_ident!("{}", value));
                }
                COLLECTION_NAME_ATTRIBUTE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.collection_name = Some(format_ident!("{}", value));
                }
                COLLECTION_KIND_ATTRIBUTE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.collection_kind = Some(format_ident!("{}", value));
                }
                REPLACE_NORMALIZE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.replace_normalize = Some(syn::parse_str(&value)?);
                }
                REPLACE_NORMALIZE_FIELDS => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.replace_normalize_fields = Some(syn::parse_str(&value)?);
                }
                REPLACE_FILTER_ATTRIBUTE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.replace_filter = Some(syn::parse_str(&value)?);
                }
                API_NAME_ATTRIBUTE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.api_name = Some(format_ident!("{}", value));
                }
                API_PAGINATED_ATTRIBUTE => {
                    result.api_paginated = process_bool_literal(&meta, name, Some(true))?;
                }
                API_MIN_ROWS_PER_PAGE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.api_min_rows_per_page = Some(syn::parse_str(&value)?);
                }
                API_MAX_ROWS_PER_PAGE => {
                    let value = process_string_literal(&meta, name, None)?;
                    result.api_max_rows_per_page = Some(syn::parse_str(&value)?);
                }
                _ => return Err(Error::UnexpectedMacroOption.with_tokens(attribute)),
            }
        }

        Ok(result)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SyncLevelType {
    None,
    OnlyDocument,
    OnlyCollection,
    DocumentAndCollection,
}

impl SyncLevelType {
    // GETTERS ----------------------------------------------------------------

    pub fn is_document_active(&self) -> bool {
        *self == SyncLevelType::OnlyDocument || *self == SyncLevelType::DocumentAndCollection
    }

    pub fn is_collection_active(&self) -> bool {
        *self == SyncLevelType::OnlyCollection || *self == SyncLevelType::DocumentAndCollection
    }

    pub fn is_active(&self) -> bool {
        *self != SyncLevelType::None
    }
}

impl Default for SyncLevelType {
    fn default() -> Self {
        Self::None
    }
}
