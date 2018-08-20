use serde::{Deserialize, Serialize};

use commons::database::documents::APISignatureAction;
use commons::database::types::Address;
use commons::error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureRequestBody {
    pub action: APISignatureAction,
    pub address: Address,
}

impl SignatureRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<()> {
        Ok(())
    }
}
