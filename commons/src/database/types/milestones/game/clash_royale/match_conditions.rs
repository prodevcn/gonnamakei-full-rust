use std::cmp::Ordering;
use std::convert::TryFrom;

use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::clients::games::ClashRoyaleBattlelogResponse;
use crate::data::games::ClashRoyaleArena;
use crate::database::types::conditions::{OptionCondition, OrderedCondition};
use crate::database::types::game::{
    ClashRoyaleGameMode, ClashRoyaleGameResult, ClashRoyaleTeamConditions,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleMatchConditions {
    /// Whether player must win or not the match.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<OptionCondition<ClashRoyaleGameResult>>,

    /// Whether friendly matches are accepted or not.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_friends: Option<bool>,

    /// The arena(s) (dis)allowed of the match.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arena: Option<OrderedCondition<ClashRoyaleArena>>,

    /// The game mode(s) (dis)allowed of the match.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<OptionCondition<ClashRoyaleGameMode>>,

    /// The conditions for the team.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<ClashRoyaleTeamConditions>,

    /// The conditions for the opponent.
    /// Missing: ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opponent: Option<ClashRoyaleTeamConditions>,
}

impl ClashRoyaleMatchConditions {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        if let Some(team) = &self.team {
            if let Some(error) = team.validate() {
                return Some(error);
            }
        }

        if let Some(opponent) = &self.opponent {
            if let Some(error) = opponent.validate() {
                return Some(error);
            }
        }

        None
    }

    pub fn verify_response(&self, battlelog: &ClashRoyaleBattlelogResponse) -> bool {
        let battlelog_team = match &battlelog.team {
            Some(v) => v,
            None => return false,
        };
        let battlelog_opponent = match &battlelog.opponent {
            Some(v) => v,
            None => return false,
        };

        let team_crows = battlelog_team[0].crowns;
        let opponent_crows = battlelog_opponent[0].crowns;
        let game_mode = if battlelog_team.len() == 1 {
            ClashRoyaleGameMode::OneVsOne
        } else {
            ClashRoyaleGameMode::TwoVsTwo
        };

        if let Some(result) = &self.result {
            let actual_result = match team_crows.cmp(&opponent_crows) {
                Ordering::Less => ClashRoyaleGameResult::Loose,
                Ordering::Equal => ClashRoyaleGameResult::Even,
                Ordering::Greater => ClashRoyaleGameResult::Win,
            };

            if !result.verify(&actual_result) {
                return false;
            }
        }

        if let Some(allow_friends) = self.allow_friends {
            let kind = match &battlelog.kind {
                Some(v) => v,
                None => return false,
            };

            if !allow_friends && kind.contains("friendly") {
                return false;
            }
        }

        if let Some(arena) = &self.arena {
            let battlelog_arena = match &battlelog.arena {
                Some(v) => v,
                None => return false,
            };

            let arena_id = match ClashRoyaleArena::try_from(battlelog_arena.id) {
                Ok(v) => v,
                Err(_) => return false,
            };

            if !arena.verify(&arena_id) {
                return false;
            }
        }

        if let Some(game_mode_option) = &self.game_mode {
            if !game_mode_option.verify(&game_mode) {
                return false;
            }
        }

        if let Some(team) = &self.team {
            if !team.verify_response(battlelog, battlelog_team.as_slice()) {
                return false;
            }
        }

        if let Some(opponent) = &self.opponent {
            if !opponent.verify_response(battlelog, battlelog_opponent.as_slice()) {
                return false;
            }
        }

        true
    }
}
