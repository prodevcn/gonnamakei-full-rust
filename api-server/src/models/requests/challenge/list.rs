use serde::{Deserialize, Serialize};

use commons::database::documents::ChallengeAPIDocumentField;
use commons::error::AppResult;
use commons::server::requests::PaginatedRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeListRequestBody {
    pub responses: PaginatedRequest<ChallengeAPIDocumentField>,
}

impl ChallengeListRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        Ok(())
    }
}
