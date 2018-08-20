use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::database::collections::AuthenticationCollection;
use crate::database::collections::CollectionKind;
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::DBExpiration;
use crate::database::types::{Address, DBId, DBUuid};
use crate::database::{DBDocument, NullableOption};

model!(
    pub struct Authentication {
        /// The time this authentication expires.
        #[db_name = "X"]
        pub db_expires_at: NullableOption<DBExpiration>,

        /// The address we want to verify.
        #[db_name = "A"]
        pub address: NullableOption<Address>,
    }
);
