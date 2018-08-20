use proc_macro2::TokenStream;
use quote::quote;
use syn::File;

pub use build_api::*;
pub use build_db::*;

use crate::data::{ModelInfo, ModelOptions};

mod build_api;
mod build_db;

pub fn process_model(file: File) -> Result<TokenStream, syn::Error> {
    let options = ModelOptions::from_attributes(&file.attrs)?;
    let info = ModelInfo::from_file_for_model(&options, &file)?;

    let tokens = if options.build_api {
        let db = build_db_model(&options, &info)?;
        let api = build_api_model(&options, &info)?;

        // Keep this for debugging purpose.
        // return Err(crate::errors::Error::Message(api.to_string()).with_tokens(file));

        let tokens = quote! {
            #db
            #api
        };

        tokens
    } else {
        build_db_model(&options, &info)?
    };

    // Keep this for debugging purpose.
    // return Err(crate::errors::Error::Message(tokens.to_string()).with_tokens(file));

    Ok(tokens)
}
