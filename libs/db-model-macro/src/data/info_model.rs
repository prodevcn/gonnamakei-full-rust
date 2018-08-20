use std::collections::HashSet;
use std::iter::Filter;
use std::slice::Iter;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, ToTokens, TokenStreamExt};
use syn::{Attribute, File, Generics, Item, ItemEnum, ItemFn, ItemStruct, Visibility};

use crate::constants::{
    API_DOCUMENT_PREFIX, DB_COLLECTION_SUFFIX, DB_DOCUMENT_SUFFIX, FIELDS_SUFFIX,
    MUTEX_FIELD_DB_NAME,
};
use crate::data::{FieldInfo, ModelOptions, StructAttributes};
use crate::errors::Error;

pub const NORMALIZE_OR_REMOVE_METHOD_NAME: &str = "normalize";
pub const NORMALIZE_FIELDS_METHOD_NAME: &str = "normalize_fields";
pub const FILTER_METHOD_NAME: &str = "filter";
pub const IS_VALID_FOR_SORTING: &str = "is_valid_for_sorting";
pub const IS_VALID_FOR_FILTERING: &str = "is_valid_for_filtering";
pub const FROM_DB_TO_API: &str = "from_db_to_api";
pub const FROM_API_TO_DB: &str = "from_api_to_db";

pub struct ModelInfo<'a> {
    pub file: &'a File,
    pub item: ModelNode<'a>,
    pub item_attributes: StructAttributes,
    pub item_fields: Vec<FieldInfo<'a>>,
    pub replace_normalize: Option<&'a ItemFn>,
    pub replace_normalize_fields: Option<&'a ItemFn>,
    pub replace_filter: Option<&'a ItemFn>,
    pub replace_api_is_valid_for_sorting: Option<&'a ItemFn>,
    pub replace_api_is_valid_for_filtering: Option<&'a ItemFn>,
    pub replace_api_from_db_to_api: Option<&'a ItemFn>,
    pub replace_api_from_api_to_db: Option<&'a ItemFn>,
    // Other info
    pub document_name: Ident,
    pub collection_name: Ident,
    pub field_enum_name: Ident,
    pub api_document_name: Ident,
    pub api_field_enum_name: Ident,
}

