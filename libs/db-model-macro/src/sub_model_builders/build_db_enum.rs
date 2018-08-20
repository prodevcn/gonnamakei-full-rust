use proc_macro2::TokenStream;
use quote::quote;
use quote::{format_ident, ToTokens};
use syn::spanned::Spanned;

use crate::data::{
    BaseTypeKind, FieldInfo, FieldTypeKind, InnerModelKind, ModelInfo, ModelOptions,
};
use crate::utils::from_camel_or_pascal_case_to_snake_case;

pub fn build_sub_db_enum_model(
    options: &ModelOptions,
    info: &ModelInfo,
) -> Result<TokenStream, syn::Error> {
    let fields_in_db: Vec<_> = info.fields_in_db().collect();
    let enum_tokens = build_enum(options, info, &fields_in_db)?;
    let impl_tokens = if !options.skip_impl {
        build_impl(options, info, &fields_in_db)?
    } else {
        quote! {}
    };

    let field_list_tokens = if !options.skip_fields {
        build_field_list(options, info, &fields_in_db)?
    } else {
        quote! {}
    };

    let aql_mapping_impl_tokens = build_aql_mapping_impl(options, info, &fields_in_db)?;

    // Build result.
    Ok(quote! {
        #enum_tokens
        #impl_tokens
        #field_list_tokens
        #aql_mapping_impl_tokens
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_enum(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let attribute_list = &info.item_attributes.db;
    let visibility = info.item.visibility();
    let generics = info.item.generics();
    let document_name = &info.document_name;

    let all_variants_are_unit = info.check_all_db_variants_are_unit();

    // Evaluate simple attributes.
    let simple_attributes = if all_variants_are_unit {
        quote! {#[derive(Copy, Eq, PartialEq, Hash)]}
    } else {
        quote! {}
    };

    // Evaluate fields.
    let field_list = fields_in_db.iter().map(|field| {
        let attribute_list = &field.attributes.db;
        let name = field.name();
        let db_name = &field.db_name;

        if let Some(inner_type) = &field.inner_type {
            quote! {
                #(#attribute_list)*
                #[serde(rename = #db_name)]
                #name(#inner_type),
            }
        } else if !all_variants_are_unit {
            quote! {
                #(#attribute_list)*
                #[serde(rename = #db_name)]
                #name(Option<()>),
            }
        } else {
            quote! {
                #(#attribute_list)*
                #[serde(rename = #db_name)]
                #name,
            }
        }
    });

    // Process serde tag.
    let serde_tag_attribute = if !all_variants_are_unit {
        quote! {
            #[serde(tag = "T", content = "V")]
        }
    } else {
        quote! {}
    };

    // Build result.
    Ok(quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #simple_attributes
        #[serde(rename_all = "camelCase")]
        #serde_tag_attribute
        #(#attribute_list)*
        #visibility enum #document_name#generics {
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

    let all_variants_are_unit = info.check_all_db_variants_are_unit();

    // Evaluate is * method.
    let is_method_list = fields_in_db.iter().map(|field| {
        let name = field.name();
        let fn_name = from_camel_or_pascal_case_to_snake_case(&name.to_string());
        let fn_name = format_ident!("is_{}", fn_name, span = name.span());

        if field.inner_type.is_some() || !all_variants_are_unit {
            quote! {
                pub fn #fn_name(&self) -> bool {
                    matches!(self, #document_name::#name(_))
                }
            }
        } else {
            quote! {
                pub fn #fn_name(&self) -> bool {
                    matches!(self, #document_name::#name)
                }
            }
        }
    });

    // Evaluate map_values_to_null_method method.
    let map_values_to_null_method_tokens = if all_variants_are_unit {
        quote! {
            pub fn map_values_to_null(&mut self) { }
        }
    } else {
        let fields = fields_in_db.iter().map(|field| {
            let name = field.name();

            if field.inner_type.is_none() {
                quote! {
                    #document_name::#name(_) => {}
                }
            } else {
                match field.attributes.inner_model {
                    InnerModelKind::Struct | InnerModelKind::Enum => quote! {
                        #document_name::#name(v) => v.map_values_to_null(),
                    },
                    InnerModelKind::Data => quote! {
                        #document_name::#name(_) => {}
                    },
                }
            }
        });

        quote! {
            pub fn map_values_to_null(&mut self) {
                match self {
                    #(#fields)*
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
            pub fn filter(&mut self, filter: &Self) {
                #method_name(self, filter)
            }
        }
    } else if !all_variants_are_unit {
        let filter_field_list = fields_in_db.iter().map(|field| {
            let name = field.name();
            if all_variants_are_unit {
                quote! {
                    #document_name::#name => {}
                }
            } else if field.inner_type.is_some() {
                match field.attributes.inner_model {
                    InnerModelKind::Struct | InnerModelKind::Enum => quote! {
                        #document_name::#name(v) => match filter {
                            #document_name::#name(v2) => v.filter(v2),
                            _ => {}
                        }
                    },
                    InnerModelKind::Data => quote! {
                        #document_name::#name(_) => {}
                    },
                }
            } else {
                quote! {
                    #document_name::#name(_) => {}
                }
            }
        });

        quote! {
            #[allow(unused_variables)]
            pub fn filter(&mut self, filter: &Self) {
                match self {
                    #(#filter_field_list)*
                }
            }
        }
    } else {
        quote! {
            #[allow(unused_variables)]
            pub fn filter(&mut self, filter: &Self) { }
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
    } else if !all_variants_are_unit {
        let normalize_field_list = fields_in_db.iter().filter_map(|field| {
            let name = field.name();

            if field.attributes.skip_normalize {
                return None;
            }

            if field.inner_type.is_none() {
                return Some(quote! {
                    #document_name::#name(v) =>{
                        if v.is_some() {
                            *v = None;
                            modified = true;
                        }
                    }
                });
            }

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
                        panic!("Cannot declare a DBReference value as Struct or Enum model")
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
                        #document_name::#name(v) => {
                            if let NullableOption::Value(v) = v {
                                let mut remove = false;
                                #base

                                if remove {
                                    *v = NullableOption::Null;
                                }
                            }
                        }
                    }
                }),
                Some(FieldTypeKind::Option) => base.map(|base| {
                    quote! {
                        #document_name::#name(v) => {
                            if let Some(v) = v {
                                let mut remove = false;
                                #base

                                if remove {
                                    *v = None;
                                }
                            }
                        }
                    }
                }),
                None => base.map(|base| {
                    quote! {
                        #document_name::#name(v) =>{
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

                match self {
                    #(#normalize_field_list,)*
                }

                if modified {
                    DBNormalizeResult::Modified
                } else {
                    DBNormalizeResult::NotModified
                }
            }
        }
    } else {
        quote! {
            fn normalize(&mut self) -> DBNormalizeResult {
                    DBNormalizeResult::NotModified
            }
        }
    };

    // Build result.
    Ok(quote! {
        impl#generics #document_name#generics {
            #(#is_method_list)*
            #map_values_to_null_method_tokens
            #filter_method_tokens

            pub fn is_all_missing(&self) -> bool {
                false
            }

            pub fn is_all_null(&self) -> bool {
                false
            }
        }

        impl#generics DBNormalize for #document_name#generics {
            #normalize_or_remove_method_tokens
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_field_list(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let document_name = &info.document_name;
    let enum_name = &info.field_enum_name;
    let visibility = info.item.visibility();

    let all_variants_are_unit = info.check_all_db_variants_are_unit();

    // Evaluate fields.
    let mut field_names = vec![];
    let mut field_paths = vec![];
    let mut get_variant_list = vec![];

    if all_variants_are_unit {
        fields_in_db.iter().for_each(|field| {
            let name = field.name();
            let db_name = &field.db_name;

            field_names.push(quote! {
                #[serde(rename = #db_name)]
                #name(Option<()>),
            });
            field_paths.push(quote! {
                #enum_name::#name(_) => #db_name.into(),
            });
            get_variant_list.push(quote! {
                #document_name::#name => #enum_name::#name(None),
            });
        });
    } else {
        fields_in_db.iter().for_each(|field| {
            let name = field.name();
            let db_name = &field.db_name;

            match field.attributes.inner_model {
                InnerModelKind::Data => {
                    field_names.push(quote! {
                        #[serde(rename = #db_name)]
                        #name(Option<()>),
                    });
                    field_paths.push(quote! {
                        #enum_name::#name(_) => #db_name.into(),
                    });
                    get_variant_list.push(quote! {
                        #document_name::#name(_) => #enum_name::#name(None),
                    });
                }
                InnerModelKind::Struct => {
                    let inner_type = field.inner_type.as_ref().unwrap();
                    let inner_type_name = field.get_inner_db_type_name();
                    let inner_type_enum =
                        format_ident!("{}Field", inner_type_name, span = inner_type.span());

                    field_names.push(quote! {
                        #[serde(rename = #db_name)]
                        #name(Option<#inner_type_enum>),
                    });
                    field_paths.push(quote! {
                        #enum_name::#name(v) => if let Some(v) = v {
                            format!("V.{}", v.path()).into()
                        } else {
                            #db_name.into()
                        },
                    });
                    get_variant_list.push(quote! {
                        #document_name::#name(_) => #enum_name::#name(None),
                    });
                }
                InnerModelKind::Enum => {
                    let inner_type = field.inner_type.as_ref().unwrap();
                    let inner_type_name = field.get_inner_db_type_name();
                    let inner_type_enum =
                        format_ident!("{}Field", inner_type_name, span = inner_type.span());

                    field_names.push(quote! {
                        #[serde(rename = #db_name)]
                        #name(Option<#inner_type_enum>),
                    });
                    field_paths.push(quote! {
                        #enum_name::#name(v) => if let Some(v) = v {
                            format!("V.{}", v.path()).into()
                        } else {
                            #db_name.into()
                        },
                    });
                    get_variant_list.push(quote! {
                        #document_name::#name(v) => #enum_name::#name(Some(v.variant())),
                    });
                }
            }
        });
    }

    // Build result.
    Ok(quote! {
        impl#generics #document_name#generics {
            pub fn variant(&self) -> #enum_name {
                match self {
                    #(#get_variant_list)*
                }
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(tag = "T", content = "V")]
        #visibility enum #enum_name {
            #(#field_names)*
        }

        impl #enum_name {
            pub fn path(&self) -> Cow<'static, str> {
                match self {
                    #(#field_paths)*
                }
            }
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_aql_mapping_impl(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_db: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let document_name = &info.document_name;

    let all_variants_are_unit = info.check_all_db_variants_are_unit();

    let include_let_steps_method = if !all_variants_are_unit {
        let fields = fields_in_db.iter().map(|field| {
            let name = field.name();

            match field.attributes.inner_model {
                InnerModelKind::Struct | InnerModelKind::Enum => if field.inner_type.is_some() {
                    quote! {
                        #document_name::#name(v) => v.include_let_steps(aql, sub_path.as_str(), next_id),
                    }
                } else {
                    quote! {
                        #document_name::#name(_) => {}
                    }
                },
                InnerModelKind::Data => quote! {
                    #document_name::#name(_) => {}
                },
            }
        });

        quote! {
            #[allow(unused_variables)]
            fn include_let_steps(&self, aql: &mut AqlBuilder, path: &str, next_id: &mut usize) {
                let sub_path = format!("{}.V", path);

                match self {
                    #(#fields)*
                }
            }
        }
    } else {
        quote! {}
    };

    let map_to_json_method = if !all_variants_are_unit {
        let fields = fields_in_db.iter().map(|field| {
            let name = field.name();

            match field.attributes.inner_model {
                InnerModelKind::Data => quote! {
                    #document_name::#name(_) => {
                        buffer.write_all(b"null").unwrap();
                    }
                },
                InnerModelKind::Struct | InnerModelKind::Enum => if field.inner_type.is_some() {
                    quote! {
                        #document_name::#name(v) => v.map_to_json(buffer, sub_path.as_str(), next_id),
                    }
                } else {
                    quote! {
                        #document_name::#name(_) => {
                            buffer.write_all(b"null").unwrap();
                        }
                    }
                },
            }
        });

        quote! {
            #[allow(unused_variables)]
            fn map_to_json(&self, buffer: &mut Vec<u8>, path: &str, next_id: &mut usize) {
                use std::io::Write;
                let sub_path = format!("{}.V", path);

                buffer.write_all(b"{T:null,V:").unwrap();

                match self {
                    #(#fields)*
                }

                buffer.write_all(b"}").unwrap();
            }
        }
    } else {
        quote! {
            #[allow(unused_variables)]
            fn map_to_json(&self, buffer: &mut Vec<u8>, path: &str, next_id: &mut usize) {
                use std::io::Write;
                buffer.write_all(b"null").unwrap();
            }
        }
    };

    Ok(quote! {
        impl#generics AQLMapping for #document_name#generics {
            #include_let_steps_method
            #map_to_json_method
        }
    })
}
