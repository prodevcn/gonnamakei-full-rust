use serde::Deserialize;
use serde::Serialize;

use crate::database::traits::{DBNormalize, DBNormalizeResult};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClashRoyaleGameMode {
    OneVsOne,
    TwoVsTwo,
}

impl DBNormalize for ClashRoyaleGameMode {
    fn normalize(&mut self) -> DBNormalizeResult {
        DBNormalizeResult::NotModified
    }
}
