use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::database::collections::CollectionKind;
use crate::database::collections::SignatureCollection;
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::DBExpiration;
use crate::database::types::{Address, DBId, DBUuid};
use crate::database::AqlBuilder;
use crate::database::{DBDocument, NullableOption};

model!(
    pub struct Signature {
        /// The action the signature is intended for.
        #[db_name = "T"]
        #[inner_model = "enum"]
        pub action: NullableOption<SignatureAction>,

        /// The address we want to verify.
        #[db_name = "A"]
        pub address: NullableOption<Address>,

        /// The time this signature expires.
        #[db_name = "X"]
        pub db_expires_at: NullableOption<DBExpiration>,
    }
);

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

sub_model!(
    #![build_api]

    pub enum SignatureAction {
        /// Action to login a user.
        #[db_name = "L"]
        Login,
    }
);

impl SignatureAction {
    // METHODS ----------------------------------------------------------------

    /// Builds the message that will be signed by the user.
    pub fn build_message(&self, address: &str, nonce: &str) -> String {
        match self {
            SignatureAction::Login => format!(
                "Approve this message to login into the app. \nUser={} \nNonce={}",
                address, nonce
            ),
        }
    }
}
