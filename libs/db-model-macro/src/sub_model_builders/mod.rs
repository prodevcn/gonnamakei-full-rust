use proc_macro2::TokenStream;
use quote::quote;
use syn::File;

pub use build_api_enum::*;
pub use build_api_struct::*;
use build_db_enum::*;
use build_db_struct::*;

use crate::data::{ModelInfo, ModelNode, ModelOptions};

mod build_api_enum;
mod build_api_struct;
mod build_db_enum;
mod build_db_struct;

pub fn process_sub_model(file: File) -> Result<TokenStream, syn::Error> {
    let options = ModelOptions::from_attributes(&file.attrs)?;
    let info = ModelInfo::from_file_for_sub_model(&options, &file)?;

    let tokens = match &info.item {
        ModelNode::Struct(_) => {
            if options.build_api {
                let db = build_sub_db_struct_model(&options, &info)?;
                let api = build_sub_api_struct_model(&options, &info)?;

                // Keep this for debugging purpose.
                // return Err(crate::errors::Error::Message(api.to_string()).with_tokens(file));

                quote! {
                    #db
                    #api
                }
            } else {
                build_sub_db_struct_model(&options, &info)?
            }
        }
        ModelNode::Enum(_) => {
            if options.build_api {
                let db = build_sub_db_enum_model(&options, &info)?;
                let api = build_sub_api_enum_model(&options, &info)?;

                // Keep this for debugging purpose.
                // return Err(crate::errors::Error::Message(api.to_string()).with_tokens(file));

                quote! {
                    #db
                    #api
                }
            } else {
                build_sub_db_enum_model(&options, &info)?
            }
        }
    };

    // Keep this for debugging purpose.
    // return Err(crate::errors::Error::Message(tokens.to_string()).with_tokens(file));

    Ok(tokens)
}
