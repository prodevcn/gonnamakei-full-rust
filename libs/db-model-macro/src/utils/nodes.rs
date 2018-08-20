use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Lit, Meta, Path, PathArguments, Type};

use crate::errors::Error;

pub fn get_simple_name_from_meta(meta: &Meta) -> Option<String> {
    match meta {
        Meta::Path(path) => get_simple_name_from_path(path),
        Meta::List(meta_list) => get_simple_name_from_path(&meta_list.path),
        Meta::NameValue(named_value) => get_simple_name_from_path(&named_value.path),
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn get_simple_name_from_path(path: &Path) -> Option<String> {
    let segments = &path.segments;
    if segments.len() != 1 {
        return None;
    }

    let segment = segments.first().unwrap();
    if !segment.arguments.is_empty() {
        return None;
    }

    Some(segment.ident.to_string())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn get_name_from_path(path: &Path) -> Option<String> {
    let segments = &path.segments;
    if segments.len() != 1 {
        return None;
    }

    let segment = segments.first().unwrap();

    Some(segment.ident.to_string())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn get_bool_literal_from_meta(meta: &Meta) -> Result<Option<bool>, syn::Error> {
    match meta {
        Meta::Path(_) => Ok(None),
        Meta::NameValue(named_value) => match &named_value.lit {
            Lit::Bool(lit) => Ok(Some(lit.value)),
            _ => Err(Error::IncorrectBoolAttributeValue.with_tokens(meta)),
        },
        _ => Err(Error::IncorrectBoolAttributeValue.with_tokens(meta)),
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn process_bool_literal(
    meta: &Meta,
    attribute_name: &str,
    default: Option<bool>,
) -> Result<bool, syn::Error> {
    match get_bool_literal_from_meta(meta)? {
        Some(v) => Ok(v),
        None => {
            if let Some(default) = default {
                Ok(default)
            } else {
                Err(Error::CompulsoryAttributeArguments(format!(
                    "The \"{}\" attribute require a bool argument",
                    attribute_name
                ))
                .with_tokens(&meta))
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn get_enum_literal_from_meta<T: Copy>(
    meta: &Meta,
    string_values: &'static [&'static str],
    values: &[T],
) -> Result<Option<T>, syn::Error> {
    match meta {
        Meta::Path(_) => Ok(None),
        Meta::NameValue(named_value) => match &named_value.lit {
            Lit::Str(lit) => {
                let argument = lit.value();
                for (string_value, value) in string_values.iter().zip(values) {
                    if &argument == string_value {
                        return Ok(Some(*value));
                    }
                }

                Err(Error::IncorrectEnumAttributeValue(string_values).with_tokens(meta))
            }
            _ => Err(Error::IncorrectEnumAttributeValue(string_values).with_tokens(meta)),
        },
        _ => Err(Error::IncorrectEnumAttributeValue(string_values).with_tokens(meta)),
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn get_string_literal_from_meta(meta: &Meta) -> Result<Option<String>, syn::Error> {
    match meta {
        Meta::Path(_) => Ok(None),
        Meta::NameValue(named_value) => match &named_value.lit {
            Lit::Str(lit) => Ok(Some(lit.value())),
            _ => Err(Error::IncorrectStringAttributeValue.with_tokens(meta)),
        },
        _ => Err(Error::IncorrectStringAttributeValue.with_tokens(meta)),
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn process_string_literal(
    meta: &Meta,
    attribute_name: &str,
    default: Option<String>,
) -> Result<String, syn::Error> {
    match get_string_literal_from_meta(meta)? {
        Some(v) => Ok(v),
        None => {
            if let Some(default) = default {
                Ok(default)
            } else {
                Err(Error::CompulsoryAttributeArguments(format!(
                    "The \"{}\" attribute require a string argument",
                    attribute_name
                ))
                .with_tokens(&meta))
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn process_enum_literal<T: Copy>(
    meta: &Meta,
    string_values: &'static [&'static str],
    values: &[T],
    attribute_name: &str,
    default: Option<T>,
) -> Result<T, syn::Error> {
    match get_enum_literal_from_meta(meta, string_values, values)? {
        Some(v) => Ok(v),
        None => {
            if let Some(default) = default {
                Ok(default)
            } else {
                return Err(Error::CompulsoryAttributeArguments(format!(
                    "The \"{}\" attribute require one of the following arguments: \"{}\"",
                    attribute_name,
                    string_values.join("\", \"")
                ))
                .with_tokens(&meta));
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn process_only_attribute(
    meta: &Meta,
    attribute_name: &str,
) -> Result<TokenStream, syn::Error> {
    match meta {
        Meta::List(meta_list) => {
            let attribute = &meta_list.nested;
            Ok(quote! {
                #[#attribute]
            })
        }
        _ => {
            return Err(Error::CompulsoryAttributeArguments(format!(
                "The \"{}\" attribute require a list of arguments, e.g: {}(...)",
                attribute_name, attribute_name,
            ))
            .with_tokens(&meta));
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Gets the inner type:
/// - A<B> -> B
/// - A<Box<B>> -> B
pub fn get_inner_type_from_path(path: &Path) -> Option<&Type> {
    if path.segments.len() != 1 {
        return None;
    }

    let path_segment = path.segments.first().unwrap();
    if let PathArguments::AngleBracketed(inner_type) = &path_segment.arguments {
        match inner_type.args.first() {
            Some(GenericArgument::Type(ty)) => Some(ty),
            _ => None,
        }
    } else {
        None
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Gets the inner type of a map:
/// - A<B, C> -> (B, C)
/// - A<Box<B, C>> -> (B, C)
pub fn get_inner_map_types_from_path(path: &Path) -> Option<(&Type, &Type)> {
    if path.segments.len() != 1 {
        return None;
    }

    let path_segment = path.segments.first().unwrap();
    if let PathArguments::AngleBracketed(inner_type) = &path_segment.arguments {
        let mut iter = inner_type.args.iter();
        let first = match iter.next() {
            Some(GenericArgument::Type(ty)) => ty,
            _ => return None,
        };

        let second = match iter.next() {
            Some(GenericArgument::Type(ty)) => ty,
            _ => return None,
        };

        Some((first, second))
    } else {
        None
    }
}
