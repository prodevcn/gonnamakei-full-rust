use std::collections::{HashMap, HashSet};

use commons::database::collections::ParticipantCollection;
use commons::database::documents::ParticipantAPIDocument;
use commons::database::types::{DBUuid, DBUuidType};
use commons::database::DBCollection;

use crate::models::requests::participant::ParticipantGetRequestBody;
use crate::models::responses::participant::ParticipantActiveBetGetResponse;
use crate::models::responses::participant::ParticipantGetResponse;
use crate::routes::RequestContextWithAuth;

pub async fn get_service(
    request_context: RequestContextWithAuth<ParticipantGetRequestBody>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let context = request_context.app_context;
    let request = request_context.request;

    // Validate input.
    request.validate()?;

    // Get participant.
    let participant_address = request_context.claims.address;
    let participant_key = DBUuid::new_with_content(
        participant_address.as_string().as_str(),
        DBUuidType::Address,
    )
    .unwrap();

    let participant = if request.return_fields {
        let collection = ParticipantCollection::instance();
        let participant = collection
            .get_one_by_key(&participant_key, None)
            .await
            .unwrap_or(None);

        participant.map(|participant| {
            let mut mapped: ParticipantAPIDocument = participant.into();
            mapped.remove_sensible_info();
            mapped
        })
    } else {
        None
    };

    // Get bets.
    let active_bets = if request.return_active_bets {
        let bet_program_client = context.solana_client.bet_program_client();
        let challenge_program_client = context.solana_client.challenge_program_client();
        let bets = bet_program_client
            .find_by_state(commons::programs::gmi_bet::state::BetState::Applied)
            .await?;

        // Get challenges.
        let challenge_keys: HashSet<_> = bets
            .iter()
            .map(|(_, account)| {
                let data = account.load_data().unwrap();
                data.receiver_account
            })
            .collect();
        let challenge_keys: Vec<_> = challenge_keys.into_iter().collect();
        let challenges = challenge_program_client
            .get_many_accounts(challenge_keys)
            .await?;
        let challenges: HashMap<_, _> = challenges.into_iter().flatten().collect();

        // Filter bets.
        let bets: Vec<_> = bets
            .into_iter()
            .filter_map(|(bet_key, bet_account)| {
                let bet = bet_account.load_data().unwrap();

                match challenges.get(&bet.receiver_account) {
                    Some(challenge_account) => {
                        let challenge = challenge_account.load_data().unwrap();

                        Some(ParticipantActiveBetGetResponse::new(
                            bet_key.into(),
                            bet.into(),
                            challenge.into(),
                        ))
                    }
                    None => None,
                }
            })
            .collect();

        Some(bets)
    } else {
        None
    };

    let response = ParticipantGetResponse::new(participant, active_bets);

    Ok(warp::reply::json(&response))
}
