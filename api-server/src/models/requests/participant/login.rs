use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use commons::database::types::DBUuid;
use commons::error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantLoginRequestBody {
    pub id: DBUuid,
    pub signature: ArcStr,
}

impl ParticipantLoginRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        Ok(())
    }
}
