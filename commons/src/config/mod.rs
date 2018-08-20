use std::sync::Arc;

use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

pub use backend::*;
pub use database::*;
pub use game::*;
pub use solana::*;
pub use wallet::*;

use crate::constants::CONFIG_FILE;
use crate::database::collections::ConfigCollection;
use crate::database::documents::ConfigDBDocument;
use crate::error::AppResult;

mod backend;
mod database;
mod game;
mod solana;
mod wallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitServiceConfig {
    pub app_name: ArcStr,
    pub public_url: ArcStr,
    pub public_short_url: ArcStr,
    pub version: ArcStr,

    // Module.
    pub database: Option<Arc<DatabaseConfig>>,
    pub game_apis: Option<Arc<GamesConfig>>,
    pub solana: Option<Arc<SolanaConfig>>,
    pub wallet: Option<Arc<WalletConfig>>,
    pub backend: Option<Arc<BackendConfig>>,
}

impl InitServiceConfig {
    // METHODS ----------------------------------------------------------------

    pub async fn db_config(&self) -> Arc<ConfigDBDocument> {
        ConfigCollection::instance().singleton().await
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn read_app_config() -> AppResult<InitServiceConfig> {
    let path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| CONFIG_FILE.to_string());
    println!("Reading config file at [CONFIG_PATH={}]", path);

    let content = std::fs::read_to_string(path)?;

    let result: InitServiceConfig = toml::from_str(content.as_str())?;

    Ok(result)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn write_app_config(config: InitServiceConfig) -> AppResult<()> {
    let path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| CONFIG_FILE.to_string());
    info!("Writing config file at: {}", path);

    let value = toml::to_string_pretty(&config)?;

    std::fs::write(path, value)?;

    Ok(())
}
