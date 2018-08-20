use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use commons::data::games::ClashRoyaleCard;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleCardGameDataResponse {
    id: u64,
    name: ArcStr,
    icon_url: ArcStr,
    max_level: u64,
}

impl ClashRoyaleCardGameDataResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(card: ClashRoyaleCard) -> Self {
        ClashRoyaleCardGameDataResponse {
            id: card as u64,
            name: card.name().into(),
            icon_url: card.icon_url().into(),
            max_level: card.max_level(),
        }
    }
}
