use commons::clients::solana::models::SerializableChallenge;
use commons::database::collections::ChallengeCollection;
use commons::database::documents::ChallengeAPIDocument;
use commons::database::types::Address;
use commons::server::requests::RequestWithParam;

use crate::models::requests::challenge::ChallengeGetRequestBody;
use crate::models::responses::challenge::ChallengeGetResponse;
use crate::routes::RequestContext;

pub async fn get_service(
    request_context: RequestContext<RequestWithParam<Address, ChallengeGetRequestBody>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let challenge_address = request_context.request.param;
    let challenge_key = challenge_address.to_uuid();
    let challenge_pubkey = challenge_address.into();
    let request = request_context.request.body;

    // Validate input.
    request.validate()?;

    // Get challenge.
    let challenge_response = if request.return_fields {
        let challenge = ChallengeCollection::get_by_key_or_reject(&challenge_key, None).await?;

        // Generate response.
        let mut challenge_response: ChallengeAPIDocument = challenge.into();
        challenge_response.remove_sensible_info();

        Some(challenge_response)
    } else {
        None
    };

    // Get challenge info from blockchain.
    let blockchain_data = if request.return_blockchain_data {
        let challenge_program_client = request_context
            .app_context
            .solana_client
            .challenge_program_client();

        let challenge = challenge_program_client
            .get_account(challenge_pubkey)
            .await?;
        let challenge_info = challenge.load_data().unwrap();

        Some(SerializableChallenge::from(challenge_info))
    } else {
        None
    };

    let response = ChallengeGetResponse::new(challenge_response, blockchain_data);

    Ok(warp::reply::json(&response))
}
