use proc_macro2::TokenStream;
use quote::quote;
use quote::{format_ident, ToTokens};

use crate::data::{FieldInfo, InnerModelKind, ModelInfo, ModelOptions};
use crate::utils::from_camel_or_pascal_case_to_snake_case;

pub fn build_sub_api_enum_model(
    options: &ModelOptions,
    info: &ModelInfo,
) -> Result<TokenStream, syn::Error> {
    let fields_in_api: Vec<_> = info.fields_in_api().collect();
    let enum_tokens = build_enum(options, info, &fields_in_api)?;
    let impl_tokens = if !options.skip_impl {
        build_impl(options, info, &fields_in_api)?
    } else {
        quote! {}
    };
    let from_to_tokens = build_from_to(options, info, &fields_in_api)?;

    let field_list_tokens = if !options.skip_fields {
        build_field_list(options, info, &fields_in_api)?
    } else {
        quote! {}
    };

    let sensible_info_impl_tokens = build_sensible_info_impl(options, info, true, &fields_in_api)?;

    // Build result.
    Ok(quote! {
        #enum_tokens
        #impl_tokens
        #from_to_tokens
        #field_list_tokens
        #sensible_info_impl_tokens
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_enum(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let attribute_list = &info.item_attributes.api;
    let visibility = info.item.visibility();
    let generics = info.item.generics();
    let document_name = &info.api_document_name;

    let all_variants_are_unit = info.check_all_api_variants_are_unit();

    // Evaluate simple attributes.
    let simple_attributes = if all_variants_are_unit {
        quote! {#[derive(Copy, Eq, PartialEq, Hash)]}
    } else {
        quote! {}
    };

    // Evaluate fields.
    let field_list = fields_in_api.iter().map(|field| {
        let attribute_list = &field.attributes.api;
        let name = field.name();

        if field.inner_type.is_some() {
            let inner_type = field.build_api_field_type();

            quote! {
                #(#attribute_list)*
                #name(#inner_type),
            }
        } else {
            quote! {
                #(#attribute_list)*
                #name,
            }
        }
    });

    // Build result.
    Ok(quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #simple_attributes
        #[serde(rename_all = "camelCase")]
        #[serde(tag = "T", content = "V")]
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
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let document_name = &info.api_document_name;

    let all_variants_are_unit = info.check_all_api_variants_are_unit();

    // Evaluate is * method.
    let is_method_list = fields_in_api.iter().map(|field| {
        let name = field.name();
        let fn_name = from_camel_or_pascal_case_to_snake_case(&name.to_string());
        let fn_name = format_ident!("is_{}", fn_name, span = name.span());

        if field.inner_type.is_some() {
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
        let fields = fields_in_api.iter().map(|field| {
            let name = field.name();

            if field.inner_type.is_none() {
                quote! {
                    #document_name::#name => {}
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

    // Build result.
    Ok(quote! {
        impl#generics #document_name#generics {
            #(#is_method_list)*
            #map_values_to_null_method_tokens

            pub fn is_all_missing(&self) -> bool {
                false
            }

            pub fn is_all_null(&self) -> bool {
                false
            }
        }
    })
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

fn build_from_to(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let document_name = &info.document_name;
    let api_document_name = &info.api_document_name;

    let all_db_variants_are_unit = info.check_all_db_variants_are_unit();

    // Evaluate fields.
    let to_api_field_list = fields_in_api.iter().map(|field| {
        let name = field.name();

        if field.inner_type.is_some() {
            quote! {
                #document_name::#name(v) => #api_document_name::#name(v.into()),
            }
        } else if all_db_variants_are_unit {
            quote! {
                #document_name::#name => #api_document_name::#name,
            }
        } else {
            quote! {
                #document_name::#name(_) => #api_document_name::#name,
            }
        }
    });

    let to_db_field_list = fields_in_api.iter().map(|field| {
        let name = field.name();

        if field.inner_type.is_some() {
            quote! {
                #api_document_name::#name(v) => #document_name::#name(v.into()),
            }
        } else if all_db_variants_are_unit {
            quote! {
                #api_document_name::#name => #document_name::#name,
            }
        } else {
            quote! {
                #api_document_name::#name => #document_name::#name(None),
            }
        }
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

    // Build result.
    Ok(quote! {
        impl#generics From<#document_name#generics> for #api_document_name#generics {
            fn from(value: #document_name#generics) -> Self {
                let mut result = match value {
                    #(#to_api_field_list)*
                };

                #replace_api_from_db_to_api_call

                result
            }
        }

        impl#generics From<#api_document_name#generics> for #document_name#generics {
            fn from(value: #api_document_name#generics) -> Self {
                let mut result = match value {
                    #(#to_db_field_list)*
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

fn build_field_list(
    _options: &ModelOptions,
    info: &ModelInfo,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let api_document_name = &info.api_document_name;
    let api_field_enum_name = &info.api_field_enum_name;
    let visibility = info.item.visibility();

    // Evaluate fields.
    let mut field_names = vec![];
    let mut field_paths = vec![];
    let mut get_variant_list = vec![];

    fields_in_api.iter().for_each(|field| {
        let name = field.name();
        let db_name = &field.db_name;

        match field.attributes.inner_model {
            InnerModelKind::Data => {
                field_names.push(quote! {
                    #name(Option<()>),
                });
                field_paths.push(quote! {
                    #api_field_enum_name::#name(_) => #db_name.into(),
                });

                if field.inner_type.is_some() {
                    get_variant_list.push(quote! {
                        #api_document_name::#name(_) => #api_field_enum_name::#name(None),
                    });
                } else {
                    get_variant_list.push(quote! {
                        #api_document_name::#name => #api_field_enum_name::#name(None),
                    });
                }
            }
            InnerModelKind::Struct => {
                let inner_api_type = field.attributes.api_inner_type.as_ref();
                let inner_api_type_name = inner_api_type
                    .map(|v| v.to_token_stream().to_string())
                    .unwrap_or_else(|| field.get_inner_db_type_name());
                let inner_api_type_enum = format_ident!("{}Field", inner_api_type_name);

                field_names.push(quote! {
                    #name(Option<#inner_api_type_enum>),
                });
                field_paths.push(quote! {
                    #api_field_enum_name::#name(v) => if let Some(v) = v {
                        format!("V.{}", v.path()).into()
                    } else {
                        #db_name.into()
                    }
                });
                get_variant_list.push(quote! {
                    #api_document_name::#name(_) => #api_field_enum_name::#name(None),
                });
            }
            InnerModelKind::Enum => {
                let inner_api_type = field.attributes.api_inner_type.as_ref();
                let inner_api_type_name = inner_api_type
                    .map(|v| v.to_token_stream().to_string())
                    .unwrap_or_else(|| field.get_inner_db_type_name());
                let inner_api_type_enum = format_ident!("{}Field", inner_api_type_name);

                field_names.push(quote! {
                    #name(Option<#inner_api_type_enum>),
                });
                field_paths.push(quote! {
                    #api_field_enum_name::#name(v) => if let Some(v) = v {
                        format!("V.{}", v.path()).into()
                    } else {
                        #db_name.into()
                    }
                });
                get_variant_list.push(quote! {
                    #api_document_name::#name(v) => #api_field_enum_name::#name(Some(v.variant())),
                });
            }
        }
    });

    // Build result.
    Ok(quote! {
        impl#generics #api_document_name#generics {
            pub fn variant(&self) -> #api_field_enum_name {
                match self {
                    #(#get_variant_list)*
                }
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(tag = "T", content = "V")]
        #visibility enum #api_field_enum_name {
            #(#field_names)*
        }

        impl #api_field_enum_name {
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

pub fn build_sensible_info_impl(
    _options: &ModelOptions,
    info: &ModelInfo,
    _is_sub_model: bool,
    fields_in_api: &[&FieldInfo],
) -> Result<TokenStream, syn::Error> {
    let generics = info.item.generics();
    let api_document_name = &info.api_document_name;

    // Evaluate fields list.
    let sensible_info_fields = fields_in_api.iter().map(|field| {
        let name = field.name();

        if field.inner_type.is_none() {
            quote! {
                #api_document_name::#name => {}
            }
        } else {
            match field.attributes.inner_model {
                InnerModelKind::Struct | InnerModelKind::Enum => quote! {
                    #api_document_name::#name(v) => v.remove_sensible_info(),
                },
                InnerModelKind::Data => quote! {
                    #api_document_name::#name(_) => {}
                },
            }
        }
    });

    // Build result.
    Ok(quote! {
        impl#generics #api_document_name#generics {
            pub fn remove_sensible_info(&mut self) {
                match self {
                    #(#sensible_info_fields)*
                }
            }
        }
    })
}
