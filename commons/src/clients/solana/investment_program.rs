use std::sync::Arc;

use anchor_client::solana_client::rpc_response::RpcKeyedAccount;
use gmi_investment::state::{Investment, InvestmentState};
use solana_sdk::pubkey::Pubkey;

use crate::anchor_client::solana_client::rpc_client::RpcClient;
use crate::anchor_client::solana_client::rpc_config::RpcProgramAccountsConfig;
use crate::anchor_client::solana_client::rpc_filter::{
    Memcmp, MemcmpEncodedBytes, MemcmpEncoding, RpcFilterType,
};
use crate::anchor_client::solana_client::rpc_request::TokenAccountsFilter;
use crate::anchor_client::Client;
use crate::clients::solana::SolanaProgramAccount;
use crate::config::InitServiceConfig;
use crate::error::AppResult;
use crate::utils::crypto::encode_to_base58;

pub struct SolanaInvestmentProgramClient {
    rpc_client: Arc<RpcClient>,
}

impl SolanaInvestmentProgramClient {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(
        _config: &Arc<InitServiceConfig>,
        rpc_client: Arc<RpcClient>,
        _anchor_client: Arc<Client>,
    ) -> Self {
        SolanaInvestmentProgramClient { rpc_client }
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_account(&self, state: Pubkey) -> AppResult<SolanaProgramAccount<Investment>> {
        let rpc_client = self.rpc_client.clone();
        let account = tokio::task::spawn_blocking(move || rpc_client.get_account(&state))
            .await
            .unwrap()?;

        Ok(SolanaProgramAccount::new(account))
    }

    // TODO uncomment when needed.
    // pub async fn find_by_state(
    //     &self,
    //     state: InvestmentState,
    // ) -> AppResult<Vec<(Pubkey, SolanaProgramAccount<Investment>)>> {
    //     let serialized_state = encode_to_base58(&[state as u8]);
    //
    //     let config = RpcProgramAccountsConfig {
    //         filters: Some(vec![
    //             // The state.
    //             RpcFilterType::Memcmp(Memcmp {
    //                 offset: gmi_investment::constants::STATE_FIELD_OFFSET,
    //                 bytes: MemcmpEncodedBytes::Binary(serialized_state),
    //                 encoding: None,
    //             }),
    //         ]),
    //         ..Default::default()
    //     };
    //
    //     let rpc_client = self.rpc_client.clone();
    //     let results = tokio::task::spawn_blocking(move || {
    //         rpc_client.get_program_accounts_with_config(&gmi_investment::ID, config)
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

    pub async fn find_by_state_and_challenge(
        &self,
        state: InvestmentState,
        challenge: &Pubkey,
    ) -> AppResult<Vec<(Pubkey, SolanaProgramAccount<Investment>)>> {
        let serialized_state = encode_to_base58(&[state as u8]);
        let serialized_challenge = format!("{}", challenge);

        let config = RpcProgramAccountsConfig {
            filters: Some(vec![
                // The challenge.
                RpcFilterType::Memcmp(Memcmp {
                    offset: gmi_investment::constants::RECEIVER_FIELD_OFFSET,
                    bytes: MemcmpEncodedBytes::Base58(serialized_challenge),
                    encoding: Some(MemcmpEncoding::Binary),
                }),
                // The state.
                RpcFilterType::Memcmp(Memcmp {
                    offset: gmi_investment::constants::STATE_FIELD_OFFSET,
                    bytes: MemcmpEncodedBytes::Base58(serialized_state),
                    encoding: Some(MemcmpEncoding::Binary),
                }),
            ]),
            ..Default::default()
        };
        let rpc_client = self.rpc_client.clone();
        let accounts = tokio::task::spawn_blocking(move || {
            rpc_client.get_program_accounts_with_config(&gmi_investment::ID, config)
        })
        .await
        .unwrap()?;

        // let config = serde_json::json!([gmi_investment::ID.to_string(), {
        //     "encoding": "base64",
        //     "filters": [
        //     {
        //         "memcmp": {
        //             "offset": gmi_investment::constants::RECEIVER_FIELD_OFFSET,
        //             "bytes": serialized_challenge
        //         },
        //         "memcmp": {
        //             "offset": gmi_investment::constants::STATE_FIELD_OFFSET,
        //             "bytes": serialized_state
        //         }
        //     }
        // ]}]);
        // let rpc_client = self.rpc_client.clone();
        // let accounts: Vec<RpcKeyedAccount> = tokio::task::spawn_blocking(move || {
        //     rpc_client.send(RpcRequest::GetProgramAccounts, config)
        // })
        // .await
        // .unwrap()?;

        Ok(accounts
            .into_iter()
            .map(|(pubkey, account)| (pubkey, SolanaProgramAccount::new(account)))
            .collect())
    }

    pub async fn find_investments_nfts(
        &self,
        investments: &[&Investment],
    ) -> AppResult<Vec<Vec<RpcKeyedAccount>>> {
        let mut jobs = Vec::with_capacity(investments.len());

        for investment in investments {
            let pda_account = Pubkey::create_program_address(
                &[
                    gmi_investment::constants::INVESTMENT_PDA_SEED,
                    &investment.investor_account.to_bytes(),
                    &investment.fungible_token_account.to_bytes(),
                    &[investment.bump_seed],
                ],
                &gmi_investment::ID,
            )
            .unwrap();

            let rpc_client = self.rpc_client.clone();
            let job = tokio::task::spawn_blocking(move || {
                rpc_client.get_token_accounts_by_owner(
                    &pda_account,
                    TokenAccountsFilter::ProgramId(anchor_spl::token::ID),
                )
            });

            jobs.push(job);
        }

        let mut results = Vec::with_capacity(investments.len());

        for job in jobs {
            results.push(job.await.unwrap()?);
        }

        Ok(results)
    }
}
