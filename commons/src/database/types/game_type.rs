use serde::Deserialize;
use serde::Serialize;

use crate::database::traits::{DBNormalize, DBNormalizeResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum GameType {
    ClashRoyale,
}

impl DBNormalize for GameType {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}
