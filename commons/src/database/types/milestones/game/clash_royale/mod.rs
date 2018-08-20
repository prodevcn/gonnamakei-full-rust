use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

pub use card_conditions::*;
pub use game_mode::*;
pub use game_result::*;
pub use match_conditions::*;
pub use member_conditions::*;
pub use team_conditions::*;
pub use user_conditions::*;

use crate::database::traits::{DBNormalize, DBNormalizeResult};

mod card_conditions;
mod game_mode;
mod game_result;
mod match_conditions;
mod member_conditions;
mod team_conditions;
mod user_conditions;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
#[serde(rename_all = "camelCase")]
pub enum ClashRoyaleMilestone {
    WinMatches(Vec<ClashRoyaleMatchConditions>),
    Achievement(Box<ClashRoyaleUserConditions>),
}

impl ClashRoyaleMilestone {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        match self {
            ClashRoyaleMilestone::WinMatches(conditions) => {
                if conditions.is_empty() {
                    return Some(arcstr::literal!(
                        "Clash Royale 'win matches' milestone without conditions"
                    ));
                }

                for condition in conditions {
                    if let Some(error) = condition.validate() {
                        return Some(error);
                    }
                }

                None
            }
            ClashRoyaleMilestone::Achievement(condition) => {
                if let Some(error) = condition.validate() {
                    return Some(error);
                }

                None
            }
        }
    }
}

impl DBNormalize for ClashRoyaleMilestone {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}
