use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub use mutex::*;

use crate::database::collections::{CollectionKind, MutexCollection};
use crate::database::types::DBUuid;
use crate::database::DBDocument;

mod mutex;
mod tests;

pub trait SynchronizedDBDocument: DBDocument + Serialize + for<'de> Deserialize<'de> {
    /// The key of the document that represents the collection in the config collection.
    fn collection_key(config_collection: &Arc<MutexCollection>) -> &DBUuid;
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Releases all mutex associated to this microservice.
pub async fn release_all_mutex_of_current_microservice() {
    let jobs: Vec<_> = CollectionKind::enum_list()
        .iter()
        .filter_map(|kind| kind.release_all_document_mutex())
        .collect();

    for job in jobs {
        let _ = job.await;
    }
}
