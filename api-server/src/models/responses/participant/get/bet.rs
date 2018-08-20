use serde::{Deserialize, Serialize};

use commons::clients::solana::models::{SerializableBet, SerializableChallenge};
use commons::database::types::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantActiveBetGetResponse {
    pub bet_key: Address,
    pub bet: SerializableBet,
    pub challenge: SerializableChallenge,
}

impl ParticipantActiveBetGetResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(bet_key: Address, bet: SerializableBet, challenge: SerializableChallenge) -> Self {
        ParticipantActiveBetGetResponse {
            bet_key,
            bet,
            challenge,
        }
    }
}
