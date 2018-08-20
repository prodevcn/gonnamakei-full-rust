use arcstr::ArcStr;
use spl_token::instruction::initialize_account;
use spl_token::native_mint;
use spl_token::solana_program::program_pack::Pack;
use spl_token::solana_program::pubkey::Pubkey;

use commons::anchor_client::anchor_lang::Id;
use commons::anchor_spl;
use commons::clients::solana::SolanaClient;
use commons::database::collections::{ChallengeCollection, ParticipantCollection};
use commons::database::documents::{BetDBDocument, ParticipantDBDocument};
use commons::database::documents::{BetDBState, ChallengeMilestone};
use commons::database::types::game::GameMilestone;
use commons::database::types::{Address, DateTime};
use commons::database::{DBDocument, DBReference, NullableOption};
use commons::error::{AppError, INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE};
use commons::solana_sdk::instruction::Instruction;
use commons::solana_sdk::message::Message;
use commons::solana_sdk::signer::Signer;
use commons::solana_sdk::system_instruction;
use commons::utils::crypto::encode_to_base58;

use crate::commons::anchor_client::anchor_lang::InstructionData;
use crate::commons::anchor_client::anchor_lang::ToAccountMetas;
use crate::constants::BET_EXPIRATION;
use crate::models::responses::challenge::ChallengeBetResponse;
use crate::routes::RequestContextWithAuth;

pub async fn bet_service(
    request_context: RequestContextWithAuth<Address>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let context = request_context.app_context;

    // Find participant in DB.
    let participant_address = request_context.claims.address;
    let participant_key = participant_address.to_uuid();
    let participant_pubkey = participant_address.into();

    let participant_db_document =
        ParticipantCollection::get_by_key_or_reject(&participant_key, None).await?;

    // Find challenge in DB and blockchain.
    let challenge_address = request_context.request;
    let challenge_key = challenge_address.to_uuid();
    let challenge_pubkey = challenge_address.into();
    let challenge_program_client = context.solana_client.challenge_program_client();

    let challenge_db_document =
        ChallengeCollection::get_by_key_or_reject(&challenge_key, None).await?;
    let challenge = challenge_program_client
        .get_account(challenge_pubkey)
        .await?;
    let challenge_info = challenge.load_data().unwrap();

    // Check participant contains all data to play the challenge.
    for milestone in challenge_db_document.milestones.unwrap_as_ref() {
        match milestone {
            ChallengeMilestone::GameMilestone(milestone) => match &milestone.milestone {
                GameMilestone::ClashRoyale(_) => {
                    let data = &participant_db_document
                        .games_data
                        .unwrap_as_ref()
                        .clash_royale;

                    let is_ok = match data {
                        NullableOption::Value(v) => matches!(&v.tag, NullableOption::Value(_)),
                        _ => false,
                    };

                    if !is_ok {
                        return Err(AppError::new_with_status(
                            warp::http::StatusCode::BAD_REQUEST,
                            INPUT_VALIDATION_INCORRECT_STATE_ERROR_CODE,
                        )
                            .message(arcstr::literal!("The participant does not contain a ClashRoyale tag, please add it first"))
                            .param(arcstr::literal!("participant"))
                            .into());
                    }
                }
            },
            ChallengeMilestone::OtherChallenge(_) => todo!(),
        }
    }

    // Prepare transaction.
    let mut instructions = Vec::new();

    // Wrap SOL to a new account.
    let min_balance = context
        .solana_client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)
        .await?;
    let lamports = challenge_info.max_bet_amount.max(min_balance);
    let wrapped_sol_account_keypair = SolanaClient::create_address();
    let wrapped_sol_account_pubkey = wrapped_sol_account_keypair.pubkey();
    instructions.push(system_instruction::create_account(
        &participant_pubkey,
        &wrapped_sol_account_pubkey,
        lamports,
        spl_token::state::Account::LEN as u64,
        &commons::anchor_spl::token::ID,
    ));
    instructions.push(
        initialize_account(
            &spl_token::id(),
            &wrapped_sol_account_pubkey,
            &native_mint::id(),
            &participant_pubkey,
        )
        .unwrap(),
    );

    // Create bet.
    let bet_account_keypair = SolanaClient::create_address();
    let bet_account_pubkey = bet_account_keypair.pubkey();
    instructions.push(Instruction {
        program_id: commons::programs::gmi_bet::ID,
        accounts: commons::programs::gmi_bet::accounts::CreateInstruction {
            bet_account: bet_account_pubkey,
            owner_account: participant_pubkey,
            fungible_token_account: wrapped_sol_account_pubkey,
            token_program: anchor_spl::token::ID,
            system_program: commons::anchor_client::anchor_lang::System::id(),
        }
        .to_account_metas(None),
        data: commons::programs::gmi_bet::instruction::Create {}.data(),
    });

    // Apply bet to challenge.
    let challenge_pda_account = Pubkey::create_program_address(
        &[
            commons::programs::gmi_challenge::constants::CHALLENGE_PDA_SEED,
            &challenge_info.creator_account.to_bytes(),
            &challenge_info.token_accumulator_account.to_bytes(),
            &[challenge_info.bump_seed],
        ],
        &commons::programs::gmi_challenge::ID,
    )
    .unwrap();
    instructions.push(Instruction {
        program_id: commons::programs::gmi_challenge::ID,
        accounts: commons::programs::gmi_challenge::accounts::BetInstruction {
            challenge_account: challenge_pubkey,
            creator_account: challenge_info.creator_account,
            bet_account: bet_account_pubkey,
            bet_owner_account: participant_pubkey,
            challenge_pda_account,
            bet_program: commons::programs::gmi_bet::ID,
        }
        .to_account_metas(None),
        data: commons::programs::gmi_challenge::instruction::Bet {}.data(),
    });

    // Create transaction.
    let mut message = Message::new(&instructions, Some(&participant_pubkey));

    // Get blockhash and fees.
    let (recent_blockhash, fee_calculator) = context.solana_client.get_recent_blockhash().await?;
    let fee = fee_calculator.calculate_fee(&message);
    message.recent_blockhash = recent_blockhash;

    let serialized_message: ArcStr = encode_to_base58(&message.serialize()).into();

    // Update the bet in DB.
    let bet_db_document = BetDBDocument {
        db_key: Some(Address::from(bet_account_pubkey).to_uuid()),
        state: NullableOption::Value(BetDBState::WaitingForCreating),
        participant: NullableOption::Value(DBReference::Document(Box::new(
            ParticipantDBDocument {
                db_key: Some(participant_key.clone()),
                games_data: participant_db_document.games_data.clone(),
                ..Default::default()
            },
        ))),
        challenge: NullableOption::Value(DBReference::new_key(challenge_key)),
        keypair: NullableOption::Value(bet_account_keypair.to_base58_string().into()),
        fungible_token_account_keypair: NullableOption::Value(
            wrapped_sol_account_keypair.to_base58_string().into(),
        ),
        transaction: NullableOption::Value(serialized_message.clone()),
        db_expires_at: NullableOption::Value(DateTime::now().after_seconds(BET_EXPIRATION).into()),
        ..Default::default()
    };

    bet_db_document.insert(false).await?;

    // Return challenge info.
    let response = ChallengeBetResponse::new(bet_account_pubkey.into(), serialized_message, fee);

    Ok(warp::reply::json(&response))
}
