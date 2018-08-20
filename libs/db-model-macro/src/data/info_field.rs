use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Field, Fields, Path, Type, Variant};

use crate::data::{FieldAttributes, InnerModelKind};
use crate::errors::Error;
use crate::utils::{get_inner_map_types_from_path, get_inner_type_from_path, get_name_from_path};

pub struct FieldInfo<'a> {
    pub node: FieldNode<'a>,
    pub attributes: FieldAttributes,
    pub db_name: String,
    // Always set for fields. Optional for variants.
    pub inner_type: Option<TokenStream>,
    pub key_type: Option<TokenStream>,
    pub base_type_kind: BaseTypeKind,
    pub field_type_kind: Option<FieldTypeKind>,
}

impl<'a> FieldInfo<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn from_field(field: &'a Field) -> Result<FieldInfo<'a>, syn::Error> {
        let attributes = FieldAttributes::from_attributes(&field.attrs, false)?;

        // Get db name.
        let db_name = if let Some(db_name) = &attributes.db_name {
            db_name.clone()
        } else {
            field.ident.as_ref().unwrap().to_string()
        };

        // Create result.
        let mut result = FieldInfo {
            node: FieldNode::Field(field),
            attributes,
            db_name,
            inner_type: Some(field.ty.to_token_stream()),
            key_type: None,
            base_type_kind: BaseTypeKind::Other,
            field_type_kind: None,
        };

        if let Type::Path(type_path) = &field.ty {
            result.read_field_type(&field.ident.as_ref().unwrap().to_string(), &type_path.path)?;
        } else {
            return Err(crate::errors::Error::Message(
                "The type of the field must be a path".to_string(),
            )
            .with_tokens(field));
        }

        let attributes = &mut result.attributes;
        match result.field_type_kind {
            Some(FieldTypeKind::NullableOption) => {
                attributes.db.push(quote! {
                    #[serde(skip_serializing_if = "NullableOption::is_missing")]
                });
                attributes.api.push(quote! {
                    #[serde(skip_serializing_if = "NullableOption::is_missing")]
                });
            }
            Some(FieldTypeKind::Option) => {
                attributes.db.push(quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                });
                attributes.api.push(quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                });
            }
            None => match result.base_type_kind {
                BaseTypeKind::Other | BaseTypeKind::Box => {}
                BaseTypeKind::Vec | BaseTypeKind::VecDBReference => {
                    attributes.db.push(quote! {
                        #[serde(skip_serializing_if = "Vec::is_empty")]
                    });
                    attributes.api.push(quote! {
                        #[serde(skip_serializing_if = "Vec::is_empty")]
                    });
                }
                BaseTypeKind::HashMap => {
                    attributes.db.push(quote! {
                        #[serde(skip_serializing_if = "HashMap::is_empty")]
                    });
                    attributes.api.push(quote! {
                        #[serde(skip_serializing_if = "HashMap::is_empty")]
                    });
                }
                BaseTypeKind::DBReference => {}
            },
        }

        Ok(result)
    }

    pub fn from_variant(variant: &'a Variant) -> Result<FieldInfo<'a>, syn::Error> {
        let attributes = FieldAttributes::from_attributes(&variant.attrs, true)?;

        // Get db name.
        let db_name = if let Some(db_name) = &attributes.db_name {
            db_name.clone()
        } else {
            variant.ident.to_string()
        };

        // Get type.
        let mut field_name: Option<String> = None;
        let inner_type = match &variant.fields {
            Fields::Unnamed(v) => match v.unnamed.iter().next() {
                Some(field) => {
                    field_name = field.ident.as_ref().map(|v| v.to_string());
                    Some(&field.ty)
                }
                None => None,
            },
            Fields::Unit => None,
            Fields::Named(v) => {
                return Err(Error::UnsupportedNamedEnumVariant.with_tokens(v));
            }
        };

        // Create result.
        let mut result = FieldInfo {
            node: FieldNode::Variant(variant),
            attributes,
            db_name,
            inner_type: inner_type.map(|v| v.to_token_stream()),
            key_type: None,
            base_type_kind: BaseTypeKind::Other,
            field_type_kind: None,
        };

        if let (Some(field_name), Some(Type::Path(type_path))) = (field_name, &inner_type) {
            result.read_field_type(&field_name, &type_path.path)?;
        }

        if inner_type.is_none() {
            result.attributes.inner_model = InnerModelKind::Data;
        }

        // Note: in enums do not include the skip serialization field because the value is important
        // no matter if it is null or not.

        Ok(result)
    }

    // GETTERS ----------------------------------------------------------------

    pub fn name(&self) -> &Ident {
        self.node.ident()
    }

    // METHODS ----------------------------------------------------------------

    fn read_base_type(&mut self, field_name: &str, path: &Path) -> Result<(), syn::Error> {
        if let Some(name) = get_name_from_path(path) {
            if name == "Vec" {
                let type_path = match get_inner_type_from_path(path) {
                    Some(v) => match v {
                        Type::Path(v) => v,
                        Type::Tuple(v) => {
                            self.inner_type = Some(v.to_token_stream());
                            self.base_type_kind = BaseTypeKind::Vec;
                            return Ok(());
                        }
                        _ => {
                            return Err(crate::errors::Error::Message(format!(
                                "Incorrect type for Vec<T> in field '{}'",
                                field_name
                            ))
                            .with_tokens(path));
                        }
                    },
                    None => {
                        return Err(crate::errors::Error::Message(format!(
                            "Incorrect type for Vec<T> in field '{}'",
                            field_name
                        ))
                        .with_tokens(path));
                    }
                };

                let name = get_name_from_path(&type_path.path);
                if let Some("DBReference") = name.as_deref() {
                    self.inner_type = match get_inner_type_from_path(&type_path.path) {
                        Some(v) => Some(v.to_token_stream()),
                        None => {
                            return Err(crate::errors::Error::Message(format!(
                                "Missing T in Vec<DBReference<T>> in field '{}'",
                                field_name
                            ))
                            .with_tokens(path));
                        }
                    };
                    self.base_type_kind = BaseTypeKind::VecDBReference;
                } else {
                    self.inner_type = Some(type_path.to_token_stream());
                    self.base_type_kind = BaseTypeKind::Vec;
                }
            } else if name == "Box" {
                self.inner_type = match get_inner_type_from_path(path) {
                    Some(v) => Some(v.to_token_stream()),
                    None => {
                        return Err(crate::errors::Error::Message(format!(
                            "Missing T in Box<T> in field '{}'",
                            field_name
                        ))
                        .with_tokens(path));
                    }
                };
                self.base_type_kind = BaseTypeKind::Box;
            } else if name == "DBReference" {
                self.inner_type = match get_inner_type_from_path(path) {
                    Some(v) => Some(v.to_token_stream()),
                    None => {
                        return Err(crate::errors::Error::Message(format!(
                            "Missing T in DBReference<T> in field '{}'",
                            field_name
                        ))
                        .with_tokens(path));
                    }
                };
                self.base_type_kind = BaseTypeKind::DBReference;
            } else if name == "HashMap" {
                let (key_type, value_type) = match get_inner_map_types_from_path(path) {
                    Some(v) => v,
                    None => {
                        return Err(crate::errors::Error::Message(format!(
                            "Incorrect type for map in field '{}'",
                            field_name
                        ))
                        .with_tokens(path));
                    }
                };

                self.inner_type = Some(value_type.to_token_stream());
                self.key_type = Some(key_type.to_token_stream());
                self.base_type_kind = BaseTypeKind::HashMap;
            } else {
                self.inner_type = Some(path.to_token_stream());
                self.base_type_kind = BaseTypeKind::Other;
            }
        } else {
            return Err(crate::errors::Error::Message(
                format!("Cannot get a base type from the tokens. They must be a simple type (T), a Vec<T> or a HashMap<K, T> in field '{}'", field_name),
            )
                .with_tokens(path));
        }

        Ok(())
    }

    fn read_field_type(&mut self, field_name: &str, path: &Path) -> Result<(), syn::Error> {
        if let Some(name) = get_name_from_path(path) {
            if name == "NullableOption" {
                let type_path = match get_inner_type_from_path(path) {
                    Some(v) => match v {
                        Type::Path(v) => v,
                        Type::Tuple(v) => {
                            self.field_type_kind = Some(FieldTypeKind::NullableOption);
                            self.base_type_kind = BaseTypeKind::Other;
                            self.inner_type = Some(v.to_token_stream());
                            return Ok(());
                        }
                        _ => {
                            return Err(crate::errors::Error::Message(format!(
                                "Incorrect type for NullableOption<T> in field '{}'",
                                field_name
                            ))
                            .with_tokens(path));
                        }
                    },
                    None => {
                        return Err(crate::errors::Error::Message(format!(
                            "Incorrect type for NullableOption<T> in field '{}'",
                            field_name
                        ))
                        .with_tokens(path));
                    }
                };

                self.field_type_kind = Some(FieldTypeKind::NullableOption);
                self.read_base_type(field_name, &type_path.path)?;
            } else if name == "Option" {
                let type_path = match get_inner_type_from_path(path) {
                    Some(v) => match v {
                        Type::Path(v) => v,
                        Type::Tuple(v) => {
                            self.field_type_kind = Some(FieldTypeKind::NullableOption);
                            self.base_type_kind = BaseTypeKind::Other;
                            self.inner_type = Some(v.to_token_stream());
                            return Ok(());
                        }
                        _ => {
                            return Err(crate::errors::Error::Message(format!(
                                "Incorrect type for Option<T> in field '{}'",
                                field_name
                            ))
                            .with_tokens(path));
                        }
                    },
                    None => {
                        return Err(crate::errors::Error::Message(format!(
                            "Incorrect type for Option<T> in field '{}'",
                            field_name
                        ))
                        .with_tokens(path));
                    }
                };

                self.field_type_kind = Some(FieldTypeKind::Option);
                self.read_base_type(field_name, &type_path.path)?;
            } else {
                self.field_type_kind = None;
                self.read_base_type(field_name, path)?;
            }
        } else {
            return Err(crate::errors::Error::Message(
                format!("Cannot get a field type from the tokens. They must be an upper type (a) or NullableOption<a> in field '{}'", field_name),
            )
                .with_tokens(path));
        }

        Ok(())
    }

    pub fn build_db_field_type(&self) -> TokenStream {
        let inner_type = if let FieldNode::Field(_) = &self.node {
            self.inner_type.clone().unwrap()
        } else {
            match &self.inner_type {
                Some(v) => v.clone(),
                None => quote! { Option<()> },
            }
        };

        let base = match self.base_type_kind {
            BaseTypeKind::Other => quote! { #inner_type },
            BaseTypeKind::Box => quote! { Box<#inner_type> },
            BaseTypeKind::Vec => quote! { Vec<#inner_type> },
            BaseTypeKind::VecDBReference => quote! { Vec<DBReference<#inner_type>> },
            BaseTypeKind::HashMap => {
                let key_type = self.key_type.as_ref().unwrap();
                quote! { HashMap<#key_type, #inner_type> }
            }
            BaseTypeKind::DBReference => quote! { DBReference<#inner_type> },
        };

        match self.field_type_kind {
            Some(FieldTypeKind::NullableOption) => quote! { NullableOption<#base> },
            Some(FieldTypeKind::Option) => quote! { Option<#base> },
            None => base,
        }
    }

    pub fn build_api_field_type(&self) -> TokenStream {
        let inner_type = if let Some(api_inner_type) = &self.attributes.api_inner_type {
            quote! { #api_inner_type }
        } else if let FieldNode::Field(_) = &self.node {
            self.inner_type.clone().unwrap()
        } else {
            match &self.inner_type {
                Some(v) => v.clone(),
                None => quote! { Option<()> },
            }
        };

        let base = match self.base_type_kind {
            BaseTypeKind::Other => quote! { #inner_type },
            BaseTypeKind::Box => quote! { Box<#inner_type> },
            BaseTypeKind::Vec => quote! { Vec<#inner_type> },
            BaseTypeKind::VecDBReference => quote! { Vec<APIReference<#inner_type>> },
            BaseTypeKind::HashMap => {
                let key_type = self.key_type.as_ref().unwrap();
                quote! { HashMap<#key_type, #inner_type> }
            }
            BaseTypeKind::DBReference => quote! { APIReference<#inner_type> },
        };

        match self.field_type_kind {
            Some(FieldTypeKind::NullableOption) => quote! { NullableOption<#base> },
            Some(FieldTypeKind::Option) => quote! { Option<#base> },
            None => base,
        }
    }

    pub fn get_inner_db_type_name(&self) -> String {
        let inner_type = self.inner_type.clone().unwrap().to_token_stream();
        let inner_type: Type = syn::parse2(inner_type).expect("The inner type must be a Type");

        match inner_type {
            Type::Path(v) => match get_name_from_path(&v.path) {
                Some(v) => v,
                None => unreachable!("Incorrect path type in get_inner_db_type_name"),
            },
            Type::Tuple(_) => {
                unreachable!("Cannot call get_inner_db_type_name with a tuple inner type")
            }
            _ => unreachable!("Incorrect type in get_inner_db_type_name"),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BaseTypeKind {
    Other,
    Box,
    Vec,
    VecDBReference,
    HashMap,
    DBReference,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FieldTypeKind {
    NullableOption,
    Option,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub enum FieldNode<'a> {
    Field(&'a Field),
    Variant(&'a Variant),
}

impl<'a> FieldNode<'a> {
    // GETTERS ----------------------------------------------------------------

    pub fn ident(&self) -> &Ident {
        match self {
            FieldNode::Field(v) => v.ident.as_ref().unwrap(),
            FieldNode::Variant(v) => &v.ident,
        }
    }

    pub fn as_field(&self) -> Option<&'a Field> {
        match self {
            FieldNode::Field(v) => Some(v),
            FieldNode::Variant(_) => None,
        }
    }
}

impl<'a> ToTokens for FieldNode<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            FieldNode::Field(v) => v.to_token_stream(),
            FieldNode::Variant(v) => v.to_token_stream(),
        });
    }
}
