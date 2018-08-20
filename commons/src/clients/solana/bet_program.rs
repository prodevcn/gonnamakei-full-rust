use std::sync::Arc;

use anchor_client::solana_client::rpc_config::RpcProgramAccountsConfig;
use gmi_bet::state::{Bet, BetState};
use solana_sdk::pubkey::Pubkey;

use crate::anchor_client::solana_client::rpc_client::RpcClient;
use crate::anchor_client::solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use crate::anchor_client::Client;
use crate::clients::solana::SolanaProgramAccount;
use crate::config::InitServiceConfig;
use crate::error::AppResult;
use crate::utils::crypto::encode_to_base58;

pub struct SolanaBetProgramClient {
    rpc_client: Arc<RpcClient>,
}

impl SolanaBetProgramClient {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(
        _config: &Arc<InitServiceConfig>,
        rpc_client: Arc<RpcClient>,
        _anchor_client: Arc<Client>,
    ) -> Self {
        SolanaBetProgramClient { rpc_client }
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_account(&self, state: Pubkey) -> AppResult<SolanaProgramAccount<Bet>> {
        let rpc_client = self.rpc_client.clone();
        let account = tokio::task::spawn_blocking(move || rpc_client.get_account(&state))
            .await
            .unwrap()?;

        Ok(SolanaProgramAccount::new(account))
    }

    pub async fn find_by_state(
        &self,
        state: BetState,
    ) -> AppResult<Vec<(Pubkey, SolanaProgramAccount<Bet>)>> {
        let serialized_state = encode_to_base58(&[state as u8]);

        let config = RpcProgramAccountsConfig {
            filters: Some(vec![
                // The state.
                RpcFilterType::Memcmp(Memcmp {
                    offset: gmi_bet::constants::STATE_FIELD_OFFSET,
                    bytes: MemcmpEncodedBytes::Base58(serialized_state),
                    encoding: None,
                }),
            ]),
            ..Default::default()
        };

        let rpc_client = self.rpc_client.clone();
        let results = tokio::task::spawn_blocking(move || {
            rpc_client.get_program_accounts_with_config(&gmi_bet::ID, config)
        })
        .await
        .unwrap()?;

        let mapped_results = results
            .into_iter()
            .map(|(key, account)| (key, SolanaProgramAccount::new(account)))
            .collect();

        Ok(mapped_results)
    }

    // TODO uncomment when needed.
    // pub async fn find_by_state_and_challenge(
    //     &self,
    //     state: BetState,
    //     challenge: &Pubkey,
    // ) -> AppResult<Vec<(Pubkey, SolanaProgramAccount<Bet>)>> {
    //     let serialized_state = encode_to_base58(&[state as u8]);
    //     let serialized_challenge = format!("{}", challenge);
    //
    //     let config = RpcProgramAccountsConfig {
    //         filters: Some(vec![
    //             // The challenge.
    //             RpcFilterType::Memcmp(Memcmp {
    //                 offset: gmi_bet::constants::RECEIVER_FIELD_OFFSET,
    //                 bytes: MemcmpEncodedBytes::Binary(serialized_challenge),
    //                 encoding: None,
    //             }),
    //             // The state.
    //             RpcFilterType::Memcmp(Memcmp {
    //                 offset: gmi_bet::constants::STATE_FIELD_OFFSET,
    //                 bytes: MemcmpEncodedBytes::Binary(serialized_state),
    //                 encoding: None,
    //             }),
    //         ]),
    //         ..Default::default()
    //     };
    //
    //     let rpc_client = self.rpc_client.clone();
    //     let results = tokio::task::spawn_blocking(move || {
    //         rpc_client.get_program_accounts_with_config(&gmi_bet::ID, config)
    //     })
    //     .await
    //     .unwrap()?;
    //
    //     let mapped_results = results
    //         .into_iter()
    //         .map(|(key, account)| (key, SolanaProgramAccount::new(account)))
    //         .collect();
    //
    //     Ok(mapped_results)
    // }
}
