use std::borrow::Cow;

use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::database::documents::{ChallengeAPIDocument, ChallengeDBDocument};
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::game::GameMilestone;
use crate::database::APIReference;
use crate::database::AqlBuilder;
use crate::database::DBReference;

sub_model!(
    #![build_api]

    pub enum ChallengeMilestone {
        /// A milestone related to a specific game.
        #[db_name = "G"]
        #[api_inner_type = "APIGameChallengeMilestone"]
        GameMilestone(GameChallengeMilestone),

        /// The milestone is to complete other challenge.
        #[db_name = "O"]
        #[api_inner_type = "APIOtherChallengeMilestone"]
        OtherChallenge(OtherChallengeMilestone),
    }
);

impl APIChallengeMilestone {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        match self {
            APIChallengeMilestone::GameMilestone(v) => v.validate(),
            APIChallengeMilestone::OtherChallenge(v) => v.validate(),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

sub_model!(
    #![build_api]

    pub struct GameChallengeMilestone {
        #[db_name = "M"]
        pub milestone: GameMilestone,
    }
);

impl APIGameChallengeMilestone {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        self.milestone.validate()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

sub_model!(
    #![build_api]

    pub struct OtherChallengeMilestone {
        #[db_name = "C"]
        #[api_inner_type = "ChallengeAPIDocument"]
        pub challenge: DBReference<ChallengeDBDocument>,
    }
);

impl APIOtherChallengeMilestone {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self) -> Option<ArcStr> {
        None
    }
}
