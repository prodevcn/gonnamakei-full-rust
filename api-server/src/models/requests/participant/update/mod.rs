use serde::{Deserialize, Serialize};

use commons::database::documents::APIParticipantGamesData;
use commons::error::AppResult;
pub use games::*;

mod games;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantUpdateRequestBody {
    pub games_data: APIParticipantGamesData,
}

impl ParticipantUpdateRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        validate_participant_games_data(&self.games_data)?;

        Ok(())
    }
}
