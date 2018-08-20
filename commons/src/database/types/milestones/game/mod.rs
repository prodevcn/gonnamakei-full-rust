use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

pub use clash_royale::*;

use crate::database::traits::{DBNormalize, DBNormalizeResult};

mod clash_royale;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "game", content = "challenge")]
pub enum GameMilestone {
    ClashRoyale(ClashRoyaleMilestone),
}

impl GameMilestone {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        match self {
            GameMilestone::ClashRoyale(v) => v.validate(),
        }
    }
}

impl DBNormalize for GameMilestone {
    fn normalize(&mut self) -> DBNormalizeResult {
        match self {
            GameMilestone::ClashRoyale(v) => v.normalize(),
        }
    }
}
