#[macro_use]
extern crate log;

use std::sync::Arc;

use commons::anchor_client::solana_sdk::signer::keypair::Keypair;
use commons::config::read_app_config;
use commons::database::collections::CollectionKind;
use commons::database::documents::ChallengeDBDocument;
use commons::database::documents::ChallengeMilestone;
use commons::database::documents::GameChallengeMilestone;
use commons::database::types::conditions::{OptionCondition, OrderedCondition};
use commons::database::types::game::ClashRoyaleGameMode;
use commons::database::types::game::ClashRoyaleMatchConditions;
use commons::database::types::game::ClashRoyaleTeamConditions;
use commons::database::types::game::{ClashRoyaleMilestone, GameMilestone};
use commons::database::types::{Address, DateTime};
use commons::database::{init_db_connection, DBDocument, NullableOption};
use commons::solana_sdk::signer::Signer;
use commons::test::commons::reset_db;
use commons::utils::init_logger;

use crate::errors::ServerResult;

mod errors;

#[tokio::main]
async fn main() {
    init_logger();

    let mut args = std::env::args();

    // First: name of the executable.
    let _ = args.next();

    let command = args
        .next()
        .expect("The scripts need a command in the first argument");

    let result = match command.as_str() {
        "clean_db" => clean_db_command().await,
        "reset_db" => reset_db_command().await,
        "insert_challenges" => insert_challenges_in_db().await,
        _ => {
            error!("Undefined command: {}", command);
            return;
        }
    };

    if let Err(e) = result {
        error!("{}", e);
        std::process::exit(1)
    }
}

async fn clean_db_command() -> ServerResult<()> {
    let config = Arc::new(read_app_config()?);
    let db_info = init_db_connection(&config).await?;

    // Drop collections.
    let jobs: Vec<_> = CollectionKind::enum_list()
        .iter()
        .map(|kind| tokio::spawn(kind.drop_collection()))
        .collect();

    // Remove all custom aql functions.
    db_info.remove_all_aql_function().await?;

    for job in jobs {
        let _ = job.await;
    }

    Ok(())
}

async fn reset_db_command() -> ServerResult<()> {
    let config = Arc::new(read_app_config()?);
    let db_info = init_db_connection(&config).await?;

    reset_db(&db_info).await?;

    Ok(())
}

async fn insert_challenges_in_db() -> ServerResult<()> {
    let config = Arc::new(read_app_config()?);
    let _db_info = init_db_connection(&config).await?;

    let challenge_keypair_bytes = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let challenge_keypair = Keypair::from_bytes(&challenge_keypair_bytes[..]).unwrap();
    let challenge_address = Address::from(challenge_keypair.pubkey());
    let challenge_key = challenge_address.to_uuid();

    let challenge = ChallengeDBDocument {
        db_key: Some(challenge_key.clone()),
        name: NullableOption::Value(arcstr::literal!("[Test] Clash Royale 1x1")),
        description: NullableOption::Value(arcstr::literal!(
            "Get 1 crown 1x to win the respect of GonnaMakeIt."
        )),
        keypair: NullableOption::Value(challenge_keypair.to_base58_string().into()),
        milestones: NullableOption::Value(vec![ChallengeMilestone::GameMilestone(
            GameChallengeMilestone {
                milestone: GameMilestone::ClashRoyale(ClashRoyaleMilestone::WinMatches(vec![
                    ClashRoyaleMatchConditions {
                        allow_friends: Some(false),
                        game_mode: Some(OptionCondition::AnyOf(vec![
                            ClashRoyaleGameMode::OneVsOne,
                        ])),
                        team: Some(ClashRoyaleTeamConditions {
                            crowns: Some(OrderedCondition::AnyOf(vec![1])),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ])),
            },
        )]),
        created_at: NullableOption::Value(DateTime::now()),
        nft_image_url: NullableOption::Value(arcstr::literal!("/assets/images/sinoloa.png")),
        ..Default::default()
    };

    challenge.insert(false).await?;

    Ok(())
}
