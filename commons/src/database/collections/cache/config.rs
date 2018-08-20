use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::config::InitServiceConfig;
use crate::database::collections::ConfigCollection;
use crate::database::documents::ConfigDBDocument;
use crate::database::DBCollection;

#[derive(Debug)]
pub struct ConfigCollectionCache {
    pub document: Arc<ConfigDBDocument>,
    pub expiration: Instant,
}

impl ConfigCollectionCache {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(config: &Arc<InitServiceConfig>) -> ConfigCollectionCache {
        ConfigCollectionCache {
            document: Arc::new(ConfigCollection::create_default_configuration(config)),
            expiration: Instant::now() - Duration::from_millis(1),
        }
    }

    // METHODS ----------------------------------------------------------------

    pub async fn reload_if_necessary(&mut self) {
        if Instant::now() <= self.expiration {
            return;
        }

        let collection = ConfigCollection::instance();
        let document = match collection
            .get_one_by_key(collection.singleton_key(), None)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                remote_error!(
                    "Cannot access to DB while trying to cache the configuration. Error: {}",
                    e
                );
                return;
            }
        };

        if let Some(document) = document {
            self.expiration = Instant::now()
                + Duration::from_secs(document.database.collections.configuration.cache_expiration);
            self.document = Arc::new(document);
        }
    }
}
