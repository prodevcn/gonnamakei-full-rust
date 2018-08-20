use std::panic;
use std::sync::Arc;

use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::Client;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::transaction::Transaction;

pub use bet_program::*;
pub use challenge_program::*;
pub use investment_program::*;
pub use loader::*;

use crate::anchor_client::anchor_lang::solana_program::fee_calculator::FeeCalculator;
use crate::anchor_client::anchor_lang::solana_program::hash::Hash;
use crate::anchor_client::Cluster;
use crate::config::InitServiceConfig;
use crate::error::AppResult;
use crate::solana_sdk::account::Account;
use crate::utils::crypto::{decode_from_base58, decode_from_base64};

mod bet_program;
mod challenge_program;
mod investment_program;
mod loader;
pub mod models;

pub struct SolanaClient {
    rpc_client: Arc<RpcClient>,

    // Programs
    bet_program: Arc<SolanaBetProgramClient>,
    challenge_program: Arc<SolanaChallengeProgramClient>,
}

impl SolanaClient {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(config: &Arc<InitServiceConfig>) -> Self {
        let solana_config = config.solana.as_ref().unwrap();
        let wallet_config = config.wallet.as_ref().unwrap();
        let rpc_client = Arc::new(RpcClient::new(solana_config.cluster_url.to_string()));
        let anchor_client = Arc::new(Client::new(
            Cluster::Custom(solana_config.cluster_url.to_string(), String::new()),
            wallet_config.keypair(),
        ));

        let investment_program = Arc::new(SolanaInvestmentProgramClient::new(
            config,
            rpc_client.clone(),
            anchor_client.clone(),
        ));

        SolanaClient {
            bet_program: Arc::new(SolanaBetProgramClient::new(
                config,
                rpc_client.clone(),
                anchor_client.clone(),
            )),
            challenge_program: Arc::new(SolanaChallengeProgramClient::new(
                config,
                rpc_client.clone(),
                anchor_client,
                investment_program,
            )),
            rpc_client,
        }
    }

    // GETTERS ----------------------------------------------------------------

    pub fn bet_program_client(&self) -> &Arc<SolanaBetProgramClient> {
        &self.bet_program
    }

    pub fn challenge_program_client(&self) -> &Arc<SolanaChallengeProgramClient> {
        &self.challenge_program
    }

    // METHODS ----------------------------------------------------------------

    /// `account` is in base58.
    /// `signature` is in base64.
    pub fn verify_signature(account: &str, nonce: &str, signature: &str) -> bool {
        let account = match decode_from_base58(account.as_bytes()) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let signature = match decode_from_base64(signature.as_bytes()) {
            Ok(v) => v,
            Err(_) => return false,
        };

        panic::catch_unwind(|| {
            let signature = Signature::new(&signature);
            signature.verify(&account, nonce.as_bytes())
        })
        .unwrap_or(false)
    }

    /// Creates a new random address to use in the blockchain.
    pub fn create_address() -> Keypair {
        Keypair::new()
    }

    /// Gets the information of an account.
    pub async fn get_account_info(&self, account: Pubkey) -> AppResult<Account> {
        let rpc_client = self.rpc_client.clone();
        Ok(
            tokio::task::spawn_blocking(move || rpc_client.get_account(&account))
                .await
                .unwrap()?,
        )
    }

    /// Gets the fee calculator.
    pub async fn get_recent_blockhash(&self) -> AppResult<(Hash, FeeCalculator)> {
        let rpc_client = self.rpc_client.clone();
        Ok(
            tokio::task::spawn_blocking(move || rpc_client.get_recent_blockhash())
                .await
                .unwrap()?,
        )
    }

    /// Gets the minimum balance for rent exception.
    pub async fn get_minimum_balance_for_rent_exemption(&self, value: usize) -> AppResult<u64> {
        let rpc_client = self.rpc_client.clone();
        Ok(tokio::task::spawn_blocking(move || {
            rpc_client.get_minimum_balance_for_rent_exemption(value)
        })
        .await
        .unwrap()?)
    }

    /// Sends a transaction to the blockchain.
    pub async fn send_transaction(&self, transaction: Transaction) -> AppResult<Signature> {
        let rpc_client = self.rpc_client.clone();
        Ok(
            tokio::task::spawn_blocking(move || rpc_client.send_transaction(&transaction))
                .await
                .unwrap()?,
        )
    }
}
