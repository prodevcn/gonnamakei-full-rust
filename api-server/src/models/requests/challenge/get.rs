use serde::{Deserialize, Serialize};

use commons::error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeGetRequestBody {
    #[serde(default)]
    pub return_fields: bool,
    #[serde(default)]
    pub return_blockchain_data: bool,
}

impl ChallengeGetRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        Ok(())
    }
}
