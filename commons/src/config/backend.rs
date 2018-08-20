use serde::Deserialize;
use serde::Serialize;

use crate::database::types::DBUuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendConfig {
    pub handy_game_token: DBUuid,
}
