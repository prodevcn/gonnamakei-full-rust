use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::clients::games::ClashRoyaleBattlelogPlayerResponse;
use crate::clients::games::ClashRoyaleBattlelogResponse;
use crate::clients::games::ClashRoyaleCardResponse;
use crate::data::games::ClashRoyaleCard;
use crate::database::types::conditions::OrderedCondition;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleCardConditions {
    pub conditions: Vec<ClashRoyaleCardCondition>,
}

impl ClashRoyaleCardConditions {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        None
    }

    pub fn verify_allowed_response(
        &self,
        _battlelog: &ClashRoyaleBattlelogResponse,
        player: &ClashRoyaleBattlelogPlayerResponse,
    ) -> bool {
        let cards = match &player.cards {
            Some(v) => v,
            None => return false,
        };

        for card in cards {
            if !self
                .conditions
                .iter()
                .filter_map(|v| v.verify_response(card))
                .any(|v| v)
            {
                return false;
            }
        }

        true
    }

    pub fn verify_forbidden_response(
        &self,
        _battlelog: &ClashRoyaleBattlelogResponse,
        player: &ClashRoyaleBattlelogPlayerResponse,
    ) -> bool {
        let cards = match &player.cards {
            Some(v) => v,
            None => return false,
        };

        for card in cards {
            if self
                .conditions
                .iter()
                .filter_map(|v| v.verify_response(card))
                .any(|v| v)
            {
                return false;
            }
        }

        true
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// The conditions are applied to a single card (if `card_id` is present) or any of them (if not).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleCardCondition {
    /// The exact card.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<ClashRoyaleCard>,

    /// The king tower hit points of the team member.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<OrderedCondition<u16>>,

    /// The number of cards the user has.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<OrderedCondition<u16>>,
}

impl ClashRoyaleCardCondition {
    // METHODS ----------------------------------------------------------------

    pub fn verify_response(&self, card: &ClashRoyaleCardResponse) -> Option<bool> {
        if let Some(card_id) = self.card_id {
            // Ignore the check if not the same card.
            if card_id as u64 != card.id {
                return None;
            }
        }

        if let Some(level) = &self.level {
            if let Some(card_level) = card.level {
                if !level.verify(&card_level) {
                    return Some(false);
                }
            }
        }

        if let Some(count) = &self.count {
            if let Some(card_count) = card.count {
                if !count.verify(&card_count) {
                    return Some(false);
                }
            }
        }

        Some(true)
    }
}
