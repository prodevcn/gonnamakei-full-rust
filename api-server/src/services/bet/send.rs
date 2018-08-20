use std::convert::TryFrom;

use commons::constants::REMOTE_MUTEX_TIMEOUT;
use commons::data::RemoteMutexGuard;
use commons::database::documents::{BetDBDocument, BetDBState};
use commons::database::types::{Address, DateTime};
use commons::database::{DBDocument, NullableOption};
use commons::error::{
    AppError, INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE,
    INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE, INPUT_VALIDATION_UNDEFINED_BET_ERROR_CODE,
};
use commons::server::requests::RequestWithParam;
use commons::solana_sdk::message::Message;
use commons::solana_sdk::signature::Keypair;
use commons::solana_sdk::signer::Signer;
use commons::solana_sdk::transaction::Transaction;
use commons::utils::crypto::decode_from_base58;

use crate::models::requests::bet::BetSendRequestBody;
use crate::models::responses::bet::BetSendResponse;
use crate::routes::RequestContextWithAuth;

pub async fn send_service(
    request_context: RequestContextWithAuth<RequestWithParam<Address, BetSendRequestBody>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let context = request_context.app_context;
    let request = request_context.request.body;

    // Validate input.
    let (signature, recent_blockhash) = request.validate()?;

    // Find bet in DB.
    let bet_address = request_context.request.param;
    let bet_key = bet_address.to_uuid();

    let (mut bet_db_document, _) = match RemoteMutexGuard::<BetDBDocument>::acquire_with_timeout(
        &bet_key,
        REMOTE_MUTEX_TIMEOUT,
        None,
    )
    .await?
    {
        Some(v) => v,
        None => {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_BET_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined bet"))
            .into());
        }
    };

    // Check state.
    if !bet_db_document.transaction.is_value() {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE,
        )
        .message(arcstr::literal!("The bet is already applied"))
        .into());
    }

    let challenge_key = bet_db_document.challenge.unwrap_as_ref().key();

    // Create transaction.
    let message = bet_db_document.transaction.unwrap();
    let message = decode_from_base58(message.as_bytes()).unwrap();
    let mut message: Message = bincode::deserialize(&message).unwrap();
    message.recent_blockhash = recent_blockhash;

    let mut transaction = Transaction::new_unsigned(message);

    // Apply the signature to the transaction.
    {
        let participant_key = bet_db_document.participant.unwrap_as_ref().key();
        let participant_address = Address::try_from(participant_key).unwrap();

        let positions = transaction
            .get_signing_keypair_positions(&[participant_address.into()])
            .unwrap();
        if positions.iter().any(|pos| pos.is_none()) {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
            )
            .message(arcstr::literal!("Incorrect signature"))
            .param(arcstr::literal!("signature"))
            .into());
        }

        let positions: Vec<usize> = positions.iter().map(|pos| pos.unwrap()).collect();

        for position in positions {
            transaction.signatures[position] = signature;
        }
    }

    // Sign transaction.
    {
        let wallet_config = request_context.config.wallet.as_ref().unwrap();
        let wallet_keypair = wallet_config.keypair();
        let keypair1 = Keypair::from_base58_string(bet_db_document.keypair.unwrap_as_ref());
        let keypair2 = Keypair::from_base58_string(
            bet_db_document
                .fungible_token_account_keypair
                .unwrap_as_ref(),
        );
        let signers: &[&dyn Signer; 3] = &[&wallet_keypair, &keypair1, &keypair2];
        transaction
            .try_partial_sign(signers, recent_blockhash)
            .map_err(AppError::from)?;
    }

    if !transaction.is_signed() {
        return Err(AppError::new_with_status(
            warp::http::StatusCode::BAD_REQUEST,
            INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE,
        )
        .message(arcstr::literal!("Cannot sign too old transaction"))
        .into());
    }

    // Send transaction.
    let signature = context.solana_client.send_transaction(transaction).await?;

    remote_info!("[bet::create] Tx: {}", signature);

    // Update the bet in DB.
    let now = DateTime::now();
    {
        bet_db_document.state = NullableOption::Value(BetDBState::Created);
        bet_db_document.transaction = NullableOption::Null;
        bet_db_document.db_expires_at = NullableOption::Null;
        bet_db_document.created_at = NullableOption::Value(now.clone());

        bet_db_document.insert_or_update(true).await?;
    }

    // Get challenge from blockchain.
    let challenge_program_client = context.solana_client.challenge_program_client();
    let challenge = challenge_program_client
        .get_account(Address::try_from(challenge_key).unwrap().into())
        .await?;
    let challenge_info = challenge.load_data().unwrap();
    let timeout = now.after_seconds(challenge_info.bets_expiration_delay as u64);

    // Return challenge info.
    let response = BetSendResponse::new(now, timeout);

    Ok(warp::reply::json(&response))
}
