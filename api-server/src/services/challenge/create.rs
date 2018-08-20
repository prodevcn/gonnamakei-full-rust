use crate::models::requests::challenge::ChallengeCreateRequestBody;
use crate::routes::RequestContextWithAuth;

pub async fn create_service(
    request_context: RequestContextWithAuth<ChallengeCreateRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request = request_context.request;

    // Validate input.
    request.validate()?;

    // TODO Get the challenge from DB.

    // TODO Sign transaction.
    // TODO Send transaction.

    // TODO Get data from the account.

    // TODO Update the challenge in DB.

    // TODO Return challenge info.

    Ok(warp::reply())
}
