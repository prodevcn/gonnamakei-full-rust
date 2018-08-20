use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct PaginatedResponse<T: Serialize> {
    pub count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    pub page: u64,
    pub rows_per_page: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    pub results: Vec<T>,
}
