//! This file contains the default configuration of the system.

use std::path::Path;
use std::sync::Arc;

use crate::config::InitServiceConfig;
use crate::database::documents::APIConfig;
use crate::database::documents::APIFilteringConfig;
use crate::database::documents::APIFilteringStatsConfig;
use crate::database::documents::CollectionsConfig;
use crate::database::documents::ConfigurationCollectionConfig;
use crate::database::documents::{ConfigDBDocument, DatabaseConfig};

pub fn create_default_configuration(config: &Arc<InitServiceConfig>) -> ConfigDBDocument {
    // Do not call singleton_id because it makes a loop during initialization.
    let db_key = config.database.as_ref().unwrap().configuration_id.clone();
    let config_path = std::env::var("CONFIG_PATH").unwrap();
    let mut local_folder_path = if cfg!(debug_assertions) {
        Path::new(&config_path)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    } else {
        Path::new(&config_path).parent().unwrap().to_path_buf()
    };
    local_folder_path.push(".local");

    ConfigDBDocument {
        db_key: Some(db_key),
        db_rev: None,
        database: Arc::new(DatabaseConfig {
            field_expiration: 2592000, // 30 days (in seconds)
            long_field_expiration: 12, // 1 year (in months)
            collections: CollectionsConfig {
                configuration: ConfigurationCollectionConfig {
                    cache_expiration: 900, // 15 min (in seconds)
                },
            },
        }),
        api: Arc::new(APIConfig {
            port: 3003,
            filtering: APIFilteringConfig {
                general: APIFilteringStatsConfig {
                    field_count: 3,
                    const_count: 10,
                    expression_count: 3,
                    function_count: 6,
                },
                admin: APIFilteringStatsConfig {
                    field_count: 20,
                    const_count: 30,
                    expression_count: 10,
                    function_count: 20,
                },
            },
        }),
    }
}
