use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::clients::games::ClashRoyaleBattlelogPlayerResponse;
use crate::clients::games::ClashRoyaleBattlelogResponse;
use crate::database::types::conditions::OrderedCondition;
use crate::database::types::game::{ClashRoyaleCardConditions, ClashRoyaleTeamMemberConditions};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleTeamConditions {
    /// The number of crowns the team must achieve.
    /// Max: 3
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crowns: Option<OrderedCondition<u8>>,

    /// The sum number of starting trophies of the whole team.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_trophies: Option<OrderedCondition<u64>>,

    // /// Note: not always available.
    // /// The total sum of the hits made to the opponent's king tower.
    // /// Missing: ignored.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub king_tower_hit_points: Option<OrderedCondition<u8>>,

    // /// Note: not always available.
    // /// The total sum of the hits made to the opponent's princess tower 1.
    // /// Missing: ignored.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub princess_tower_1_hit_points: Option<OrderedCondition<u8>>,

    // /// Note: not always available.
    // /// The total sum of the hits made to the opponent's princess tower 2.
    // /// Missing: ignored.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub princess_tower_2_hit_points: Option<OrderedCondition<u8>>,
    /// The allowed cards for the whole team.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_cards: Option<ClashRoyaleCardConditions>,

    /// The forbidden cards for the whole team.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_cards: Option<ClashRoyaleCardConditions>,

    /// The conditions of the first team member.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_member: Option<ClashRoyaleTeamMemberConditions>,

    /// The conditions of the second team member.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_member: Option<ClashRoyaleTeamMemberConditions>,
}

impl ClashRoyaleTeamConditions {
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

        if let Some(first_member) = &self.first_member {
            if let Some(error) = first_member.validate() {
                return Some(error);
            }
        }

        if let Some(second_member) = &self.second_member {
            if let Some(error) = second_member.validate() {
                return Some(error);
            }
        }

        None
    }

    pub fn verify_response(
        &self,
        battlelog: &ClashRoyaleBattlelogResponse,
        players: &[ClashRoyaleBattlelogPlayerResponse],
    ) -> bool {
        let team_starting_trophies = players.iter().filter_map(|v| v.starting_trophies).sum();

        if let Some(crowns) = &self.crowns {
            let team_crowns = match players[0].crowns {
                Some(v) => v,
                None => return false,
            };

            if !crowns.verify(&team_crowns) {
                return false;
            }
        }

        if let Some(starting_trophies) = &self.starting_trophies {
            if !starting_trophies.verify(&team_starting_trophies) {
                return false;
            }
        }

        if let Some(allowed_cards) = &self.allowed_cards {
            for player in players {
                if !allowed_cards.verify_allowed_response(battlelog, player) {
                    return false;
                }
            }
        }

        if let Some(forbidden_cards) = &self.forbidden_cards {
            for player in players {
                if !forbidden_cards.verify_forbidden_response(battlelog, player) {
                    return false;
                }
            }
        }

        if players.len() == 1 {
            if let Some(first_member) = &self.first_member {
                if !first_member.verify_response(battlelog, &players[0]) {
                    return false;
                }
            }
        } else {
            match (&self.first_member, &self.second_member) {
                (Some(first_member), Some(second_member)) => {
                    if !first_member.verify_response(battlelog, &players[0]) {
                        if !second_member.verify_response(battlelog, &players[0]) {
                            return false;
                        }

                        if !first_member.verify_response(battlelog, &players[1]) {
                            return false;
                        }
                    } else if !second_member.verify_response(battlelog, &players[1]) {
                        return false;
                    }
                }
                (None, None) => {}
                _ => return false,
            }
        }

        true
    }
}
