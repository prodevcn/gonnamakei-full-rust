use serde::{Deserialize, Serialize};

use crate::database::types::Address;
use crate::programs::gmi_challenge::state::{Challenge, ChallengeState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializableChallenge {
    pub creator_account: Address,
    pub validator_account: Address,
    pub state: SerializedChallengeState,
    pub bump_seed: u8,
    pub url: String,
    pub authorized_investments: bool,
    pub authorized_bets: bool,
    pub allow_redeem_many_nfts: bool,
    pub bets_expiration_delay: i64,
    pub min_bet_amount: u64,
    pub max_bet_amount: u64,
    pub reward_times: u64,
    pub wins: u64,
    pub losses: u64,
    pub expirations: u64,
    pub total_nfts: u64,
    pub max_nfts: u64,
    pub investments: u64,
    pub max_investments: u64,
    pub min_investment_amount: u64,
    pub max_investment_amount: u64,
    pub total_invested: u64,
    pub max_fungible_tokens: u64,
    pub parallel_bets: u64,
    pub max_parallel_bets: u64,
    pub history_bets: u64,
    pub max_history_bets: u64,
    pub token_accumulator_account: Address,
    pub creator_fee_account: Address,
    pub bet_fee: u64,
    pub bet_fee_percentage: u8,
    pub investment_fee: u64,
    pub investment_fee_percentage: u8,
    pub withdraw_investment_fee: u64,
    pub withdraw_investment_fee_percentage: u8,
}

impl From<&Challenge> for SerializableChallenge {
    fn from(challenge: &Challenge) -> Self {
        Self {
            creator_account: Address::from(challenge.creator_account),
            validator_account: Address::from(challenge.validator_account),
            state: challenge.state.into(),
            bump_seed: challenge.bump_seed,
            url: challenge.url_as_str().to_string(),
            authorized_investments: challenge.authorized_investments,
            authorized_bets: challenge.authorized_bets,
            allow_redeem_many_nfts: challenge.allow_redeem_many_nfts,
            bets_expiration_delay: challenge.bets_expiration_delay,
            min_bet_amount: challenge.min_bet_amount,
            max_bet_amount: challenge.max_bet_amount,
            reward_times: challenge.reward_times,
            wins: challenge.wins,
            losses: challenge.losses,
            expirations: challenge.expirations,
            total_nfts: challenge.total_nfts,
            max_nfts: challenge.max_nfts,
            investments: challenge.investments,
            max_investments: challenge.max_investments,
            min_investment_amount: challenge.min_investment_amount,
            max_investment_amount: challenge.max_investment_amount,
            total_invested: challenge.total_invested,
            max_fungible_tokens: challenge.max_fungible_tokens,
            parallel_bets: challenge.parallel_bets,
            max_parallel_bets: challenge.max_parallel_bets,
            history_bets: challenge.history_bets,
            max_history_bets: challenge.max_history_bets,
            token_accumulator_account: Address::from(challenge.token_accumulator_account),
            creator_fee_account: Address::from(challenge.creator_fee_account),
            bet_fee: challenge.bet_fee,
            bet_fee_percentage: challenge.bet_fee_percentage,
            investment_fee: challenge.investment_fee,
            investment_fee_percentage: challenge.investment_fee_percentage,
            withdraw_investment_fee: challenge.withdraw_investment_fee,
            withdraw_investment_fee_percentage: challenge.withdraw_investment_fee_percentage,
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SerializedChallengeState {
    Uninitialized,
    Initiated,
    Active,
}

impl From<ChallengeState> for SerializedChallengeState {
    fn from(state: ChallengeState) -> Self {
        match state {
            ChallengeState::Uninitialized => SerializedChallengeState::Uninitialized,
            ChallengeState::Initiated => SerializedChallengeState::Initiated,
            ChallengeState::Active => SerializedChallengeState::Active,
        }
    }
}
