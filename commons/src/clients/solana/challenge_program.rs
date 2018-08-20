use std::str::FromStr;
use std::sync::Arc;

use anchor_client::anchor_lang::InstructionData;
use anchor_client::anchor_lang::ToAccountMetas;
use gmi_bet::state::Bet;
use gmi_investment::state::InvestmentState;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

use crate::anchor_client::solana_client::rpc_client::RpcClient;
use crate::anchor_client::Client;
use crate::clients::solana::{SolanaInvestmentProgramClient, SolanaProgramAccount};
use crate::config::InitServiceConfig;
use crate::error::{AppError, AppResult};
use crate::programs::gmi_challenge::state::Challenge;
use crate::programs::gmi_investment::state::Investment;
use crate::solana_sdk::instruction::Instruction;
use crate::solana_sdk::message::Message;
use crate::solana_sdk::signer::Signer;

pub struct SolanaChallengeProgramClient {
    config: Arc<InitServiceConfig>,
    rpc_client: Arc<RpcClient>,
    investment_client: Arc<SolanaInvestmentProgramClient>,
}

impl SolanaChallengeProgramClient {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(
        config: &Arc<InitServiceConfig>,
        rpc_client: Arc<RpcClient>,
        _anchor_client: Arc<Client>,
        investment_client: Arc<SolanaInvestmentProgramClient>,
    ) -> Self {
        SolanaChallengeProgramClient {
            config: config.clone(),
            rpc_client,
            investment_client,
        }
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_account(&self, state: Pubkey) -> AppResult<SolanaProgramAccount<Challenge>> {
        let rpc_client = self.rpc_client.clone();
        let account = tokio::task::spawn_blocking(move || rpc_client.get_account(&state))
            .await
            .unwrap()?;

        Ok(SolanaProgramAccount::new(account))
    }

    pub async fn get_many_accounts(
        &self,
        challenges: Vec<Pubkey>,
    ) -> AppResult<Vec<Option<(Pubkey, SolanaProgramAccount<Challenge>)>>> {
        let rpc_client = self.rpc_client.clone();
        let (challenges, results) = tokio::task::spawn_blocking(move || {
            rpc_client
                .get_multiple_accounts(&challenges)
                .map(|v| (challenges, v))
        })
        .await
        .unwrap()?;

        let mapped_results = challenges
            .into_iter()
            .zip(results.into_iter())
            .map(|(key, v)| {
                v.map(|account| {
                    if account.owner == gmi_challenge::ID {
                        Some((key, SolanaProgramAccount::new(account)))
                    } else {
                        None
                    }
                })
                .flatten()
            })
            .collect();

        Ok(mapped_results)
    }

    pub async fn call_check_bet(
        &self,
        challenge_account: &Pubkey,
        challenge_info: &Challenge,
        bet_account: &Pubkey,
        bet_info: &Bet,
    ) -> AppResult<String> {
        let rpc_client = self.rpc_client.clone();

        let wallet_config = self.config.wallet.as_ref().unwrap();
        let wallet_keypair = wallet_config.keypair();
        let wallet_pubkey = wallet_keypair.pubkey();

        let challenge_pda_account = Pubkey::create_program_address(
            &[
                gmi_challenge::constants::CHALLENGE_PDA_SEED,
                &challenge_info.creator_account.to_bytes(),
                &challenge_info.token_accumulator_account.to_bytes(),
                &[challenge_info.bump_seed],
            ],
            &gmi_challenge::ID,
        )
        .unwrap();
        let bet_pda_account = Pubkey::create_program_address(
            &[
                gmi_bet::constants::BET_PDA_SEED,
                &bet_info.owner_account.to_bytes(),
                &bet_info.fungible_token_account.to_bytes(),
                &[bet_info.bump_seed],
            ],
            &gmi_bet::ID,
        )
        .unwrap();

        // Prepare transaction.
        let instructions = vec![Instruction {
            program_id: gmi_challenge::ID,
            accounts: gmi_challenge::accounts::CheckBetInstruction {
                challenge_account: *challenge_account,
                creator_fee_account: challenge_info.creator_fee_account,
                token_accumulator_account: challenge_info.token_accumulator_account,
                bet_account: *bet_account,
                bet_fungible_token_account: bet_info.fungible_token_account,
                challenge_pda_account,
                bet_pda_account,
                bet_program: gmi_bet::ID,
                token_program: anchor_spl::token::ID,
            }
            .to_account_metas(None),
            data: gmi_challenge::instruction::CheckBet {}.data(),
        }];

        // Create transaction.
        let message = Message::new(&instructions, Some(&wallet_pubkey));
        let mut transaction = Transaction::new_unsigned(message);

        // Get blockhash and fees.
        let (recent_blockhash, _) =
            tokio::task::spawn_blocking(move || rpc_client.get_recent_blockhash())
                .await
                .unwrap()?;

        // Sign transaction.
        {
            let signers: &[&dyn Signer; 1] = &[&wallet_keypair];
            transaction
                .try_sign(signers, recent_blockhash)
                .map_err(AppError::from)?;
        }

        let rpc_client = self.rpc_client.clone();
        let result = tokio::task::spawn_blocking(move || {
            rpc_client
                .send_transaction(&transaction)
                .map(|v| v.to_string())
        })
        .await
        .unwrap()?;

        Ok(result)
    }

    pub async fn call_validate_bet(
        &self,
        challenge_account: &Pubkey,
        challenge_info: &Challenge,
        bet_account: &Pubkey,
        bet_info: &Bet,
        won: bool,
    ) -> AppResult<String> {
        let rpc_client = self.rpc_client.clone();

        let wallet_config = self.config.wallet.as_ref().unwrap();
        let wallet_keypair = wallet_config.keypair();
        let wallet_pubkey = wallet_keypair.pubkey();

        let challenge_pda_account = Pubkey::create_program_address(
            &[
                gmi_challenge::constants::CHALLENGE_PDA_SEED,
                &challenge_info.creator_account.to_bytes(),
                &challenge_info.token_accumulator_account.to_bytes(),
                &[challenge_info.bump_seed],
            ],
            &gmi_challenge::ID,
        )
        .unwrap();
        let bet_pda_account = Pubkey::create_program_address(
            &[
                gmi_bet::constants::BET_PDA_SEED,
                &bet_info.owner_account.to_bytes(),
                &bet_info.fungible_token_account.to_bytes(),
                &[bet_info.bump_seed],
            ],
            &gmi_bet::ID,
        )
        .unwrap();

        // Prepare transaction.
        let instructions = vec![Instruction {
            program_id: gmi_challenge::ID,
            accounts: gmi_challenge::accounts::ValidateBetInstruction {
                challenge_account: *challenge_account,
                creator_fee_account: challenge_info.creator_fee_account,
                validator_account: challenge_info.validator_account,
                token_accumulator_account: challenge_info.token_accumulator_account,
                bet_account: *bet_account,
                bet_fungible_token_account: bet_info.fungible_token_account,
                challenge_pda_account,
                bet_pda_account,
                bet_program: gmi_bet::ID,
                token_program: anchor_spl::token::ID,
            }
            .to_account_metas(None),
            data: gmi_challenge::instruction::ValidateBet { won }.data(),
        }];

        // Create transaction.
        let message = Message::new(&instructions, Some(&wallet_pubkey));
        let mut transaction = Transaction::new_unsigned(message);

        // Get blockhash and fees.
        let (recent_blockhash, _) =
            tokio::task::spawn_blocking(move || rpc_client.get_recent_blockhash())
                .await
                .unwrap()?;

        // Sign transaction.
        {
            let signers: &[&dyn Signer; 1] = &[&wallet_keypair];
            transaction
                .try_sign(signers, recent_blockhash)
                .map_err(AppError::from)?;
        }

        let rpc_client = self.rpc_client.clone();
        let result = tokio::task::spawn_blocking(move || {
            rpc_client
                .send_transaction(&transaction)
                .map(|v| v.to_string())
        })
        .await
        .unwrap()?;

        Ok(result)
    }

    pub async fn call_validate_bet_and_send_reward(
        &self,
        challenge_account: &Pubkey,
        challenge_info: &Challenge,
        bet_account: &Pubkey,
        bet_info: &Bet,
    ) -> AppResult<Option<(String, Pubkey)>> {
        // Get NFTs.
        let investments = self
            .investment_client
            .find_by_state_and_challenge(InvestmentState::Invested, challenge_account)
            .await?;
        let investment_objects: Vec<_> = investments
            .iter()
            .map(|(_, v)| v.load_data().unwrap())
            .collect();
        let nfts = self
            .investment_client
            .find_investments_nfts(&investment_objects)
            .await?;

        for (nfts, (investment_key, investment)) in nfts.iter().zip(investments) {
            for nft in nfts {
                let nft_key = Pubkey::from_str(&nft.pubkey).unwrap();

                // Try sending the transaction.
                if let Some(signature) = self
                    .call_validate_bet_and_redeem_nft(
                        challenge_account,
                        challenge_info,
                        bet_account,
                        bet_info,
                        &investment_key,
                        investment.load_data().unwrap(),
                        &nft_key,
                    )
                    .await?
                {
                    return Ok(Some((signature, nft_key)));
                }
            }
        }

        Ok(None)
    }

    #[allow(clippy::too_many_arguments)]
    async fn call_validate_bet_and_redeem_nft(
        &self,
        challenge_account: &Pubkey,
        challenge_info: &Challenge,
        bet_account: &Pubkey,
        bet_info: &Bet,
        investment_account: &Pubkey,
        investment_info: &Investment,
        nft_account: &Pubkey,
    ) -> AppResult<Option<String>> {
        let rpc_client = self.rpc_client.clone();

        let wallet_config = self.config.wallet.as_ref().unwrap();
        let wallet_keypair = wallet_config.keypair();
        let wallet_pubkey = wallet_keypair.pubkey();

        let challenge_pda_account = Pubkey::create_program_address(
            &[
                gmi_challenge::constants::CHALLENGE_PDA_SEED,
                &challenge_info.creator_account.to_bytes(),
                &challenge_info.token_accumulator_account.to_bytes(),
                &[challenge_info.bump_seed],
            ],
            &gmi_challenge::ID,
        )
        .unwrap();
        let bet_pda_account = Pubkey::create_program_address(
            &[
                gmi_bet::constants::BET_PDA_SEED,
                &bet_info.owner_account.to_bytes(),
                &bet_info.fungible_token_account.to_bytes(),
                &[bet_info.bump_seed],
            ],
            &gmi_bet::ID,
        )
        .unwrap();
        let investment_pda_account = Pubkey::create_program_address(
            &[
                gmi_investment::constants::INVESTMENT_PDA_SEED,
                &investment_info.investor_account.to_bytes(),
                &investment_info.fungible_token_account.to_bytes(),
                &[investment_info.bump_seed],
            ],
            &gmi_investment::ID,
        )
        .unwrap();

        // Prepare transaction.
        let instructions = vec![
            Instruction {
                program_id: gmi_challenge::ID,
                accounts: gmi_challenge::accounts::ValidateBetInstruction {
                    challenge_account: *challenge_account,
                    creator_fee_account: challenge_info.creator_fee_account,
                    validator_account: challenge_info.validator_account,
                    token_accumulator_account: challenge_info.token_accumulator_account,
                    bet_account: *bet_account,
                    bet_fungible_token_account: bet_info.fungible_token_account,
                    challenge_pda_account,
                    bet_pda_account,
                    bet_program: gmi_bet::ID,
                    token_program: anchor_spl::token::ID,
                }
                .to_account_metas(None),
                data: gmi_challenge::instruction::ValidateBet { won: true }.data(),
            },
            Instruction {
                program_id: gmi_challenge::ID,
                accounts: gmi_challenge::accounts::RedeemNFTInstruction {
                    challenge_account: *challenge_account,
                    creator_account: challenge_info.creator_account,
                    token_accumulator_account: challenge_info.token_accumulator_account,
                    bet_account: *bet_account,
                    bet_fungible_token_account: bet_info.fungible_token_account,
                    investment_account: *investment_account,
                    nft_account: *nft_account,
                    nft_destination_account: bet_info.owner_account,
                    challenge_pda_account,
                    bet_pda_account,
                    investment_pda_account,
                    bet_program: gmi_bet::ID,
                    investment_program: gmi_investment::ID,
                    token_program: anchor_spl::token::ID,
                }
                .to_account_metas(None),
                data: gmi_challenge::instruction::RedeemNft {}.data(),
            },
        ];

        // Create transaction.
        let message = Message::new(&instructions, Some(&wallet_pubkey));
        let mut transaction = Transaction::new_unsigned(message);

        // Get blockhash and fees.
        let (recent_blockhash, _) =
            tokio::task::spawn_blocking(move || rpc_client.get_recent_blockhash())
                .await
                .unwrap()?;

        // Sign transaction.
        {
            let signers: &[&dyn Signer; 1] = &[&wallet_keypair];
            transaction
                .try_sign(signers, recent_blockhash)
                .map_err(AppError::from)?;
        }

        let rpc_client = self.rpc_client.clone();
        let result = tokio::task::spawn_blocking(move || {
            rpc_client
                .send_transaction(&transaction)
                .map(|v| v.to_string())
        })
        .await
        .unwrap();

        match result {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}
