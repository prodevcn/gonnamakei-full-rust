use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

pub use games::*;

use crate::data::SynchronizedDBDocument;
use crate::database::{DBDocument, NullableOption};
use crate::database::APIDocument;
use crate::database::AqlBuilder;
use crate::database::collections::{CollectionKind, MutexCollection, ParticipantCollection};
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::{DBId, DBMutex, DBUuid};

mod games;

model!(
    #![sync_level = "document"]
    #![build_api]

    pub struct Participant {
        /// The information of the user in the Clash Royale game.
        #[db_name = "G"]
        #[inner_model = "struct"]
        #[api_inner_type = "APIParticipantGamesData"]
        pub games_data: NullableOption<ParticipantGamesData>,
    }
);
