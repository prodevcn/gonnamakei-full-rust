use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

pub use clash_royale::*;

use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::AqlBuilder;
use crate::database::NullableOption;

mod clash_royale;

sub_model!(
    #![build_api]

    pub struct ParticipantGamesData {
        #[db_name = "CR"]
        #[inner_model = "struct"]
        #[api_inner_type = "APIParticipantClashRoyaleGameData"]
        pub clash_royale: NullableOption<ParticipantClashRoyaleGameData>,
    }
);
