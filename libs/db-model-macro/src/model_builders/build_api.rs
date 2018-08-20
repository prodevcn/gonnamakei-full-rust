use proc_macro2::TokenStream;
use quote::format_ident;
use quote::{quote, ToTokens};

use crate::data::{
    BaseTypeKind, FieldInfo, FieldTypeKind, InnerModelKind, ModelInfo, ModelOptions,
};
use crate::utils::from_snake_case_to_pascal_case;

pub fn build_api_model(
    options: &ModelOptions,
    info: &ModelInfo,
) -> Result<TokenStream, syn::Error> {
    let fields_in_api: Vec<_> = info.fields_in_api().collect();
    let struct_tokens = build_api_struct(options, info, false, &fields_in_api)?;
    let from_to_tokens = build_from_to(options, info, false, &fields_in_api)?;
    let api_fields_tokens = build_api_fields(options, info, false, &fields_in_api)?;

    let impl_tokens = if !options.skip_impl {
        build_api_document_impl(options, info, &fields_in_api)?
    } else {
        quote! {}
    };

    let paginated_tokens = if options.api_paginated {
        build_paginated(options, info, &fields_in_api)?
    } else {
        quote! {}
    };

    let sensible_info_impl_tokens =
        build_api_struct_sensible_info_impl(options, info, false, &fields_in_api)?;

    // Build result.
    Ok(quote! {
        #struct_tokens
        #from_to_tokens
        #api_fields_tokens
        #impl_tokens
        #paginated_tokens
        #sensible_info_impl_tokens
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn build_api_struct(
    _options: &ModelOptions,
    info: &ModelInfo,
    is_sub_model: bool,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let attribute_list = &info.item_attributes.api;
    let visibility = info.item.visibility();
    let generics = info.item.generics();
    let api_document_name = &info.api_document_name;

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
    let field_list = fields_in_api.iter().map(|field| {
        let node = field.node.as_field().unwrap();
        let visibility = &node.vis;
        let attribute_list = &field.attributes.api;
        let name = field.name();
        let field_type = field.build_api_field_type();

        quote! {
            #(#attribute_list)*
            #visibility #name: #field_type,
        }
    });

    // Id field.
    let id_field = if !is_sub_model {
        quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id: Option<DBUuid>,
        }
    } else {
        quote! {}
    };

    // Build result.
    Ok(quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #default_attribute
        #(#attribute_list)*
        #visibility struct #api_document_name#generics {
            #id_field

            #(#field_list)*
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn build_from_to(
    _options: &ModelOptions,
    info: &ModelInfo,
    is_sub_model: bool,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let document_name = &info.document_name;
    let api_document_name = &info.api_document_name;

    let all_fields_are_optional_or_db_properties =
        info.check_all_db_fields_are_optional_or_properties();

    // Evaluate fields.
    let to_api_field_list = fields_in_api.iter().filter_map(|field| {
        let name = field.name();

        if field.attributes.skip_in_api || field.attributes.skip_in_db {
            return None;
        }

        let apply_into = field.attributes.api_inner_type.is_some();

        let base = match field.base_type_kind {
            BaseTypeKind::Other => {
                if apply_into {
                    quote! {
                        v.into()
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::Box => {
                if apply_into {
                    quote! {
                        Box::new((*v).into())
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::Vec => {
                if apply_into {
                    quote! {
                        v.into_iter().map(|v| v.into()).collect()
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::VecDBReference => {
                if apply_into {
                    quote! {
                        v.into_iter().map(|v| v.map_to_api(|v| Box::new((*v).into()))).collect()
                    }
                } else {
                    quote! {
                        v.into_iter().map(|v| v.map_to_api(|v| Box::new(v))).collect()
                    }
                }
            }
            BaseTypeKind::HashMap => {
                if apply_into {
                    quote! {
                        v.into_iter().map(|(k, v)| (k, v.into())).collect()
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::DBReference => {
                if apply_into {
                    quote! {
                        v.map_to_api(|v| Box::new((*v).into()))
                    }
                } else {
                    quote! {
                        v.map_to_api(|v| Box::new(v))
                    }
                }
            }
        };

        let result = match field.field_type_kind {
            Some(FieldTypeKind::NullableOption) | Some(FieldTypeKind::Option) => {
                if apply_into {
                    quote! {
                        #name: value.#name.map(|v| #base),
                    }
                } else {
                    quote! {
                        #name: {
                            let v = value.#name;
                            #base
                        },
                    }
                }
            }
            None => quote! {
                #name: {
                    let v = value.#name;
                    #base
                },
            },
        };

        Some(result)
    });

    let to_db_field_list = fields_in_api.iter().filter_map(|field| {
        let name = field.name();

        if field.attributes.skip_in_api || field.attributes.skip_in_db {
            return None;
        }

        let apply_into = field.attributes.api_inner_type.is_some();

        let base = match field.base_type_kind {
            BaseTypeKind::Other => {
                if apply_into {
                    quote! {
                        v.into()
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::Box => {
                if apply_into {
                    quote! {
                        Box::new((*v).into())
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::Vec => {
                if apply_into {
                    quote! {
                        v.into_iter().map(|v| v.into()).collect()
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::VecDBReference => {
                if apply_into {
                    quote! {
                        v.into_iter().map(|v| v.map_to_db(|v| Box::new((*v).into()))).collect()
                    }
                } else {
                    quote! {
                        v.into_iter().map(|v| v.map_to_db(|v| Box::new(v))).collect()
                    }
                }
            }
            BaseTypeKind::HashMap => {
                if apply_into {
                    quote! {
                        v.into_iter().map(|(k, v)| (k, v.into())).collect()
                    }
                } else {
                    quote! {
                        v
                    }
                }
            }
            BaseTypeKind::DBReference => {
                if apply_into {
                    quote! {
                        v.map_to_db(|v| Box::new((*v).into()))
                    }
                } else {
                    quote! {
                        v.map_to_db(|v| Box::new(v))
                    }
                }
            }
        };

        let result = match field.field_type_kind {
            Some(FieldTypeKind::NullableOption) | Some(FieldTypeKind::Option) => {
                if apply_into {
                    quote! {
                        #name: value.#name.map(|v| #base),
                    }
                } else {
                    quote! {
                        #name: {
                            let v = value.#name;
                            #base
                        },
                    }
                }
            }
            None => quote! {
                #name: {
                    let v = value.#name;
                    #base
                },
            },
        };

        Some(result)
    });

    let replace_api_from_db_to_api_method = info
        .replace_api_from_db_to_api
        .map(|v| quote! { #v })
        .unwrap_or_else(|| quote! {});
    let replace_api_from_api_to_db_method = info
        .replace_api_from_api_to_db
        .map(|v| quote! { #v })
        .unwrap_or_else(|| quote! {});

    let replace_api_from_db_to_api_call = info
        .replace_api_from_db_to_api
        .map(|v| {
            let ident = &v.sig.ident;
            quote! { #ident(&value, &mut result) }
        })
        .unwrap_or_else(|| quote! {});
    let replace_api_from_api_to_db_call = info
        .replace_api_from_api_to_db
        .map(|v| {
            let ident = &v.sig.ident;
            quote! { #ident(&value, &mut result) }
        })
        .unwrap_or_else(|| quote! {});

    let (to_api_id_field, to_db_key_field) = if !is_sub_model {
        (
            quote! {
                id: value.db_key,
            },
            quote! {
                db_key: value.id,
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    // Evaluate default fields.
    let default_rest = if all_fields_are_optional_or_db_properties {
        quote! { ..Default::default() }
    } else {
        quote! {}
    };

    // Build result.
    Ok(quote! {
        impl#generics From<#document_name#generics> for #api_document_name#generics {
            #[allow(clippy::needless_update)]
            fn from(value: #document_name#generics) -> Self {
                 let mut result = Self {
                    #to_api_id_field
                    #(#to_api_field_list)*
                    #default_rest
                };

                #replace_api_from_db_to_api_call

                result
            }
        }

        impl#generics From<#api_document_name#generics> for #document_name#generics {
            #[allow(clippy::needless_update)]
            fn from(value: #api_document_name#generics) -> Self {
                let mut result = Self {
                    #to_db_key_field
                    #(#to_db_field_list)*
                    #default_rest
                };

                #replace_api_from_api_to_db_call

                result
            }
        }

        #replace_api_from_db_to_api_method
        #replace_api_from_api_to_db_method
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn build_api_fields(
    _options: &ModelOptions,
    info: &ModelInfo,
    is_sub_model: bool,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let visibility = info.item.visibility();
    let api_field_enum_name = &info.api_field_enum_name;

    // Evaluate fields.
    let mut enum_fields = vec![];
    let mut path_fields = vec![];

    fields_in_api.iter().for_each(|field| {
        let name_str = from_snake_case_to_pascal_case(&field.name().to_string());
        let name = format_ident!("{}", name_str, span = field.name().span());
        let db_name = &field.db_name;

        match field.attributes.inner_model {
            InnerModelKind::Struct => match field.base_type_kind {
                BaseTypeKind::DBReference => {
                    let key_path = format!("{}._key", db_name);

                    enum_fields.push(quote! {
                        #name,
                    });
                    path_fields.push(quote! {
                        #api_field_enum_name::#name => #key_path.into(),
                    });
                }
                _ => {
                    let inner_api_type = field.attributes.api_inner_type.as_ref();
                    let inner_api_type_name = inner_api_type
                        .map(|v| v.to_token_stream().to_string())
                        .unwrap_or_else(|| field.get_inner_db_type_name());
                    let inner_api_type_enum = format_ident!("{}Field", inner_api_type_name);
                    let sub_pattern = format!("{}.{{}}", db_name);

                    enum_fields.push(quote! {
                        #name(Option<#inner_api_type_enum>),
                    });
                    path_fields.push(quote! {
                        #api_field_enum_name::#name(v) => if let Some(v) = v {
                            format!(#sub_pattern, v.path()).into()
                        } else {
                            #db_name.into()
                        }
                    });
                }
            },
            InnerModelKind::Data | InnerModelKind::Enum => match field.base_type_kind {
                BaseTypeKind::DBReference => {
                    let key_path = format!("{}._key", db_name);

                    enum_fields.push(quote! {
                        #name,
                    });
                    path_fields.push(quote! {
                        #api_field_enum_name::#name => #key_path.into(),
                    });
                }
                _ => {
                    enum_fields.push(quote! {
                        #name,
                    });
                    path_fields.push(quote! {
                        #api_field_enum_name::#name => #db_name.into(),
                    });
                }
            },
        }
    });

    // Check it is empty.
    if enum_fields.is_empty() {
        return Ok(quote! {});
    }

    // Id field.
    let (id_field, id_field_path) = if !is_sub_model {
        (
            quote! {
                Id,
            },
            quote! {
                #api_field_enum_name::Id => "_key".into(),
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    // Build result.
    Ok(quote! {
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(tag = "T", content = "V")]
        #visibility enum #api_field_enum_name {
            #id_field
            #(#enum_fields)*
        }

        impl #api_field_enum_name {
            pub fn path(&self) -> Cow<'static, str> {
                match self {
                    #id_field_path
                    #(#path_fields)*
                }
            }
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_api_document_impl(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let api_document_name = &info.api_document_name;

    let all_fields_are_optional_or_db_properties =
        info.check_all_api_fields_are_optional_or_properties();

    // Evaluate is_all_missing method.
    let is_all_missing_method_tokens = if all_fields_are_optional_or_db_properties {
        let fields = fields_in_api.iter().map(|field| {
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

    // Build result.
    Ok(quote! {
        impl APIDocument for #api_document_name {
            // GETTERS --------------------------------------------------------

            fn id(&self) -> &Option<DBUuid> {
                &self.id
            }

            #is_all_missing_method_tokens
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_paginated(
    options: &ModelOptions,
    info: &ModelInfo,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let db_document_name = &info.document_name;
    let generics = info.item.generics();
    let api_document_name = &info.api_document_name;
    let api_field_enum_name = &info.api_field_enum_name;

    // Evaluate fields.
    let map_to_null_fields = fields_in_api.iter().filter_map(|field| {
        let name = field.name();

        if field.attributes.api_skip_map_to_null {
            return None;
        }

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

    let paginated_fields = if !options.skip_fields {
        build_paginated_fields(options, info, fields_in_api)?
    } else {
        quote! {}
    };

    // Build result.
    Ok(quote! {
        impl#generics PaginatedDocument for #api_document_name#generics {
            type Field = #api_field_enum_name;
            type DBDocument = #db_document_name;

            fn map_values_to_null(&mut self) {
                #(#map_to_null_fields)*
            }
        }

        #paginated_fields
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_paginated_fields(
    options: &ModelOptions,
    info: &ModelInfo,
    _fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let api_document_name = &info.api_document_name;
    let api_field_enum_name = &info.api_field_enum_name;

    // Evaluate methods.
    let is_valid_for_sorting = if let Some(method) = &info.replace_api_is_valid_for_sorting {
        method
    } else {
        return Err(crate::errors::Error::Message("Cannot create pagination fields because the method \"is_valid_for_sorting\" is not present.\
        Please add the following signature: fn is_valid_for_sorting(&self, user_type: UserType) -> bool {}".to_string()).with_tokens(&info.file));
    };

    let is_valid_for_filtering = if let Some(method) = &info.replace_api_is_valid_for_filtering {
        method
    } else {
        return Err(crate::errors::Error::Message("Cannot create pagination fields because the method \"is_valid_for_filtering\" is not present.\
        Please add the following signature: fn is_valid_for_filtering(&self, user_type: UserType) -> bool {}".to_string()).with_tokens(&info.file));
    };

    // Evaluate rows per page constants.
    let min_rows_per_page = if let Some(method) = &options.api_min_rows_per_page {
        method
    } else {
        return Err(crate::errors::Error::Message("Cannot create pagination fields because the attribute \"api_min_rows_per_page\" is missing.\
        Please add the attribute: #![api_min_rows_per_page = \"...\"]".to_string()).with_tokens(&info.file));
    };

    let max_rows_per_page = if let Some(method) = &options.api_max_rows_per_page {
        method
    } else {
        return Err(crate::errors::Error::Message("Cannot create pagination fields because the attribute \"api_max_rows_per_page\" is missing.\
        Please add the attribute: #![api_max_rows_per_page = \"...\"]".to_string()).with_tokens(&info.file));
    };

    // Build result.
    Ok(quote! {
        impl PaginatedDocumentField for #api_field_enum_name {
            type Document = #api_document_name;

            #is_valid_for_sorting
            #is_valid_for_filtering

            fn path_to_value(&self) -> Cow<'static, str> {
                self.path()
            }

            fn min_rows_per_page() -> u64 {
                #min_rows_per_page
            }

            fn max_rows_per_page() -> u64 {
                #max_rows_per_page
            }
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn build_api_struct_sensible_info_impl(
    _options: &ModelOptions,
    info: &ModelInfo,
    _is_sub_model: bool,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let api_document_name = &info.api_document_name;

    // Evaluate fields list.
    let sensible_info_fields = fields_in_api.iter().filter_map(|field| {
        let name = field.name();

        let is_sensible = field.attributes.api_sensible_info;

        match field.attributes.inner_model {
            InnerModelKind::Data => {
                let base = match field.base_type_kind {
                    BaseTypeKind::Other | BaseTypeKind::Box |
                    BaseTypeKind::Vec | BaseTypeKind::HashMap => None,
                    BaseTypeKind::VecDBReference => {
                        Some(quote! {
                            for v in v {
                                v.and(|v| v.remove_sensible_info());
                            }
                        })
                    }
                    BaseTypeKind::DBReference => {
                        Some(quote! {
                            v.and(|v| v.remove_sensible_info())
                        })
                    }
                };

                match field.field_type_kind {
                    Some(FieldTypeKind::NullableOption) => {
                        if is_sensible {
                            Some(quote! {
                                self.#name = NullableOption::Missing;
                            })
                        } else {
                            base.map(|base| quote! {
                                if let NullableOption::Value(v) = &mut self.#name {
                                    #base
                                }
                            })
                        }
                    }
                    Some(FieldTypeKind::Option) => {
                        if is_sensible {
                            Some(quote! {
                                self.#name = None;
                            })
                        } else {
                            base.map(|base| quote! {
                                if let Some(v) = &mut self.#name {
                                    #base
                                }
                            })
                        }
                    }
                    None => {
                        if is_sensible {
                            panic!("Cannot mark field '{}' as sensible info if it is not an Option or NullableOption", name.to_string());
                        } else {
                            base.map(|base| quote! {
                            {
                                let v = &mut self.#name;
                                #base
                            }
                        })
                        }
                    }
                }
            }
            InnerModelKind::Struct | InnerModelKind::Enum => {
                let base = match field.base_type_kind {
                    BaseTypeKind::Other | BaseTypeKind::Box => {
                        quote! {
                            v.remove_sensible_info();
                        }
                    }
                    BaseTypeKind::Vec => {
                        quote! {
                            for v in v {
                                v.remove_sensible_info();
                            }
                        }
                    }
                    BaseTypeKind::HashMap => {
                        quote! {
                            for (_, v) in v {
                                v.remove_sensible_info();
                            }
                        }
                    }
                    BaseTypeKind::VecDBReference => {
                        quote! {
                            for v in v {
                                v.and(|v| v.remove_sensible_info());
                            }
                        }
                    }
                    BaseTypeKind::DBReference => {
                        quote! {
                            v.and(|v| v.remove_sensible_info())
                        }
                    }
                };

                match field.field_type_kind {
                    Some(FieldTypeKind::NullableOption) => {
                        if is_sensible {
                            Some(quote! {
                                self.#name = NullableOption::Missing;
                            })
                        } else {
                            Some(quote! {
                                if let NullableOption::Value(v) = &mut self.#name {
                                    #base
                                }
                            })
                        }
                    }
                    Some(FieldTypeKind::Option) => {
                        if is_sensible {
                            Some(quote! {
                                self.#name = None;
                            })
                        } else {
                            Some(quote! {
                                if let Some(v) = &mut self.#name {
                                    #base
                                }
                            })
                        }
                    }
                    None => {
                        if is_sensible {
                            panic!("Cannot mark field '{}' as sensible info if it is not an Option or NullableOption", name.to_string());
                        } else {
                            Some(quote! {
                                {
                                    let v = &mut self.#name;
                                    #base
                                }
                            })
                        }
                    }
                }
            }
        }
    });

    // Build result.
    Ok(quote! {
        impl#generics #api_document_name#generics {
            pub fn remove_sensible_info(&mut self) {
                #(#sensible_info_fields)*
            }
        }
    })
}
