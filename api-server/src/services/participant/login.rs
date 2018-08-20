use commons::constants::AUTHORIZATION_EXPIRATION_DELAY;
use commons::database::collections::AuthenticationCollection;
use commons::database::documents::{AuthenticationDBDocument, SignatureAction};
use commons::database::types::{DBUuid, DBUuidType, DateTime};
use commons::database::{DBDocument, NullableOption};

use crate::models::requests::participant::ParticipantLoginRequestBody;
use crate::models::responses::participant::ParticipantLoginResponse;
use crate::routes::RequestContext;
use crate::services::utils::validate_signature;

pub async fn login_service(
    request_context: RequestContext<ParticipantLoginRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request = request_context.request;

    // Validate input.
    request.validate()?;
    let signature = validate_signature(
        &request.id,
        request.signature.as_str(),
        SignatureAction::Login,
    )
    .await?;

    // Remove a previous authorization.
    AuthenticationCollection::remove_by_address_or_reject(signature.address.unwrap_as_ref())
        .await?;

    // Add new authorization.
    let token = DBUuid::new(DBUuidType::DBKey);
    let authorization = AuthenticationDBDocument {
        db_key: Some(token.clone()),
        db_expires_at: NullableOption::Value(
            DateTime::now()
                .after_seconds(AUTHORIZATION_EXPIRATION_DELAY)
                .into(),
        ),
        address: signature.address.clone(),
        ..Default::default()
    };

    authorization.insert(false).await?;

    let response = ParticipantLoginResponse::new(token);

    Ok(warp::reply::json(&response))
}
