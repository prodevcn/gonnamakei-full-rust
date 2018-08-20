use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

pub use api::*;
pub use database::*;

use crate::database::collections::{CollectionKind, ConfigCollection};
use crate::database::document::DBDocument;
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::{DBId, DBUuid};

mod api;
mod database;

model!(
    #![collection_kind = "Configuration"]

    pub struct Config {
        #[skip_normalize]
        pub database: Arc<DatabaseConfig>,

        #[skip_normalize]
        pub api: Arc<APIConfig>,
    }
);
