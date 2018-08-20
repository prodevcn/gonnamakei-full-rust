use std::sync::Arc;

use chrono::{TimeZone, Utc};

use commons::clients::games::ClashRoyaleBattlelogResponse;
use commons::database::collections::{BetCollection, ChallengeCollection};
use commons::database::documents::{BetDBDocument, ChallengeDBDocument};
use commons::database::documents::{BetDBState, ChallengeMilestone};
use commons::database::types::game::{ClashRoyaleMilestone, GameMilestone};
use commons::database::types::{Address, DateTime};
use commons::database::{DBDocument, NullableOption};
use commons::error::{
    AppError, AppResult, INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE,
    INPUT_VALIDATION_UNDEFINED_BET_ERROR_CODE,
};
use commons::programs::gmi_bet::state::{Bet, BetState};

use crate::constants::BET_EXPIRATION_THRESHOLD;
use crate::context::AppContext;
use crate::models::responses::challenge::{ChallengeCheckResponse, ChallengeCheckResponseStatus};
use crate::routes::RequestContext;

pub async fn check_service(
    request_context: RequestContext<Address>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let context = request_context.app_context;

    // Find bet in DB and blockchain.
    let bet_address = request_context.request;
    let bet_key = bet_address.to_uuid();
    let bet_pubkey = bet_address.into();
    let bet_program_client = context.solana_client.bet_program_client();

    let bet_db_document = BetCollection::get_by_key_or_reject(&bet_key, None).await?;
    let bet = match bet_program_client.get_account(bet_pubkey).await {
        Ok(v) => v,
        Err(_) => {
            // The bet is not yet confirmed.
            let response = ChallengeCheckResponse::new(ChallengeCheckResponseStatus::NotInitiated);
            return Ok(warp::reply::json(&response));
        }
    };
    let bet_info = match bet.load_data() {
        Some(v) => v,
        None => {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_BET_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined bet"))
            .param(arcstr::literal!("address"))
            .into());
        }
    };

    // Check bet status.
    if bet_info.state != BetState::Applied {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE,
        )
        .message(arcstr::literal!(
            "Only Bets in not yet validated state can be validated"
        ))
        .param(arcstr::literal!("address"))
        .into());
    }

    // Find challenge in DB and blockchain.
    let challenge_address = Address::from(bet_info.receiver_account);
    let challenge_key = challenge_address.to_uuid();
    let challenge_pubkey = challenge_address.into();
    let challenge_program_client = context.solana_client.challenge_program_client();

    let challenge_db_document =
        ChallengeCollection::get_by_key_or_reject(&challenge_key, None).await?;
    let challenge = challenge_program_client
        .get_account(challenge_pubkey)
        .await?;
    let challenge_info = challenge.load_data().unwrap();

    // Check expiration.
    let bet_expiration = DateTime::new(Utc.timestamp(bet_info.expires_at, 0));

    // Verify the challenge.
    // If the time is around after a threshold, the challenge is also verified.
    let result_status = if bet_expiration.is_expired_with_threshold(BET_EXPIRATION_THRESHOLD) {
        ChallengeCheckResponseStatus::Expired
    } else {
        verify_challenge(&context, &challenge_db_document, &bet_db_document, bet_info).await?
    };

    let mut updated_document = BetDBDocument::default();

    match result_status {
        ChallengeCheckResponseStatus::Won => {
            // Send call to validate the bet in the blockchain.
            match challenge_program_client
                .call_validate_bet_and_send_reward(
                    &challenge_pubkey,
                    challenge_info,
                    &bet_pubkey,
                    bet_info,
                )
                .await?
            {
                Some((signature, nft_key)) => {
                    updated_document.won_nft = NullableOption::Value(nft_key.into());
                    remote_info!(
                        "[challenge::validate] Won nft: {}. Tx: {}",
                        nft_key,
                        signature
                    );
                }
                None => {
                    // Validate the bet without sending a reward in case there is no more.
                    let signature = challenge_program_client
                        .call_validate_bet(
                            &challenge_pubkey,
                            challenge_info,
                            &bet_pubkey,
                            bet_info,
                            true,
                        )
                        .await?;

                    remote_info!("[challenge::validate] Won. Tx: {}", signature);
                }
            };

            updated_document.state = NullableOption::Value(BetDBState::Won);
        }
        ChallengeCheckResponseStatus::Lost => {
            // Send call to validate the bet in the blockchain.
            let signature = challenge_program_client
                .call_validate_bet(
                    &challenge_pubkey,
                    challenge_info,
                    &bet_pubkey,
                    bet_info,
                    false,
                )
                .await?;

            remote_info!("[challenge::validate] Lost. Tx: {}", signature);
            updated_document.state = NullableOption::Value(BetDBState::Lost);
        }
        ChallengeCheckResponseStatus::Expired => {
            // Send call to check the bet in the blockchain.
            updated_document.state = match challenge_program_client
                .call_check_bet(&challenge_pubkey, challenge_info, &bet_pubkey, bet_info)
                .await
            {
                Ok(signature) => {
                    remote_info!("[challenge::check_bet] Expired. Tx: {}", signature);
                    NullableOption::Value(BetDBState::Expired)
                }
                Err(_) => NullableOption::Value(BetDBState::ExpiredNotInBlockchain),
            };
        }
        ChallengeCheckResponseStatus::NotInitiated => {}
        ChallengeCheckResponseStatus::Initiated => {}
    };

    // Update bet.
    if !updated_document.is_all_missing() {
        updated_document.db_key = bet_db_document.db_key.clone();
        updated_document.update(true).await?;
    }

    let response = ChallengeCheckResponse::new(result_status);
    Ok(warp::reply::json(&response))
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

