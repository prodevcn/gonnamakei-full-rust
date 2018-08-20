use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::data::SynchronizedDBDocument;
use crate::database::collections::CollectionKind;
use crate::database::collections::MutexCollection;
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::{DBId, DBMutex, DBUuid};
use crate::database::{DBDocument, NullableOption};

model!(
    #![sync_level = "document"]
    #![collection_kind = "Mutexes"]

    pub struct Mutex {
    }
);
