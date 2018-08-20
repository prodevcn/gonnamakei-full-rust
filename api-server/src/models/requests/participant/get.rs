use serde::{Deserialize, Serialize};

use commons::error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantGetRequestBody {
    #[serde(default)]
    pub return_fields: bool,
    #[serde(default)]
    pub return_active_bets: bool,
}

impl ParticipantGetRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        Ok(())
    }
}
