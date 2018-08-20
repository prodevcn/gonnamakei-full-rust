use std::borrow::Cow;

use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;

use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::NullableOption;

sub_model!(
    #![build_api]

    pub struct ParticipantClashRoyaleGameData {
        #[db_name = "T"]
        pub tag: NullableOption<ArcStr>,
    }
);
