//! Procedural macro to create a Database model for types.

#![recursion_limit = "128"]

extern crate proc_macro;

use syn::{parse_macro_input, File};

use model_builders::*;
use sub_model_builders::*;

mod constants;
mod data;
mod errors;
mod model_builders;
mod sub_model_builders;
mod utils;

/// Creates a database model for a type.
#[proc_macro]
pub fn model(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file = parse_macro_input!(item as File);

    process_model(file)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Creates a subtype of a database model for a type.
#[proc_macro]
pub fn sub_model(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file = parse_macro_input!(item as File);

    process_sub_model(file)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
