use serde::{Deserialize, Serialize};

use commons::database::types::DateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BetSendResponse {
    pub start_time: DateTime,
    pub timeout: DateTime,
}

impl BetSendResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(start_time: DateTime, timeout: DateTime) -> Self {
        BetSendResponse {
            start_time,
            timeout,
        }
    }
}
