use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use crate::data::{
    BaseTypeKind, FieldInfo, FieldTypeKind, InnerModelKind, ModelInfo, ModelOptions,
};
use crate::model_builders::{build_db_struct_aql_mapping_impl, build_db_struct_field_list};

pub fn build_sub_db_struct_model(
    options: &ModelOptions,
    info: &ModelInfo,
) -> Result<TokenStream, syn::Error> {
    let fields_in_db: Vec<_> = info.fields_in_db().collect();
    let struct_tokens = build_struct(options, info, &fields_in_db)?;
    let impl_tokens = if !options.skip_impl {
        build_impl(options, info, &fields_in_db)?
    } else {
        quote! {}
    };

    let field_list_tokens = if !options.skip_fields {
        build_db_struct_field_list(options, info, &fields_in_db)?
    } else {
        quote! {}
    };

    let aql_mapping_impl_tokens =
        build_db_struct_aql_mapping_impl(options, info, true, &fields_in_db)?;

    // Build result.
    Ok(quote! {
        #struct_tokens
        #impl_tokens
        #field_list_tokens
        #aql_mapping_impl_tokens
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_struct(
    _options: &ModelOptions,
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

    // Evaluate fields.
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

    // Evaluate is_all_missing_method_tokens method.
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
            pub fn is_all_missing(&self) -> bool {
                #(#fields)*

                true
            }
        }
    } else {
        quote! {
            pub fn is_all_missing(&self) -> bool {
                false
            }
        }
    };

    // Evaluate is_all_null_method_tokens method.
    let is_all_null_method_tokens = if all_fields_are_optional_or_db_properties {
        let fields = fields_in_db.iter().map(|field| {
            let name = field.name();
            match field.field_type_kind {
                Some(FieldTypeKind::NullableOption) => {
                    quote! {
                        if !self.#name.is_null() {
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
                    unreachable!("Cannot generate is_all_null for plain fields")
                }
            }
        });

        quote! {
            pub fn is_all_null(&self) -> bool {
                #(#fields)*

                true
            }
        }
    } else {
        quote! {
            pub fn is_all_null(&self) -> bool {
                false
            }
        }
    };

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

        quote! {
            #[allow(clippy::needless_update)]
            pub fn all_null() -> Self {
                Self {
                    #(#null_field_list,)*
                    ..Default::default()
                }
            }
        }
    } else {
        quote! {}
    };

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
            pub fn map_values_to_null(&mut self) {
                #(#fields)*
            }
        }
    };

    // Evaluate filter method.
    let filter_method_tokens = if let Some(method_name) = &info.replace_filter {
        method_name.to_token_stream()
    } else if let Some(method_name) = &options.replace_filter {
        quote! {
            #[allow(unused_variables)]
            pub fn filter(&mut self, filter: &Self) {
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
            pub fn filter(&mut self, filter: &Self) {
                #(#filter_field_list;)*
            }
        }
    };

    // Evaluate normalize method.
    let normalize_or_remove_method_tokens = if let Some(method_name) = &info.replace_normalize {
        method_name.to_token_stream()
    } else if let Some(method_name) = &options.replace_normalize {
        quote! {
            fn normalize(&mut self) -> DBNormalizeResult {
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
            fn normalize(&mut self) -> DBNormalizeResult {
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

    // Build result.
    Ok(quote! {
        impl#generics #document_name#generics {
            #is_all_missing_method_tokens
            #is_all_null_method_tokens
            #all_null_method_tokens
            #map_values_to_null_method_tokens
            #filter_method_tokens
        }

        impl#generics DBNormalize for #document_name#generics {
            #normalize_or_remove_method_tokens
        }
    })
}
