use chrono::{TimeZone, Utc};
use gmi_bet::state::{Bet, BetState};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::database::types::{Address, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializableBet {
    pub owner_account: Address,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receiver_account: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receiver_authority_account: Option<Address>,
    pub state: SerializedBetState,
    pub bump_seed: u8,
    pub amount: u64,
    pub won_amount: u64,
    pub applied_at: DateTime,
    pub expires_at: DateTime,
    pub fungible_token_account: Address,
}

impl From<&Bet> for SerializableBet {
    fn from(bet: &Bet) -> Self {
        Self {
            owner_account: Address::from(bet.owner_account),
            receiver_account: if bet.receiver_account == Pubkey::default() {
                None
            } else {
                Some(Address::from(bet.receiver_account))
            },
            receiver_authority_account: if bet.receiver_authority_account == Pubkey::default() {
                None
            } else {
                Some(Address::from(bet.receiver_authority_account))
            },
            state: bet.state.into(),
            bump_seed: bet.bump_seed,
            amount: bet.amount,
            won_amount: bet.won_amount,
            applied_at: DateTime::new(Utc.timestamp(bet.applied_at, 0)),
            expires_at: DateTime::new(Utc.timestamp(bet.expires_at, 0)),
            fungible_token_account: Address::from(bet.fungible_token_account),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SerializedBetState {
    Uninitialized,
    Initiated,
    Applied,
    Won,
}

impl From<BetState> for SerializedBetState {
    fn from(state: BetState) -> Self {
        match state {
            BetState::Uninitialized => SerializedBetState::Uninitialized,
            BetState::Initiated => SerializedBetState::Initiated,
            BetState::Applied => SerializedBetState::Applied,
            BetState::Won => SerializedBetState::Won,
        }
    }
}
