use serde::{Deserialize, Serialize};

use commons::database::types::DBUuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantLoginResponse {
    pub token: DBUuid,
}

impl ParticipantLoginResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(token: DBUuid) -> Self {
        ParticipantLoginResponse { token }
    }
}
