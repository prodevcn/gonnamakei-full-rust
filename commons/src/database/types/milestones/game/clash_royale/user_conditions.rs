use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::data::games::ClashRoyaleArena;
use crate::database::types::conditions::OrderedCondition;
use crate::database::types::game::ClashRoyaleCardConditions;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleUserConditions {
    /// The level of the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_level: Option<OrderedCondition<u8>>,

    /// The number of trophies of the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trophies: Option<OrderedCondition<u64>>,

    /// The number of best trophies of the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_trophies: Option<OrderedCondition<u64>>,

    /// The number of wins of the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wins: Option<OrderedCondition<u64>>,

    /// The number of 3-crowns wins of the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_crowns_wins: Option<OrderedCondition<u64>>,

    /// The number of losses of the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub losses: Option<OrderedCondition<u64>>,

    /// The number of battles the user has played.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battle_count: Option<OrderedCondition<u64>>,

    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_cards_won: Option<OrderedCondition<u64>>,

    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_max_wins: Option<OrderedCondition<u64>>,

    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournament_cards_won: Option<OrderedCondition<u64>>,

    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournament_battle_count: Option<OrderedCondition<u64>>,

    /// The number of donations the user has done.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donations: Option<OrderedCondition<u64>>,

    /// The number of donations the user has receive.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donations_received: Option<OrderedCondition<u64>>,

    /// The number of total donations related to the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_donations: Option<OrderedCondition<u64>>,

    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub war_day_wins: Option<OrderedCondition<u64>>,

    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_cards_collected: Option<OrderedCondition<u64>>,

    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_arena: Option<OrderedCondition<ClashRoyaleArena>>,

    /// The allowed cards for the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cards: Option<ClashRoyaleCardConditions>,

    /// The forbidden cards for the user.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_cards: Option<ClashRoyaleCardConditions>,
}

impl ClashRoyaleUserConditions {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        if let Some(allowed_cards) = &self.allowed_cards {
            if let Some(error) = allowed_cards.validate() {
                return Some(error);
            }
        }

        if let Some(forbidden_cards) = &self.forbidden_cards {
            if let Some(error) = forbidden_cards.validate() {
                return Some(error);
            }
        }

        None
    }
}