impl<'a> ModelInfo<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn from_file_for_model(
        options: &ModelOptions,
        file: &'a File,
    ) -> Result<ModelInfo<'a>, syn::Error> {
        let mut items_iter = file.items.iter();

        // Check a struct is present and in the first position.
        let struct_item = match items_iter.next() {
            Some(v) => v,
            None => return Err(Error::MissingStructItem.with_tokens(file)),
        };

        let struct_item = match struct_item {
            Item::Struct(v) => v,
            _ => return Err(Error::MissingStructItem.with_tokens(file)),
        };
        let struct_attributes = StructAttributes::from_attributes(&struct_item.attrs)?;

        // Check struct fields.
        let mut struct_fields = Vec::with_capacity(struct_item.fields.len());
        for field in &struct_item.fields {
            struct_fields.push(FieldInfo::from_field(field)?);
        }

        // Build other info.
        let document_name = format_ident!("{}{}", struct_item.ident, DB_DOCUMENT_SUFFIX);
        let api_document_name = Self::compute_api_document_name(options, &document_name);
        let collection_name = if let Some(collection_name) = &options.collection_name {
            format_ident!("{}", collection_name)
        } else {
            format_ident!("{}{}", struct_item.ident, DB_COLLECTION_SUFFIX)
        };
        let field_enum_name = format_ident!(
            "{}{}{}",
            struct_item.ident,
            DB_DOCUMENT_SUFFIX,
            FIELDS_SUFFIX
        );
        let api_field_enum_name = format_ident!("{}{}", api_document_name, FIELDS_SUFFIX);

        // Build result.
        let mut result = ModelInfo {
            file,
            item: ModelNode::Struct(struct_item),
            item_attributes: struct_attributes,
            item_fields: struct_fields,
            replace_normalize: None,
            replace_normalize_fields: None,
            replace_filter: None,
            replace_api_is_valid_for_sorting: None,
            replace_api_is_valid_for_filtering: None,
            replace_api_from_db_to_api: None,
            replace_api_from_api_to_db: None,
            document_name,
            collection_name,
            field_enum_name,
            api_document_name,
            api_field_enum_name,
        };

        // Analyze rest functions.
        result.analyze_rest_functions(items_iter)?;

        // Final checks.
        result.check_names(options)?;

        Ok(result)
    }

    pub fn from_file_for_sub_model(
        options: &ModelOptions,
        file: &'a File,
    ) -> Result<ModelInfo<'a>, syn::Error> {
        let mut items_iter = file.items.iter();

        // Check a struct/enum is present and in the first position.
        let item = match items_iter.next() {
            Some(v) => v,
            None => return Err(Error::MissingStructOrEnumItem.with_tokens(file)),
        };

        let item = match item {
            Item::Struct(v) => ModelNode::Struct(v),
            Item::Enum(v) => ModelNode::Enum(v),
            _ => return Err(Error::MissingStructOrEnumItem.with_tokens(file)),
        };
        let item_attributes = StructAttributes::from_attributes(item.attributes())?;

        // Check struct fields.
        let item_fields = match &item {
            ModelNode::Struct(item) => {
                let mut item_fields = Vec::with_capacity(item.fields.len());
                for field in &item.fields {
                    item_fields.push(FieldInfo::from_field(field)?);
                }
                item_fields
            }
            ModelNode::Enum(item) => {
                let mut item_fields = Vec::with_capacity(item.variants.len());
                for variant in &item.variants {
                    item_fields.push(FieldInfo::from_variant(variant)?);
                }
                item_fields
            }
        };

        // Build other info.
        let document_name = item.ident().clone();
        let collection_name = format_ident!("undefined");
        let field_enum_name = format_ident!("{}{}", document_name, FIELDS_SUFFIX);
        let api_document_name = Self::compute_api_document_name(options, &document_name);
        let api_field_enum_name = format_ident!("{}{}", api_document_name, FIELDS_SUFFIX);

        // Build result.
        let mut result = ModelInfo {
            file,
            item,
            item_attributes,
            item_fields,
            replace_normalize: None,
            replace_normalize_fields: None,
            replace_filter: None,
            replace_api_is_valid_for_sorting: None,
            replace_api_is_valid_for_filtering: None,
            replace_api_from_db_to_api: None,
            replace_api_from_api_to_db: None,
            document_name,
            api_document_name,
            collection_name,
            field_enum_name,
            api_field_enum_name,
        };

        // Analyze rest functions.
        result.analyze_rest_functions(items_iter)?;

        // Final checks.
        result.check_names(options)?;

        Ok(result)
    }

    // GETTERS ----------------------------------------------------------------

    pub fn check_all_db_fields_are_optional_or_properties(&self) -> bool {
        self.fields_in_db()
            .all(|field| field.field_type_kind.is_some())
    }

    pub fn check_all_api_fields_are_optional_or_properties(&self) -> bool {
        self.fields_in_api()
            .all(|field| field.field_type_kind.is_some())
    }

    pub fn check_all_db_variants_are_unit(&self) -> bool {
        self.fields_in_db().all(|field| field.inner_type.is_none())
    }

    pub fn check_all_api_variants_are_unit(&self) -> bool {
        self.fields_in_api().all(|field| field.inner_type.is_none())
    }

    pub fn fields_in_db(&self) -> Filter<Iter<'_, FieldInfo<'a>>, fn(&&'a FieldInfo<'a>) -> bool> {
        self.item_fields
            .iter()
            .filter(|field| !field.attributes.skip_in_db)
    }

    pub fn fields_in_api(&self) -> Filter<Iter<'_, FieldInfo<'a>>, fn(&&'a FieldInfo<'a>) -> bool> {
        self.item_fields
            .iter()
            .filter(|field| !field.attributes.skip_in_api)
    }

    // METHODS ----------------------------------------------------------------

    fn check_names(&self, options: &ModelOptions) -> Result<(), syn::Error> {
        let mut names = HashSet::with_capacity(self.item_fields.len());

        let key = "_key".to_string();
        let rev = "_rev".to_string();
        let mutex = MUTEX_FIELD_DB_NAME.to_string();
        names.insert(&key);
        names.insert(&rev);

        if options.sync_level.is_document_active() {
            names.insert(&mutex);
        }

        for field in &self.item_fields {
            let db_name = &field.db_name;
            if names.contains(db_name) {
                return Err(Error::DuplicatedStructName(db_name.clone()).with_tokens(&field.node));
            }

            names.insert(db_name);
        }

        Ok(())
    }

    fn analyze_rest_functions(&mut self, items_iter: Iter<'a, Item>) -> Result<(), syn::Error> {
        for item in items_iter {
            match item {
                Item::Fn(v) => {
                    let name = v.sig.ident.to_string();

                    match name.as_str() {
                        NORMALIZE_OR_REMOVE_METHOD_NAME => self.replace_normalize = Some(v),
                        NORMALIZE_FIELDS_METHOD_NAME => self.replace_normalize_fields = Some(v),
                        FILTER_METHOD_NAME => self.replace_filter = Some(v),
                        IS_VALID_FOR_SORTING => self.replace_api_is_valid_for_sorting = Some(v),
                        IS_VALID_FOR_FILTERING => self.replace_api_is_valid_for_filtering = Some(v),
                        FROM_DB_TO_API => self.replace_api_from_db_to_api = Some(v),
                        FROM_API_TO_DB => self.replace_api_from_api_to_db = Some(v),
                        _ => return Err(Error::UnexpectedItem.with_tokens(item)),
                    }
                }
                _ => return Err(Error::UnexpectedItem.with_tokens(item)),
            }
        }

        Ok(())
    }

    // STATIC METHODS ---------------------------------------------------------

    fn compute_api_document_name(options: &ModelOptions, name: &Ident) -> Ident {
        match &options.api_name {
            Some(v) => v.clone(),
            None => {
                let db_name = name.to_string();
                let api_name = db_name.replace("DB", "API");

                if db_name == api_name {
                    format_ident!("{}{}", API_DOCUMENT_PREFIX, api_name, span = name.span())
                } else {
                    format_ident!("{}", api_name, span = name.span())
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub enum ModelNode<'a> {
    Struct(&'a ItemStruct),
    Enum(&'a ItemEnum),
}

impl<'a> ModelNode<'a> {
    // GETTERS ----------------------------------------------------------------

    pub fn ident(&self) -> &Ident {
        match self {
            ModelNode::Struct(v) => &v.ident,
            ModelNode::Enum(v) => &v.ident,
        }
    }

    pub fn visibility(&self) -> &Visibility {
        match self {
            ModelNode::Struct(v) => &v.vis,
            ModelNode::Enum(v) => &v.vis,
        }
    }

    pub fn generics(&self) -> &Generics {
        match self {
            ModelNode::Struct(v) => &v.generics,
            ModelNode::Enum(v) => &v.generics,
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        match self {
            ModelNode::Struct(v) => &v.attrs,
            ModelNode::Enum(v) => &v.attrs,
        }
    }
}

impl<'a> ToTokens for ModelNode<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            ModelNode::Struct(v) => v.to_token_stream(),
            ModelNode::Enum(v) => v.to_token_stream(),
        });
    }
}
