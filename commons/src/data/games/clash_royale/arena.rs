//! Info: https://github.com/RoyaleAPI/cr-api-data/tree/master/docs/json
//! Info: https://github.com/RoyaleAPI/cr-api-assets

use std::cmp::Ordering;
use std::convert::TryFrom;

use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

use enum_derive::EnumList;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize_repr, Deserialize_repr, EnumList)]
#[repr(u64)]
pub enum ClashRoyaleArena {
    Arena1 = 54000001,
    Arena2 = 54000002,
    Arena3 = 54000003,
    Arena4 = 54000004,
    Arena5 = 54000005,
    Arena6 = 54000006,
    Arena7 = 54000008,
    Arena8 = 54000009,
    Arena9 = 54000010,
    Arena10 = 54000007,
    Arena11 = 54000024,
    Arena12 = 54000011,
    Arena13 = 54000055,
    Arena14 = 54000056,
    LegendaryArena = 54000057,
    ChallengerII = 54000013,
    ChallengerIII = 54000014,
    MasterI = 54000015,
    MasterII = 54000016,
    MasterIII = 54000017,
    Champion = 54000018,
    GrandChampion = 54000019,
    RoyalChampion = 54000020,
    UltimateChampion = 54000031,
}

impl ClashRoyaleArena {
    // GETTERS ----------------------------------------------------------------

    pub fn order(&self) -> u64 {
        match self {
            ClashRoyaleArena::Arena1 => 1,
            ClashRoyaleArena::Arena2 => 2,
            ClashRoyaleArena::Arena3 => 3,
            ClashRoyaleArena::Arena4 => 4,
            ClashRoyaleArena::Arena5 => 5,
            ClashRoyaleArena::Arena6 => 6,
            ClashRoyaleArena::Arena7 => 7,
            ClashRoyaleArena::Arena8 => 8,
            ClashRoyaleArena::Arena9 => 9,
            ClashRoyaleArena::Arena10 => 10,
            ClashRoyaleArena::Arena11 => 11,
            ClashRoyaleArena::Arena12 => 12,
            ClashRoyaleArena::Arena13 => 13,
            ClashRoyaleArena::Arena14 => 14,
            ClashRoyaleArena::LegendaryArena => 15,
            ClashRoyaleArena::ChallengerII => 16,
            ClashRoyaleArena::ChallengerIII => 17,
            ClashRoyaleArena::MasterI => 18,
            ClashRoyaleArena::MasterII => 19,
            ClashRoyaleArena::MasterIII => 20,
            ClashRoyaleArena::Champion => 21,
            ClashRoyaleArena::GrandChampion => 22,
            ClashRoyaleArena::RoyalChampion => 23,
            ClashRoyaleArena::UltimateChampion => 24,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            ClashRoyaleArena::Arena1 => "Arena 1",
            ClashRoyaleArena::Arena2 => "Arena 2",
            ClashRoyaleArena::Arena3 => "Arena 3",
            ClashRoyaleArena::Arena4 => "Arena 4",
            ClashRoyaleArena::Arena5 => "Arena 5",
            ClashRoyaleArena::Arena6 => "Arena 6",
            ClashRoyaleArena::Arena7 => "Arena 7",
            ClashRoyaleArena::Arena8 => "Arena 8",
            ClashRoyaleArena::Arena9 => "Arena 9",
            ClashRoyaleArena::Arena10 => "Arena 10",
            ClashRoyaleArena::Arena11 => "Arena 11",
            ClashRoyaleArena::Arena12 => "Arena 12",
            ClashRoyaleArena::Arena13 => "Arena 13",
            ClashRoyaleArena::Arena14 => "Arena 14",
            ClashRoyaleArena::LegendaryArena => "Legendary Arena",
            ClashRoyaleArena::ChallengerII => "Challenger II",
            ClashRoyaleArena::ChallengerIII => "Challenger III",
            ClashRoyaleArena::MasterI => "Master I",
            ClashRoyaleArena::MasterII => "Master II",
            ClashRoyaleArena::MasterIII => "Master III",
            ClashRoyaleArena::Champion => "Champion",
            ClashRoyaleArena::GrandChampion => "Grand Champion",
            ClashRoyaleArena::RoyalChampion => "Royal Champion",
            ClashRoyaleArena::UltimateChampion => "Ultimate Champion",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            ClashRoyaleArena::Arena1 => "Goblin Stadium",
            ClashRoyaleArena::Arena2 => "Bone Pit",
            ClashRoyaleArena::Arena3 => "Barbarian Bowl",
            ClashRoyaleArena::Arena4 => "P.E.K.K.A's Playhouse",
            ClashRoyaleArena::Arena5 => "Spell Valley",
            ClashRoyaleArena::Arena6 => "Builder's Workshop",
            ClashRoyaleArena::Arena7 => "Royal Arena",
            ClashRoyaleArena::Arena8 => "Frozen Peak",
            ClashRoyaleArena::Arena9 => "Jungle Arena",
            ClashRoyaleArena::Arena10 => "Hog Mountain",
            ClashRoyaleArena::Arena11 => "Electro Valley",
            ClashRoyaleArena::Arena12 => "Spooky Town",
            ClashRoyaleArena::Arena13 => "Rascal's Hideout",
            ClashRoyaleArena::Arena14 => "Serenity Peak",
            ClashRoyaleArena::LegendaryArena => "Legendary Arena",
            ClashRoyaleArena::ChallengerII => "Challenger II",
            ClashRoyaleArena::ChallengerIII => "Challenger III",
            ClashRoyaleArena::MasterI => "Master I",
            ClashRoyaleArena::MasterII => "Master II",
            ClashRoyaleArena::MasterIII => "Master III",
            ClashRoyaleArena::Champion => "Champion",
            ClashRoyaleArena::GrandChampion => "Grand Champion",
            ClashRoyaleArena::RoyalChampion => "Royal Champion",
            ClashRoyaleArena::UltimateChampion => "Ultimate Champion",
        }
    }

