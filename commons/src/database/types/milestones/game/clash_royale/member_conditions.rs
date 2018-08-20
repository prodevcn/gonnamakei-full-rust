use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::clients::games::ClashRoyaleBattlelogPlayerResponse;
use crate::clients::games::ClashRoyaleBattlelogResponse;
use crate::database::types::conditions::OrderedCondition;
use crate::database::types::game::ClashRoyaleCardConditions;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleTeamMemberConditions {
    /// The number of starting trophies of the team member.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_trophies: Option<OrderedCondition<u64>>,

    // /// Note: not always available.
    // /// The hits made to the opponent's king tower.
    // /// Missing: ignored.
    // pub king_tower_hit_points: Option<OrderedCondition<u8>>,

    // /// Note: not always available.
    // /// The hits made to the opponent's princess tower 1.
    // /// Missing: ignored.
    // pub princess_tower_1_hit_points: Option<OrderedCondition<u8>>,

    // /// Note: not always available.
    // /// The hits made to the opponent's princess tower 2.
    // /// Missing: ignored.
    // pub princess_tower_2_hit_points: Option<OrderedCondition<u8>>,
    /// The allowed cards for this member.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cards: Option<ClashRoyaleCardConditions>,

    /// The forbidden cards for this member.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_cards: Option<ClashRoyaleCardConditions>,
}

impl ClashRoyaleTeamMemberConditions {
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

    pub fn verify_response(
        &self,
        battlelog: &ClashRoyaleBattlelogResponse,
        player: &ClashRoyaleBattlelogPlayerResponse,
    ) -> bool {
        if let Some(starting_trophies) = &self.starting_trophies {
            let actual_trophies = match player.starting_trophies {
                Some(v) => v,
                None => return false,
            };

            if !starting_trophies.verify(&actual_trophies) {
                return false;
            }
        }

        if let Some(allowed_cards) = &self.allowed_cards {
            if !allowed_cards.verify_allowed_response(battlelog, player) {
                return false;
            }
        }

        if let Some(forbidden_cards) = &self.forbidden_cards {
            if !forbidden_cards.verify_forbidden_response(battlelog, player) {
                return false;
            }
        }

        true
    }
}
