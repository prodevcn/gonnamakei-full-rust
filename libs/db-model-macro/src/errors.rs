use std::fmt;
use std::fmt::Display;

use quote::ToTokens;

#[derive(Debug)]
pub enum Error {
    // Keep this for debugging purpose.
    Message(String),
    UnexpectedItem,
    MissingStructItem,
    MissingStructOrEnumItem,
    UnexpectedMacroOption,
    CompulsoryAttributeArguments(String),
    IncorrectBoolAttributeValue,
    IncorrectStringAttributeValue,
    IncorrectEnumAttributeValue(&'static [&'static str]),
    DuplicatedStructName(String),
    UnsupportedNamedEnumVariant,
}

impl Error {
    pub fn with_tokens<T: ToTokens>(self, tokens: T) -> syn::Error {
        syn::Error::new_spanned(tokens, self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(v) => f.write_str(v),
            Error::UnexpectedItem => f.write_str("Unexpected element"),
            Error::MissingStructItem => f.write_str("A struct is required as the first element"),
            Error::MissingStructOrEnumItem => {
                f.write_str("A struct or enum is required as the first element")
            }
            Error::UnexpectedMacroOption => f.write_str("Unexpected macro option"),
            Error::CompulsoryAttributeArguments(msg) => {
                write!(f, "Compulsory attribute arguments. {}", msg)
            }
            Error::IncorrectBoolAttributeValue => f.write_str("The attribute value must be a bool"),
            Error::IncorrectStringAttributeValue => {
                f.write_str("The attribute value must be a string")
            }
            Error::IncorrectEnumAttributeValue(values) => write!(
                f,
                "The attribute value must be one of the following values: \"{}\"",
                values.join("\", \"")
            ),
            Error::DuplicatedStructName(name) => {
                write!(f, "The field name \"{}\" is duplicated", name)
            }
            Error::UnsupportedNamedEnumVariant => {
                f.write_str("Enum variants must always be anonymous")
            }
        }
    }
}
