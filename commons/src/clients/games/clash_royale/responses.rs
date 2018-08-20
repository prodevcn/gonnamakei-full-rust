use arcstr::ArcStr;
use chrono::{TimeZone, Utc};
use serde::Serialize;
use serde::{de, Deserialize, Deserializer};

use crate::database::types::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyalePlayerInfoResponse {
    #[serde(default)]
    pub tag: Option<ArcStr>,

    #[serde(default)]
    pub name: Option<ArcStr>,

    #[serde(default)]
    pub exp_level: Option<u16>,

    #[serde(default)]
    pub trophies: Option<u64>,

    #[serde(default)]
    pub best_trophies: Option<u64>,

    #[serde(default)]
    pub wins: Option<u64>,

    #[serde(default)]
    pub losses: Option<u64>,

    #[serde(default)]
    pub battle_count: Option<u64>,

    #[serde(default)]
    pub three_crown_wins: Option<u64>,

    #[serde(default)]
    pub challenge_cards_won: Option<u64>,

    #[serde(default)]
    pub challenge_max_wins: Option<u64>,

    #[serde(default)]
    pub tournament_cards_won: Option<u64>,

    #[serde(default)]
    pub tournament_battle_count: Option<u64>,

    #[serde(default)]
    pub donations: Option<u64>,

    #[serde(default)]
    pub donations_received: Option<u64>,

    #[serde(default)]
    pub total_donations: Option<u64>,

    #[serde(default)]
    pub war_day_wins: Option<u64>,

    #[serde(default)]
    pub clan_cards_collected: Option<u64>,

    #[serde(default)]
    pub arena: Option<ClashRoyaleArenaResponse>,

    #[serde(default)]
    pub cards: Option<Vec<ClashRoyaleCardResponse>>,

    #[serde(default)]
    pub current_deck: Option<Vec<ClashRoyaleCardResponse>>,

    #[serde(default)]
    pub current_favourite_card: Option<ClashRoyaleCardResponse>,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleArenaResponse {
    pub id: u64, // ClashRoyaleArena
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleCardResponse {
    pub id: u64,
    // ClashRoyaleCard
    pub level: Option<u16>,
    pub count: Option<u16>,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleBattlelogResponse {
    #[serde(default)]
    #[serde(rename = "type")]
    pub kind: Option<String>,

    #[serde(deserialize_with = "deserialize_clash_royale_battle_time")]
    pub battle_time: DateTime,

    #[serde(default)]
    pub arena: Option<ClashRoyaleArenaResponse>,

    #[serde(default)]
    pub team: Option<Vec<ClashRoyaleBattlelogPlayerResponse>>,

    #[serde(default)]
    pub opponent: Option<Vec<ClashRoyaleBattlelogPlayerResponse>>,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRoyaleBattlelogPlayerResponse {
    pub tag: Option<ArcStr>,

    #[serde(default)]
    pub starting_trophies: Option<u64>,

    #[serde(default)]
    pub crowns: Option<u8>,

    #[serde(default)]
    pub cards: Option<Vec<ClashRoyaleCardResponse>>,
}

// ----------------------------------------------------------------------------
// Auxiliary methods ----------------------------------------------------------
// ----------------------------------------------------------------------------

fn deserialize_clash_royale_battle_time<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateTimeVisitor;

    impl<'de> de::Visitor<'de> for DateTimeVisitor {
        type Value = DateTime;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing json data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // 20211014T123745.000Z
            if v.len() != 20 {
                return Err(E::custom("incorrect string"));
            }

            let year = (&v[0..4]).parse::<i32>().map_err(E::custom)?;
            let month = (&v[4..6]).parse::<u32>().map_err(E::custom)?;
            let day = (&v[6..8]).parse::<u32>().map_err(E::custom)?;

            let hour = (&v[9..11]).parse::<u32>().map_err(E::custom)?;
            let minute = (&v[11..13]).parse::<u32>().map_err(E::custom)?;
            let second = (&v[13..15]).parse::<u32>().map_err(E::custom)?;
            let millis = (&v[16..19]).parse::<u32>().map_err(E::custom)?;

            Ok(DateTime::new(
                Utc.ymd(year, month, day)
                    .and_hms_milli(hour, minute, second, millis),
            ))
        }
    }

    deserializer.deserialize_string(DateTimeVisitor)
}
