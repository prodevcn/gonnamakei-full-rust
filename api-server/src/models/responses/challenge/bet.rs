use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use commons::database::types::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeBetResponse {
    pub bet: Address,
    pub message: ArcStr,
    pub fee: u64,
}

impl ChallengeBetResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(bet: Address, message: ArcStr, fee: u64) -> Self {
        ChallengeBetResponse { bet, message, fee }
    }
}
