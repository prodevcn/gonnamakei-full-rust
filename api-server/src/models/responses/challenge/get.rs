use serde::{Deserialize, Serialize};

use commons::clients::solana::models::SerializableChallenge;
use commons::database::documents::ChallengeAPIDocument;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeGetResponse {
    #[serde(default)]
    pub challenge: Option<ChallengeAPIDocument>,
    #[serde(default)]
    pub blockchain_info: Option<SerializableChallenge>,
}

impl ChallengeGetResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(
        challenge: Option<ChallengeAPIDocument>,
        blockchain_info: Option<SerializableChallenge>,
    ) -> Self {
        ChallengeGetResponse {
            challenge,
            blockchain_info,
        }
    }
}
