use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeCheckResponse {
    pub status: ChallengeCheckResponseStatus,
}

impl ChallengeCheckResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(status: ChallengeCheckResponseStatus) -> Self {
        ChallengeCheckResponse { status }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChallengeCheckResponseStatus {
    Won,
    Lost,
    NotInitiated,
    Initiated,
    Expired,
}
