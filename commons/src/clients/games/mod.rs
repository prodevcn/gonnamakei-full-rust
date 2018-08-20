use std::sync::Arc;

pub use clash_royale::*;

use crate::config::InitServiceConfig;

mod clash_royale;

pub struct GameClients {
    pub clash_royale: Arc<ClashRoyaleClient>,
}

impl GameClients {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(config: &Arc<InitServiceConfig>, http_client: &Arc<reqwest::Client>) -> Self {
        let games_config = config.game_apis.clone().unwrap();
        GameClients {
            clash_royale: Arc::new(ClashRoyaleClient::new(games_config, http_client.clone())),
        }
    }
}
