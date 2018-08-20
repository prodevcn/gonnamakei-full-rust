use arcstr::ArcStr;
use serde::{Deserialize, Serialize};

use commons::data::games::ClashRoyaleArena;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleArenaGameDataResponse {
    id: u64,
    order: u64,
    title: ArcStr,
    subtitle: ArcStr,
    icon_url: ArcStr,
}

impl ClashRoyaleArenaGameDataResponse {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(arena: ClashRoyaleArena) -> Self {
        ClashRoyaleArenaGameDataResponse {
            id: arena as u64,
            order: arena.order(),
            title: arena.title().into(),
            subtitle: arena.subtitle().into(),
            icon_url: arena.icon_url().into(),
        }
    }
}
