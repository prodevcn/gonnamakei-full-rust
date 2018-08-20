use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIConfig {
    pub port: u16,
    pub filtering: APIFilteringConfig,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIFilteringConfig {
    #[serde(default)]
    pub general: APIFilteringStatsConfig,
    #[serde(default)]
    pub admin: APIFilteringStatsConfig,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIFilteringStatsConfig {
    #[serde(default)]
    pub field_count: usize,
    #[serde(default)]
    pub const_count: usize,
    #[serde(default)]
    pub expression_count: usize,
    #[serde(default)]
    pub function_count: usize,
}
