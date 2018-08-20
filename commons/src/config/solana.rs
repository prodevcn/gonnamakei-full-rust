use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaConfig {
    pub cluster_url: ArcStr,
}
