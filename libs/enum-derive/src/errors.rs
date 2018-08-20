use std::fmt;
use std::fmt::Display;

use quote::ToTokens;

#[derive(Debug)]
pub enum Error {
    ExpectedEnum,
    ExpectedUnitVariant,
    GenericsUnsupported,
}

impl Error {
    pub fn with_tokens<T: ToTokens>(self, tokens: T) -> syn::Error {
        syn::Error::new_spanned(tokens, self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ExpectedEnum => f.write_str("EnumList can only be derived for enum types"),
            Error::ExpectedUnitVariant => {
                f.write_str("EnumList can only be derived for enum types with unit variants only")
            }
            Error::GenericsUnsupported => {
                f.write_str("EnumList cannot be derived for generic types")
            }
        }
    }
}
