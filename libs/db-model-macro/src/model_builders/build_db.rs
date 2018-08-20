use proc_macro2::TokenStream;
use quote::quote;
use quote::{format_ident, ToTokens};
use syn::spanned::Spanned;

use crate::constants::{MUTEX_FIELD_DB_NAME, MUTEX_FIELD_NAME};
use crate::data::{
    BaseTypeKind, FieldInfo, FieldTypeKind, InnerModelKind, ModelInfo, ModelOptions,
};
use crate::utils::{from_camel_or_pascal_case_to_snake_case, from_snake_case_to_pascal_case};

pub fn build_db_model(options: &ModelOptions, info: &ModelInfo) -> Result<TokenStream, syn::Error> {
    let fields_in_db: Vec<_> = info.fields_in_db().collect();
    let struct_tokens = build_struct(options, info, &fields_in_db)?;
    let impl_tokens = if !options.skip_impl {
        let impl_tokens = build_impl(options, info, &fields_in_db)?;
        let db_document_impl_tokens = build_db_document_impl(options, info, &fields_in_db)?;
        quote! {
            #impl_tokens
            #db_document_impl_tokens
        }
    } else {
        quote! {}
    };

    let field_list_tokens = if !options.skip_fields {
        build_db_struct_field_list(options, info, &fields_in_db)?
    } else {
        quote! {}
    };

    let sync_impl_tokens = if options.sync_level.is_active() {
        build_sync_impl(options, info)?
    } else {
        quote! {}
    };

    let edge_db_document_impl_tokens = check_and_build_edge_db_impl(options, info, &fields_in_db)?;
    let aql_mapping_impl_tokens =
        build_db_struct_aql_mapping_impl(options, info, false, &fields_in_db)?;

    // Build result.
    Ok(quote! {
        #struct_tokens
        #impl_tokens
        #field_list_tokens
        #sync_impl_tokens
        #edge_db_document_impl_tokens
        #aql_mapping_impl_tokens
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_struct(
    options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let attribute_list = &info.item_attributes.db;
    let visibility = info.item.visibility();
    let generics = info.item.generics();
    let document_name = &info.document_name;

    let all_fields_are_optional_or_db_properties =
        info.check_all_db_fields_are_optional_or_properties();

    // Evaluate default attribute.
    let default_attribute =
        if !info.item_attributes.skip_default && all_fields_are_optional_or_db_properties {
            quote! {
                #[derive(Default)]
                #[serde(default)]
            }
        } else {
            quote! {}
        };

    // Evaluate lock field.
    let lock_field = if options.sync_level.is_document_active() {
        let name = format_ident!("{}", MUTEX_FIELD_NAME);
        let db_name = MUTEX_FIELD_DB_NAME;
        quote! {
            #[serde(skip_serializing_if = "NullableOption::is_missing")]
            #[serde(rename = #db_name)]
            pub #name: NullableOption<DBMutex>,
        }
    } else {
        quote! {}
    };

    // Evaluate rest fields.
    let field_list = fields_in_db.iter().map(|field| {
        let node = field.node.as_field().unwrap();
        let visibility = &node.vis;
        let attribute_list = &field.attributes.db;
        let name = field.name();
        let db_name = &field.db_name;
        let field_type = field.build_db_field_type();

        quote! {
            #(#attribute_list)*
            #[serde(rename = #db_name)]
            #visibility #name: #field_type,
        }
    });

    // Build result.
    Ok(quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #default_attribute
        #(#attribute_list)*
        #visibility struct #document_name#generics {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "_key")]
            pub db_key: Option<DBUuid>,

            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "_rev")]
            pub db_rev: Option<ArcStr>,

            #lock_field

            #(#field_list)*
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_impl(
    options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let document_name = &info.document_name;

    let all_fields_are_optional_or_db_properties =
        info.check_all_db_fields_are_optional_or_properties();

    // Evaluate all null method.
    let all_null_method_tokens = if all_fields_are_optional_or_db_properties {
        let null_field_list = fields_in_db.iter().filter_map(|field| {
            let name = field.name();

            match field.field_type_kind {
                Some(FieldTypeKind::NullableOption) => Some(quote! {
                    #name: NullableOption::Null
                }),
                Some(FieldTypeKind::Option) => Some(quote! {
                    #name: None
                }),
                None => None,
            }
        });

        // Check mutex field.
        let mutex_field = if options.sync_level.is_document_active() {
            quote! {
                mutex: NullableOption::Null,
            }
        } else {
            quote! {}
        };

        quote! {
            #[allow(clippy::needless_update)]
            pub fn all_null() -> Self {
                Self {
                    #mutex_field
                    #(#null_field_list,)*
                    ..Default::default()
                }
            }
        }
    } else {
        quote! {}
    };

    // Build result.
    Ok(quote! {
        impl#generics #document_name#generics {
            #all_null_method_tokens
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_db_document_impl(
    options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let document_name = &info.document_name;
    let collection_name = &info.collection_name;
    let collection_kind = if let Some(collection_kind) = &options.collection_kind {
        format_ident!("{}", collection_kind)
    } else {
        format_ident!("{}s", info.item.ident())
    };

    let all_fields_are_optional_or_db_properties =
        info.check_all_db_fields_are_optional_or_properties();

    // Evaluate map_values_to_null_method method.
    let map_values_to_null_method_tokens = {
        let fields = fields_in_db.iter().filter_map(|field| {
            let name = field.name();

            match field.attributes.inner_model {
                InnerModelKind::Data => match field.field_type_kind {
                    Some(FieldTypeKind::NullableOption) => Some(quote! {
                        if self.#name.is_value() {
                            self.#name = NullableOption::Null;
                        }
                    }),
                    Some(FieldTypeKind::Option) => Some(quote! {
                        self.#name = None;
                    }),
                    None => None,
                },
                InnerModelKind::Struct | InnerModelKind::Enum => {
                    let base = match field.base_type_kind {
                        BaseTypeKind::Other | BaseTypeKind::Box => Some(quote! {
                            v.map_values_to_null();
                        }),
                        BaseTypeKind::Vec => Some(quote! {
                            for v in v {
                                v.map_values_to_null();
                            }
                        }),
                        BaseTypeKind::VecDBReference => {
                            panic!("Cannot declare a VecDBReference value as Struct or Enum model")
                        }
                        BaseTypeKind::HashMap => Some(quote! {
                            for (_, v) in v {
                                v.map_values_to_null();
                            }
                        }),
                        BaseTypeKind::DBReference => {
                            panic!("Cannot declare a DBReference value as Struct or Enum model")
                        }
                    };

                    match field.field_type_kind {
                        Some(FieldTypeKind::NullableOption) => base.map(|base| {
                            quote! {
                                if let NullableOption::Value(v) = &mut self.#name {
                                    #base
                                }
                            }
                        }),
                        Some(FieldTypeKind::Option) => base.map(|base| {
                            quote! {
                                if let Some(v) = &mut self.#name {
                                    #base
                                }
                            }
                        }),
                        None => base.map(|base| {
                            quote! {
                                {
                                    let v = &mut self.#name;
                                    #base
                                }
                            }
                        }),
                    }
                }
            }
        });

        quote! {
            fn map_values_to_null(&mut self) {
                #(#fields)*
            }
        }
    };

    // Evaluate is_all_missing method.
    let is_all_missing_method_tokens = if all_fields_are_optional_or_db_properties {
        let fields = fields_in_db.iter().map(|field| {
            let name = field.name();
            match field.field_type_kind {
                Some(FieldTypeKind::NullableOption) => {
                    quote! {
                        if !self.#name.is_missing() {
                            return false;
                        }
                    }
                }
                Some(FieldTypeKind::Option) => {
                    quote! {
                        if self.#name.is_some() {
                            return false;
                        }
                    }
                }
                None => {
                    unreachable!("Cannot generate is_all_missing for plain fields")
                }
            }
        });

        quote! {
            fn is_all_missing(&self) -> bool {
                #(#fields)*

                true
            }
        }
    } else {
        quote! {
            fn is_all_missing(&self) -> bool {
                false
            }
        }
    };

    // Evaluate normalize method.
    let normalize_method_tokens = if let Some(method_name) = &info.replace_normalize_fields {
        method_name.to_token_stream()
    } else if let Some(method_name) = &options.replace_normalize_fields {
        quote! {
            #[allow(unused_variables)]
            fn normalize_fields(&mut self) -> DBNormalizeResult {
                #method_name(self)
            }
        }
    } else {
        let normalize_field_list = fields_in_db.iter().filter_map(|field| {
            if field.attributes.skip_normalize {
                return None;
            }

            let name = field.name();
            let base = match field.attributes.inner_model {
                InnerModelKind::Data => match field.base_type_kind {
                    BaseTypeKind::Other | BaseTypeKind::Box | BaseTypeKind::DBReference => {
                        Some(quote! {
                            match v.normalize() {
                                DBNormalizeResult::NotModified => {}
                                DBNormalizeResult::Modified => {
                                    modified = true;
                                }
                                DBNormalizeResult::Removed => {
                                    modified = true;
                                    remove = true;
                                }
                            }
                        })
                    }
                    BaseTypeKind::VecDBReference => Some(quote! {
                        for i in (0..v.len()).rev() {
                            let v2 = v.get_mut(i).unwrap();

                            match v2.normalize() {
                                DBNormalizeResult::NotModified => {}
                                DBNormalizeResult::Modified => {
                                    modified = true;
                                }
                                DBNormalizeResult::Removed => {
                                    modified = true;
                                    v.remove(i);
                                }
                            }
                        }

                        if v.is_empty() {
                            modified = true;
                            remove = true;
                        }
                    }),
                    BaseTypeKind::Vec | BaseTypeKind::HashMap => Some(quote! {
                        if v.is_empty() {
                            modified = true;
                            remove = true;
                        }
                    }),
                },
                InnerModelKind::Struct | InnerModelKind::Enum => match field.base_type_kind {
                    BaseTypeKind::Other | BaseTypeKind::Box => Some(quote! {
                        match v.normalize() {
                            DBNormalizeResult::NotModified => {}
                            DBNormalizeResult::Modified => {
                                modified = true;
                            }
                            DBNormalizeResult::Removed => {
                                modified = true;
                                remove = true;
                            }
                        }
                    }),
                    BaseTypeKind::Vec => Some(quote! {
                        for i in (0..v.len()).rev() {
                            let v2 = v.get_mut(i).unwrap();

                            match v2.normalize() {
                                DBNormalizeResult::NotModified => {}
                                DBNormalizeResult::Modified => {
                                    modified = true;
                                }
                                DBNormalizeResult::Removed => {
                                    modified = true;
                                    v.remove(i);
                                }
                            }
                        }

                        if v.is_empty() {
                            remove = true;
                        }
                    }),
                    BaseTypeKind::VecDBReference => {
                        panic!("Cannot declare a VecDBReference value as Struct or Enum model")
                    }
                    BaseTypeKind::HashMap => Some(quote! {
                        v.retain(|_, v| {
                            match v.normalize() {
                                DBNormalizeResult::NotModified => true,
                                DBNormalizeResult::Modified => {
                                    modified = true;
                                    true
                                }
                                DBNormalizeResult::Removed => {
                                    modified = true;
                                    false
                                }
                            }
                        });

                        if v.is_empty() {
                            remove = true;
                        }
                    }),
                    BaseTypeKind::DBReference => {
                        panic!("Cannot declare a DBReference value as Struct or Enum model")
                    }
                },
            };

            match field.field_type_kind {
                Some(FieldTypeKind::NullableOption) => base.map(|base| {
                    quote! {
                        if let NullableOption::Value(v) = &mut self.#name {
                            let mut remove = false;
                            #base

                            if remove {
                                self.#name = NullableOption::Null;
                            }
                        }
                    }
                }),
                Some(FieldTypeKind::Option) => base.map(|base| {
                    quote! {
                        if let Some(v) = &mut self.#name {
                            let mut remove = false;
                            #base

                            if remove {
                                self.#name = None;
                            }
                        }
                    }
                }),
                None => base.map(|base| {
                    quote! {
                        {
                            let v = &mut self.#name;
                            let mut remove = false;
                            #base
                        }
                    }
                }),
            }
        });

        quote! {
            #[allow(unused_variables)]
            fn normalize_fields(&mut self) -> DBNormalizeResult {
                let mut modified = false;

                #(#normalize_field_list;)*

                if self.is_all_missing() {
                    DBNormalizeResult::Removed
                } else if modified {
                    DBNormalizeResult::Modified
                } else {
                    DBNormalizeResult::NotModified
                }
            }
        }
    };

    // Evaluate filter method.
    let filter_method_tokens = if let Some(method_name) = &info.replace_filter {
        method_name.to_token_stream()
    } else if let Some(method_name) = &options.replace_filter {
        quote! {
            #[allow(unused_variables)]
            fn filter(&mut self, filter: &Self) {
                #method_name(self, filter)
            }
        }
    } else {
        let filter_field_list = fields_in_db.iter().filter_map(|field| {
            let name = field.name();

            match field.attributes.inner_model {
                InnerModelKind::Data => match field.field_type_kind {
                    Some(FieldTypeKind::NullableOption) => Some({
                        quote! {
                            if filter.#name.is_missing() {
                                self.#name = NullableOption::Missing;
                            }
                        }
                    }),
                    Some(FieldTypeKind::Option) => Some({
                        quote! {
                            if filter.#name.is_none() {
                                self.#name = None;
                            }
                        }
                    }),
                    None => None,
                },
                InnerModelKind::Struct | InnerModelKind::Enum => {
                    let base = match field.base_type_kind {
                        BaseTypeKind::Other | BaseTypeKind::Box => Some(quote! {
                            self_field.filter(filter_field);
                        }),
                        BaseTypeKind::Vec => None,
                        BaseTypeKind::VecDBReference => {
                            panic!("Cannot declare a VecDBReference value as Struct or Enum model")
                        }
                        BaseTypeKind::HashMap => None,
                        BaseTypeKind::DBReference => {
                            panic!("Cannot declare a DBReference value as Struct or Enum model")
                        }
                    };

                    match field.field_type_kind {
                        Some(FieldTypeKind::NullableOption) => base.map(|base| {
                            quote! {
                                match &filter.#name {
                                    NullableOption::Missing => {
                                        self.#name = NullableOption::Missing;
                                    }
                                    NullableOption::Null => {}
                                    NullableOption::Value(filter_field) => {
                                        if let NullableOption::Value(self_field) = &mut self.#name {
                                            #base
                                        }
                                    }
                                }
                            }
                        }),
                        Some(FieldTypeKind::Option) => base.map(|base| {
                            quote! {
                                match &filter.#name {
                                    NullableOption::Missing => {
                                        self.#name = NullableOption::Missing;
                                    }
                                    NullableOption::Null => {}
                                    NullableOption::Value(filter_field) => {
                                        if let NullableOption::Value(self_field) = &mut self.#name {
                                            #base
                                        }
                                    }
                                }
                            }
                        }),
                        None => base.map(|base| {
                            quote! {
                                {
                                    let filter_field = &filter.#name;
                                    let self_field = &mut self.#name;
                                    #base
                               }
                            }
                        }),
                    }
                }
            }
        });

        quote! {
            #[allow(unused_variables)]
            fn filter(&mut self, filter: &Self) {
                #(#filter_field_list;)*
            }
        }
    };

    // Build result.
    Ok(quote! {
        #[async_trait]
        impl DBDocument for #document_name {
            type Collection = #collection_name;

            // GETTERS --------------------------------------------------------

            fn db_id(&self) -> Option<DBId> {
                self.db_key
                    .as_ref()
                    .map(|key| DBId::new(key.clone(), CollectionKind::#collection_kind))
            }

            fn db_key(&self) -> &Option<DBUuid> {
                &self.db_key
            }

            fn db_rev(&self) -> &Option<ArcStr> {
                &self.db_rev
            }

            fn collection() -> Arc<Self::Collection> {
                Self::Collection::instance()
            }

            #is_all_missing_method_tokens

            // SETTERS --------------------------------------------------------

            fn set_db_key(&mut self, value: Option<DBUuid>) {
                self.db_key = value;
            }

            // METHODS ----------------------------------------------------------------

            #map_values_to_null_method_tokens
            #normalize_method_tokens
            #filter_method_tokens
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn build_db_struct_field_list(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let enum_name = &info.field_enum_name;
    let visibility = info.item.visibility();

    // Evaluate fields.
    let mut enum_fields = vec![];
    let mut path_fields = vec![];

    fields_in_db.iter().for_each(|field| {
        let name_str = from_snake_case_to_pascal_case(&field.name().to_string());
        let name = format_ident!("{}", name_str, span = field.name().span());
        let db_name = &field.db_name;

        match field.attributes.inner_model {
            InnerModelKind::Data => match field.base_type_kind {
                BaseTypeKind::DBReference => {
                    let name_key = format_ident!("{}Key", name_str, span = field.name().span());
                    let key_path = format!("{}._key", db_name);
                    let db_name_key = format!("{}K", db_name);

                    enum_fields.push(quote! {
                        #[serde(rename = #db_name)]
                        #name(Option<()>),
                        #[serde(rename = #db_name_key)]
                        #name_key(Option<()>),
                    });
                    path_fields.push(quote! {
                        #enum_name::#name(_) => #db_name.into(),
                        #enum_name::#name_key(_) => #key_path.into(),
                    });
                }
                _ => {
                    enum_fields.push(quote! {
                        #[serde(rename = #db_name)]
                        #name(Option<()>),
                    });
                    path_fields.push(quote! {
                        #enum_name::#name(_) => #db_name.into(),
                    });
                }
            },
            InnerModelKind::Struct | InnerModelKind::Enum => match field.base_type_kind {
                BaseTypeKind::DBReference => {
                    let name_key = format_ident!("{}Key", name_str, span = field.name().span());
                    let key_path = format!("{}._key", db_name);
                    let db_name_key = format!("{}K", db_name);

                    enum_fields.push(quote! {
                        #[serde(rename = #db_name)]
                        #name(Option<()>),
                        #[serde(rename = #db_name_key)]
                        #name_key(Option<()>),
                    });
                    path_fields.push(quote! {
                        #enum_name::#name(_) => #db_name.into(),
                        #enum_name::#name_key(_) => #key_path.into(),
                    });
                }
                _ => {
                    let inner_type = field.inner_type.as_ref().unwrap();
                    let inner_type_name = field.get_inner_db_type_name();
                    let inner_type_enum =
                        format_ident!("{}Field", inner_type_name, span = inner_type.span());
                    let sub_pattern = format!("{}.{{}}", db_name);

                    enum_fields.push(quote! {
                        #[serde(rename = #db_name)]
                        #name(Option<#inner_type_enum>),
                    });
                    path_fields.push(quote! {
                        #enum_name::#name(v) => if let Some(v) = v {
                            format!(#sub_pattern, v.path()).into()
                        } else {
                            #db_name.into()
                        },
                    });
                }
            },
        }
    });

    // Check it is empty.
    if enum_fields.is_empty() {
        return Ok(quote! {});
    }

    // Build result.
    Ok(quote! {
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(tag = "T", content = "V")]
        #visibility enum #enum_name {
            #(#enum_fields)*
        }

        impl #enum_name {
            pub fn path(&self) -> Cow<'static, str> {
                match self {
                    #(#path_fields)*
                }
            }
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_sync_impl(options: &ModelOptions, info: &ModelInfo) -> Result<TokenStream, syn::Error> {
    let document_name = &info.document_name;
    let config_collection_key_method = if let Some(sync_collection_key_method) =
        &options.sync_collection_key_method
    {
        sync_collection_key_method.clone()
    } else {
        let document_name = from_camel_or_pascal_case_to_snake_case(&info.item.ident().to_string());

        format_ident!(
            "{}_mutex_key",
            document_name,
            span = info.item.ident().span()
        )
    };

    // Evaluate method content.
    let collection_key_value = if options.sync_level.is_collection_active() {
        quote! {
            mutex_collection.#config_collection_key_method()
        }
    } else {
        quote! {
            unreachable!("This collection cannot be locked")
        }
    };

    // Build result.
    Ok(quote! {
        impl SynchronizedDBDocument for #document_name {
            #[allow(unused_variables)]
            fn collection_key(mutex_collection: &Arc<MutexCollection>) -> &DBUuid {
                #collection_key_value
            }
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn check_and_build_edge_db_impl(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let from_field = fields_in_db.iter().find(|field| field.db_name == "_from");
    let to_field = fields_in_db.iter().find(|field| field.db_name == "_to");

    if let (Some(from_field), Some(to_field)) = (from_field, to_field) {
        let document_name = &info.document_name;
        let from_field = from_field.name();
        let to_field = to_field.name();

        Ok(quote! {
            impl DBEdgeDocument for #document_name {
                fn db_from(&self) -> &Option<DBId> {
                    &self.#from_field
                }

                fn db_to(&self) -> &Option<DBId> {
                    &self.#to_field
                }
            }
        })
    } else {
        Ok(quote! {})
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn build_db_struct_aql_mapping_impl(
    options: &ModelOptions,
    info: &ModelInfo,
    is_sub_model: bool,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let document_name = &info.document_name;

    let include_let_steps_fields: Vec<_> = fields_in_db
        .iter()
        .filter_map(|field| {
            let name = field.name();

            match field.attributes.inner_model {
                InnerModelKind::Data => {
                    let base = match field.base_type_kind {
                        BaseTypeKind::Other
                        | BaseTypeKind::Box
                        | BaseTypeKind::Vec
                        | BaseTypeKind::HashMap => None,
                        BaseTypeKind::VecDBReference | BaseTypeKind::DBReference => Some(quote! {
                            v.include_let_steps(aql, path, next_id);
                        }),
                    };

                    match field.field_type_kind {
                        Some(FieldTypeKind::Option) => base.map(|base| {
                            quote! {
                                if let Some(v) = &self.#name {
                                    #base
                                }
                            }
                        }),
                        Some(FieldTypeKind::NullableOption) => base.map(|base| {
                            quote! {
                                if let NullableOption::Value(v) = &self.#name {
                                    #base
                                }
                            }
                        }),
                        None => base.map(|base| {
                            quote! {
                                let v = &self.#name;
                                #base
                            }
                        }),
                    }
                }
                InnerModelKind::Struct | InnerModelKind::Enum => match field.field_type_kind {
                    Some(FieldTypeKind::Option) => Some(quote! {
                        if let Some(v) = &self.#name {
                            v.include_let_steps(aql, path, next_id);
                        }
                    }),
                    Some(FieldTypeKind::NullableOption) => Some(quote! {
                        if let NullableOption::Value(v) = &self.#name {
                            v.include_let_steps(aql, path, next_id);
                        }
                    }),
                    None => Some(quote! {
                        self.#name.include_let_steps(aql, path, next_id);
                    }),
                },
            }
        })
        .collect();

    let include_let_steps_method = if !include_let_steps_fields.is_empty() {
        quote! {
            #[allow(unused_variables)]
            fn include_let_steps(&self, aql: &mut AqlBuilder, path: &str, next_id: &mut usize) {
                #(#include_let_steps_fields)*
            }
        }
    } else {
        quote! {}
    };

    let map_to_json_fields = fields_in_db.iter().map(|field| {
        let db_name = &field.db_name;
        let name = field.name();

        match field.attributes.inner_model {
            InnerModelKind::Data => {
                let pattern1 = format!("{}:", db_name);
                let pattern2 = format!("{{}}.{}", db_name);
                let pattern3 = format!(".{},", db_name);

                let base = match field.base_type_kind {
                    BaseTypeKind::Other
                    | BaseTypeKind::Box
                    | BaseTypeKind::Vec
                    | BaseTypeKind::HashMap => {
                        quote! {
                            buffer.write_all(path.as_bytes()).unwrap();
                            buffer.write_all(#pattern3.as_bytes()).unwrap();
                        }
                    }
                    BaseTypeKind::VecDBReference | BaseTypeKind::DBReference => {
                        quote! {
                            let sub_path = format!(#pattern2, path);
                            v.map_to_json(buffer, sub_path.as_str(), next_id);
                            buffer.write_all(b",").unwrap();
                        }
                    }
                };

                match field.field_type_kind {
                    Some(FieldTypeKind::Option) => {
                        quote! {
                            if let Some(v) = &self.#name {
                                buffer.write_all(#pattern1.as_bytes()).unwrap();
                                #base
                            }
                        }
                    }
                    Some(FieldTypeKind::NullableOption) => {
                        quote! {
                            match &self.#name {
                                NullableOption::Value(v) => {
                                    buffer.write_all(#pattern1.as_bytes()).unwrap();
                                    #base
                                }
                                NullableOption::Null => {
                                    buffer.write_all(#pattern1.as_bytes()).unwrap();
                                    buffer.write_all(path.as_bytes()).unwrap();
                                    buffer.write_all(#pattern3.as_bytes()).unwrap();
                                }
                                NullableOption::Missing => {}
                            }
                        }
                    }
                    None => {
                        quote! {
                            let v = &self.#name;
                            buffer.write_all(#pattern1.as_bytes()).unwrap();
                            #base
                        }
                    }
                }
            }
            InnerModelKind::Struct | InnerModelKind::Enum => {
                let pattern1 = format!("{}:", db_name);
                let pattern2 = format!("{{}}.{}", db_name);

                match field.field_type_kind {
                    Some(FieldTypeKind::Option) => {
                        quote! {
                            if let Some(v) = &self.#name {
                                buffer.write_all(#pattern1.as_bytes()).unwrap();

                                let sub_path = format!(#pattern2, path);
                                v.map_to_json(buffer, sub_path.as_str(), next_id);
                                buffer.write_all(b",").unwrap();
                            }
                        }
                    }
                    Some(FieldTypeKind::NullableOption) => {
                        let pattern3 = format!(".{},", db_name);

                        quote! {
                            match &self.#name {
                                NullableOption::Value(v) => {
                                    buffer.write_all(#pattern1.as_bytes()).unwrap();

                                    let sub_path = format!(#pattern2, path);
                                    v.map_to_json(buffer, sub_path.as_str(), next_id);
                                    buffer.write_all(b",").unwrap();
                                }
                                NullableOption::Null => {
                                    buffer.write_all(#pattern1.as_bytes()).unwrap();
                                    buffer.write_all(path.as_bytes()).unwrap();
                                    buffer.write_all(#pattern3.as_bytes()).unwrap();
                                }
                                NullableOption::Missing => {}
                            }
                        }
                    }
                    None => {
                        quote! {
                            buffer.write_all(#pattern1.as_bytes()).unwrap();

                            let sub_path = format!(#pattern2, path);
                            self.#name.map_to_json(buffer, sub_path.as_str(), next_id);
                            buffer.write_all(b",").unwrap();
                        }
                    }
                }
            }
        }
    });

    let lock_field = if options.sync_level.is_document_active() {
        let db_name = MUTEX_FIELD_DB_NAME;
        let name = format_ident!("{}", MUTEX_FIELD_NAME);

        let pattern1 = format!("{}:", db_name);
        let pattern2 = format!("{{}}.{}", db_name);
        let pattern3 = format!(".{},", db_name);

        quote! {
            match &self.#name {
                NullableOption::Value(v) => {
                    buffer.write_all(#pattern1.as_bytes()).unwrap();

                    let sub_path = format!(#pattern2, path);
                    v.map_to_json(buffer, sub_path.as_str(), next_id);
                    buffer.write_all(b",").unwrap();
                }
                NullableOption::Null => {
                    buffer.write_all(#pattern1.as_bytes()).unwrap();
                    buffer.write_all(path.as_bytes()).unwrap();
                    buffer.write_all(#pattern3.as_bytes()).unwrap();
                }
                NullableOption::Missing => {}
            }
        }
    } else {
        quote! {}
    };

    let from_field = fields_in_db.iter().any(|field| field.db_name == "_from");
    let to_field = fields_in_db.iter().any(|field| field.db_name == "_to");

    let from_to_fields = if from_field && to_field {
        quote! {
            buffer.write_all(b"_from:").unwrap();
            buffer.write_all(path.as_bytes()).unwrap();
            buffer.write_all(b"._from,").unwrap();

            buffer.write_all(b"_to:").unwrap();
            buffer.write_all(path.as_bytes()).unwrap();
            buffer.write_all(b"._to,").unwrap();
        }
    } else {
        quote! {}
    };

    let model_fields = if !is_sub_model {
        quote! {
            // Include always _key and _rev.
            buffer.write_all(b"_key:").unwrap();
            buffer.write_all(path.as_bytes()).unwrap();
            buffer.write_all(b"._key,").unwrap();

            buffer.write_all(b"_rev:").unwrap();
            buffer.write_all(path.as_bytes()).unwrap();
            buffer.write_all(b"._rev,").unwrap();

            // Include always _from and _to also.
            #from_to_fields

            #lock_field
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        impl#generics AQLMapping for #document_name#generics {
            #include_let_steps_method

            #[allow(unused_variables)]
            fn map_to_json(&self, buffer: &mut Vec<u8>, path: &str, next_id: &mut usize) {
                use std::io::Write;
                buffer.write_all(b"{").unwrap();

                #model_fields

                #(#map_to_json_fields)*

                buffer.write_all(b"}").unwrap();
            }
        }
    })
}
