use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::database::types::DBUuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfig {
    pub url: ArcStr,
    pub username: ArcStr,
    pub password: ArcStr,
    pub secret: Vec<u8>,
    pub salt: Vec<u8>,
    pub configuration_id: DBUuid,
}
