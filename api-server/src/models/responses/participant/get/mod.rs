use serde::{Deserialize, Serialize};

pub use bet::*;
use commons::database::documents::ParticipantAPIDocument;

mod bet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<ParticipantAPIDocument>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bets: Option<Vec<ParticipantActiveBetGetResponse>>,
}

impl ParticipantGetResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(
        participant: Option<ParticipantAPIDocument>,
        active_bets: Option<Vec<ParticipantActiveBetGetResponse>>,
    ) -> Self {
        ParticipantGetResponse {
            participant,
            active_bets,
        }
    }
}
