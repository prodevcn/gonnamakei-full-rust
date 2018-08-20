use std::convert::TryFrom;

use commons::database::collections::ChallengeCollection;
use commons::database::documents::{ChallengeAPIDocument, ChallengeDBDocument};
use commons::database::types::Address;
use commons::database::{DBCollection, NullableOption};
use commons::server::responses::PaginatedResponse;

use crate::models::requests::challenge::ChallengeListRequestBody;
use crate::routes::RequestContext;

pub async fn list_service(
    request_context: RequestContext<ChallengeListRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut request = request_context.request;
    let db_config = request_context.config.db_config().await;
    let api_config = &db_config.api;

    // Validate input.
    request.validate()?;
    request.responses.normalize(api_config)?;

    let page = request.responses.page;
    let rows_per_page = request.responses.rows_per_page;

    // Generate query.
    let collection = ChallengeCollection::instance();
    let query = request.responses.build_aql(ChallengeCollection::name())?;

    // Send the query.
    let aql_result = collection
        .send_generic_aql::<ChallengeDBDocument>(&query)
        .await?;

    // Map results.
    let mut mapped_challenges = Vec::with_capacity(aql_result.results.len());

    // Get challenge blockchain data.
    let challenge_program_client = request_context
        .app_context
        .solana_client
        .challenge_program_client();
    let pubkeys: Vec<_> = aql_result
        .results
        .iter()
        .map(|v| Address::try_from(v.db_key.clone().unwrap()).unwrap().into())
        .collect();
    let blockchain_data = challenge_program_client.get_many_accounts(pubkeys).await?;

    for (challenge, blockchain_data) in aql_result.results.into_iter().zip(blockchain_data) {
        let mut mapped_challenge: ChallengeAPIDocument = challenge.into();
        let loader = blockchain_data.unwrap().1;
        let data = loader.load_data().unwrap();
        mapped_challenge.blockchain_info = NullableOption::Value(data.into());

        mapped_challenge.remove_sensible_info();
        mapped_challenges.push(mapped_challenge);
    }

    // Make result.
    let response = PaginatedResponse {
        count: mapped_challenges.len() as u64,
        total_count: aql_result.full_count,
        page,
        rows_per_page,
        total_pages: aql_result.full_count.map(|v| {
            let mut result = v / rows_per_page;

            if v % rows_per_page != 0 {
                result += 1;
            }

            result
        }),
        results: mapped_challenges,
    };

    Ok(warp::reply::json(&response))
}
