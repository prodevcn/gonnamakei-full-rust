use crate::models::requests::challenge::ChallengeCreateConfigRequestBody;
use crate::routes::RequestContextWithAuth;

pub async fn create_config_service(
    request_context: RequestContextWithAuth<ChallengeCreateConfigRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request = request_context.request;

    // Validate input.
    request.validate()?;

    // TODO Prepare transaction.
    // TODO Serialize transaction.
    // TODO Store the challenge with an expiration in DB.
    // TODO Return the transaction.

    Ok(warp::reply())
}
