use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use commons::database::types::DBUuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureRequestResponse {
    pub id: DBUuid,
    pub message: ArcStr,
}

impl SignatureRequestResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(id: DBUuid, message: ArcStr) -> Self {
        SignatureRequestResponse { id, message }
    }
}