    pub fn icon_url(&self) -> &'static str {
        match self {
            ClashRoyaleArena::Arena1 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena1.png",
            ClashRoyaleArena::Arena2 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena2.png",
            ClashRoyaleArena::Arena3 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena3.png",
            ClashRoyaleArena::Arena4 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena4.png",
            ClashRoyaleArena::Arena5 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena5.png",
            ClashRoyaleArena::Arena6 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena6.png",
            ClashRoyaleArena::Arena7 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena7.png",
            ClashRoyaleArena::Arena8 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena8.png",
            ClashRoyaleArena::Arena9 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena9.png",
            ClashRoyaleArena::Arena10 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena10.png",
            ClashRoyaleArena::Arena11 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena11.png",
            ClashRoyaleArena::Arena12 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena12.png",
            ClashRoyaleArena::Arena13 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena13.png",
            ClashRoyaleArena::Arena14 => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena14.png",
            ClashRoyaleArena::LegendaryArena => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena15.png",
            ClashRoyaleArena::ChallengerII => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena16.png",
            ClashRoyaleArena::ChallengerIII => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena17.png",
            ClashRoyaleArena::MasterI => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena18.png",
            ClashRoyaleArena::MasterII => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena19.png",
            ClashRoyaleArena::MasterIII => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena20.png",
            ClashRoyaleArena::Champion => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena21.png",
            ClashRoyaleArena::GrandChampion => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena22.png",
            ClashRoyaleArena::RoyalChampion => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena23.png",
            ClashRoyaleArena::UltimateChampion => "https://raw.githubusercontent.com/RoyaleAPI/cr-api-assets/master/arenas/arena24.png",
        }
    }
}

impl PartialOrd for ClashRoyaleArena {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClashRoyaleArena {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order().cmp(&other.order())
    }
}

impl TryFrom<u64> for ClashRoyaleArena {
    type Error = ();

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            54000001 => Ok(ClashRoyaleArena::Arena1),
            54000002 => Ok(ClashRoyaleArena::Arena2),
            54000003 => Ok(ClashRoyaleArena::Arena3),
            54000004 => Ok(ClashRoyaleArena::Arena4),
            54000005 => Ok(ClashRoyaleArena::Arena5),
            54000006 => Ok(ClashRoyaleArena::Arena6),
            54000008 => Ok(ClashRoyaleArena::Arena7),
            54000009 => Ok(ClashRoyaleArena::Arena8),
            54000010 => Ok(ClashRoyaleArena::Arena9),
            54000007 => Ok(ClashRoyaleArena::Arena10),
            54000024 => Ok(ClashRoyaleArena::Arena11),
            54000011 => Ok(ClashRoyaleArena::Arena12),
            54000055 => Ok(ClashRoyaleArena::Arena13),
            54000056 => Ok(ClashRoyaleArena::Arena14),
            54000057 => Ok(ClashRoyaleArena::LegendaryArena),
            54000013 => Ok(ClashRoyaleArena::ChallengerII),
            54000014 => Ok(ClashRoyaleArena::ChallengerIII),
            54000015 => Ok(ClashRoyaleArena::MasterI),
            54000016 => Ok(ClashRoyaleArena::MasterII),
            54000017 => Ok(ClashRoyaleArena::MasterIII),
            54000018 => Ok(ClashRoyaleArena::Champion),
            54000019 => Ok(ClashRoyaleArena::GrandChampion),
            54000020 => Ok(ClashRoyaleArena::RoyalChampion),
            54000031 => Ok(ClashRoyaleArena::UltimateChampion),
            _ => Err(()),
        }
    }
}
