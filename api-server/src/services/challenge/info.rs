use commons::database::collections::ChallengeCollection;
use commons::database::documents::ChallengeAPIDocument;
use commons::database::types::Address;

use crate::routes::RequestContext;

pub async fn info_service(
    request_context: RequestContext<Address>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let challenge_address = request_context.request;
    let challenge_key = challenge_address.to_uuid();

    let challenge = ChallengeCollection::get_by_key_or_reject(&challenge_key, None).await?;

    // Generate response.
    let mut challenge_response: ChallengeAPIDocument = challenge.into();
    challenge_response.remove_sensible_info();

    // TODO in the future normalize the data.

    Ok(warp::reply::json(&challenge_response))
}
