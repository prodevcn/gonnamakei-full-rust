use commons::clients::solana::SolanaClient;
use commons::database::collections::SignatureCollection;
use commons::database::documents::{SignatureAction, SignatureDBDocument};
use commons::database::types::DBUuid;
use commons::database::NullableOption;
use commons::error::{
    AppError, AppResult, INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
    INPUT_VALIDATION_UNDEFINED_SIGNATURE_ERROR_CODE, INTERNAL_INCOMPLETE_ERROR_CODE,
};

pub async fn validate_signature(
    signature_key: &DBUuid,
    signature: &str,
    action: SignatureAction,
) -> AppResult<SignatureDBDocument> {
    let signature_doc = SignatureCollection::remove_and_get_by_key_or_reject(signature_key).await?;

    // Check action is the same.
    match signature_doc.action {
        NullableOption::Value(db_action) => {
            if db_action != action {
                return Err(AppError::new_with_status(
                    warp::http::StatusCode::BAD_REQUEST,
                    INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
                )
                .message(arcstr::literal!("Invalid signature action"))
                .param(arcstr::literal!("signature")));
            }
        }
        _ => {
            error!(
                "[Signature: {}] Missing action field in signature",
                signature_doc.db_key.unwrap()
            );

            return Err(AppError::new_with_status(
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                INTERNAL_INCOMPLETE_ERROR_CODE,
            ));
        }
    }

    let address = match &signature_doc.address {
        NullableOption::Value(v) => v,
        _ => {
            error!(
                "[Signature: {}] Missing address field in signature",
                signature_doc.db_key.unwrap()
            );

            return Err(AppError::new_with_status(
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                INTERNAL_INCOMPLETE_ERROR_CODE,
            ));
        }
    };

    let is_expired = match &signature_doc.db_expires_at {
        NullableOption::Value(v) => v.is_expired(),
        _ => true,
    };

    if is_expired {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_UNDEFINED_SIGNATURE_ERROR_CODE,
        )
        .message(arcstr::literal!("Undefined signature")));
    }

    let nonce = signature_doc.db_key.as_ref().unwrap();
    let message = action.build_message(address.as_string().as_str(), nonce.as_string().as_str());

    if !SolanaClient::verify_signature(address.as_string().as_str(), message.as_str(), signature) {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
        )
        .message(arcstr::literal!("Invalid signature"))
        .param(arcstr::literal!("signature")));
    }

    Ok(signature_doc)
}
