use std::str::FromStr;

use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use commons::error::{AppError, AppResult, INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE};
use commons::solana_sdk::hash::Hash;
use commons::solana_sdk::signature::Signature;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BetSendRequestBody {
    pub signature: ArcStr,
    pub recent_block_hash: ArcStr,
}

impl BetSendRequestBody {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> AppResult<(Signature, Hash)> {
        let signature = match Signature::from_str(self.signature.as_str()) {
            Ok(v) => v,
            Err(_) => {
                return Err(AppError::new_with_status(
                    warp::http::StatusCode::BAD_REQUEST,
                    INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
                )
                .message(arcstr::literal!("The signature is incorrect"))
                .param(arcstr::literal!("signature")));
            }
        };

        let hash = match Hash::from_str(self.recent_block_hash.as_str()) {
            Ok(v) => v,
            Err(_) => {
                return Err(AppError::new_with_status(
                    warp::http::StatusCode::BAD_REQUEST,
                    INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
                )
                .message(arcstr::literal!("The recentBlockHash is incorrect"))
                .param(arcstr::literal!("recentBlockHash")));
            }
        };

        Ok((signature, hash))
    }
}
