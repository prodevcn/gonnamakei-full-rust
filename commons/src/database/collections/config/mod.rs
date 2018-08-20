use std::ops::Deref;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::config::InitServiceConfig;
use crate::database::collection::DBCollection;
use crate::database::collections::cache::ConfigCollectionCache;
use crate::database::collections::CollectionKind;
use crate::database::documents::ConfigDBDocument;
use crate::database::types::DBUuid;
use crate::database::{DBDocument, DBInfo};
use crate::error::AppResult;

mod default_config;

lazy_static! {
    static ref COLLECTION: std::sync::Mutex<Option<Arc<ConfigCollection>>> =
        std::sync::Mutex::new(None);
}

#[derive(Debug)]
pub struct ConfigCollection {
    db_info: Arc<DBInfo>,
    cache: Arc<Mutex<ConfigCollectionCache>>,
}

impl ConfigCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<ConfigCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(ConfigCollection {
                    db_info: db_info.clone(),
                    cache: Arc::new(Mutex::new(ConfigCollectionCache::new(&db_info.config))),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<ConfigCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // GETTERS ----------------------------------------------------------------

    pub fn singleton_key(&self) -> &DBUuid {
        &self
            .db_info
            .config
            .database
            .as_ref()
            .unwrap()
            .configuration_id
    }

    /// Gets the singleton element in the collection.
    pub async fn singleton(&self) -> Arc<ConfigDBDocument> {
        let collection = ConfigCollection::instance();
        let mut cache = collection.cache.lock().await;
        cache.reload_if_necessary().await;

        cache.document.clone()
    }

    // METHODS ----------------------------------------------------------------

    /// Creates the singleton element.
    pub(in crate::database) async fn init_singleton(
        &self,
        config: &Arc<InitServiceConfig>,
    ) -> AppResult<()> {
        let default_configuration = Self::create_default_configuration(config);
        default_configuration.insert(false).await?;

        Ok(())
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;
        let config = &db_info.config;

        // Initialize collection.
        let collection = ConfigCollection::init(db_info);
        let collection_name = CollectionKind::Configuration.name();
        let _ = database.create_collection(collection_name).await; // Ignore error because it means already created.

        // Add initial config.
        if !collection
            .exists_by_key(collection.singleton_key())
            .await
            .unwrap()
        {
            collection.init_singleton(config).await?;

            info!("Configuration inserted");
        }

        Ok(())
    }

    pub(crate) async fn drop_analyzers(&self) -> AppResult<()> {
        Ok(())
    }

    pub(crate) fn create_default_configuration(
        config: &Arc<InitServiceConfig>,
    ) -> ConfigDBDocument {
        default_config::create_default_configuration(config)
    }
}

impl DBCollection for ConfigCollection {
    type Document = ConfigDBDocument;

    fn name() -> &'static str {
        CollectionKind::Configuration.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
