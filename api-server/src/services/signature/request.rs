use commons::constants::SIGNATURE_EXPIRATION_DELAY;
use commons::database::documents::{SignatureAction, SignatureDBDocument};
use commons::database::types::{DBUuid, DBUuidType, DateTime};
use commons::database::{DBDocument, NullableOption};

use crate::models::requests::signature::SignatureRequestBody;
use crate::models::responses::signature::SignatureRequestResponse;
use crate::routes::RequestContext;

pub async fn request_service(
    request_context: RequestContext<SignatureRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request = &request_context.request;

    // Validate input.
    request.validate()?;

    // Persist the signature.
    let db_key = DBUuid::new(DBUuidType::Nonce);
    let action: SignatureAction = request.action.into();
    let message = action.build_message(
        request.address.as_string().as_str(),
        db_key.as_string().as_str(),
    );
    let signature = SignatureDBDocument {
        db_key: Some(db_key.clone()),
        action: NullableOption::Value(action),
        address: NullableOption::Value(request.address.clone()),
        db_expires_at: NullableOption::Value(
            DateTime::now()
                .after_seconds(SIGNATURE_EXPIRATION_DELAY)
                .into(),
        ),
        ..Default::default()
    };

    signature.insert(false).await?;

    // Send response.
    let response = SignatureRequestResponse::new(db_key, message.into());

    Ok(warp::reply::json(&response))
}