async fn verify_challenge(
    context: &Arc<AppContext>,
    challenge: &ChallengeDBDocument,
    bet: &BetDBDocument,
    bet_info: &Bet,
) -> AppResult<ChallengeCheckResponseStatus> {
    let milestones = challenge.milestones.unwrap_as_ref();
    let bet_start_date = bet.created_at.unwrap_as_ref();
    let bet_info = DateTime::new(Utc::timestamp(&Utc, bet_info.expires_at, 0));
    let games_data = bet
        .participant
        .unwrap_as_ref()
        .unwrap_document_as_ref()
        .games_data
        .unwrap_as_ref();

    for milestone in milestones {
        match milestone {
            ChallengeMilestone::GameMilestone(milestone) => match &milestone.milestone {
                GameMilestone::ClashRoyale(milestone) => match milestone {
                    ClashRoyaleMilestone::WinMatches(conditions) => {
                        let client = context.game_clients.clash_royale.clone();
                        let battlelogs = client
                            .get_player_battlelog(
                                games_data.clash_royale.unwrap_as_ref().tag.unwrap_as_ref(),
                            )
                            .await?;

                        // Filter to get only those battlelogs that happened after the starting date.
                        let battlelogs =
                            get_battlelog_matches_after_date(bet_start_date, &battlelogs);

                        // Initial checks.
                        if battlelogs.is_empty() {
                            return Ok(ChallengeCheckResponseStatus::NotInitiated);
                        }

                        if battlelogs.len() < conditions.len() {
                            return Ok(ChallengeCheckResponseStatus::Initiated);
                        }

                        // Pick only the necessary ones.
                        let battlelogs = &battlelogs[(battlelogs.len() - conditions.len())..];

                        // Verify last match first.
                        for (battlelog, condition) in battlelogs.iter().rev().zip(conditions.iter())
                        {
                            // When any of the matches are after the final date, the user has lost.
                            let match_date = &battlelog.battle_time;
                            if match_date.0 >= bet_info.0 {
                                return Ok(ChallengeCheckResponseStatus::Lost);
                            }

                            // Verify the challenge.
                            if !condition.verify_response(battlelog) {
                                return Ok(ChallengeCheckResponseStatus::Lost);
                            }
                        }
                    }
                    ClashRoyaleMilestone::Achievement(_) => todo!(),
                },
            },
            ChallengeMilestone::OtherChallenge(_) => todo!(),
        }
    }

    Ok(ChallengeCheckResponseStatus::Won)
}

fn get_battlelog_matches_after_date<'a>(
    start_date: &DateTime,
    battlelogs: &'a [ClashRoyaleBattlelogResponse],
) -> &'a [ClashRoyaleBattlelogResponse] {
    for i in 0..battlelogs.len() {
        let battlelog = &battlelogs[i];
        let battle_time = &battlelog.battle_time;

        if battle_time.0 < start_date.0 {
            return &battlelogs[0..i];
        }
    }

    battlelogs
}
