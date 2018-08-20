use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

pub use milestone::*;

use crate::clients::solana::models::SerializableChallenge;
use crate::constants::MAX_CHALLENGE_RESPONSES_PER_PAGE;
use crate::constants::MIN_CHALLENGE_RESPONSES_PER_PAGE;
use crate::data::SynchronizedDBDocument;
use crate::database::collections::{ChallengeCollection, CollectionKind, MutexCollection};
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::DBExpiration;
use crate::database::types::UserType;
use crate::database::types::{DBId, DBMutex, DBUuid, DateTime};
use crate::database::APIDocument;
use crate::database::AqlBuilder;
use crate::database::{DBDocument, NullableOption};
use crate::server::requests::PaginatedDocument;
use crate::server::requests::PaginatedDocumentField;

mod milestone;

model!(
    #![sync_level = "document"]
    #![build_api]
    #![api_paginated]
    #![api_min_rows_per_page = "MIN_CHALLENGE_RESPONSES_PER_PAGE"]
    #![api_max_rows_per_page = "MAX_CHALLENGE_RESPONSES_PER_PAGE"]

    pub struct Challenge {
        /// The name of the challenge.
        #[db_name = "N"]
        pub name: NullableOption<ArcStr>,

        /// The description of the challenge.
        #[db_name = "D"]
        pub description: NullableOption<ArcStr>,

        /// The public key of the challenge. It can be
        #[db_name = "P"]
        #[api_sensible_info]
        pub public_key: NullableOption<ArcStr>,

        /// The keypair of the challenge.
        #[db_name = "K"]
        #[api_sensible_info]
        pub keypair: NullableOption<ArcStr>,

        /// The milestones of the challenge the user must achieve in order to complete it.
        #[db_name = "M"]
        #[inner_model = "struct"]
        #[api_inner_type = "APIChallengeMilestone"]
        pub milestones: NullableOption<Vec<ChallengeMilestone>>,

        /// The transaction to create this challenge.
        /// Note: only present when not yet created.
        #[db_name = "Tx"]
        #[api_sensible_info]
        pub transaction: NullableOption<Vec<u8>>,

        /// The time this challenge was created.
        #[db_name = "C"]
        pub created_at: NullableOption<DateTime>,

        /// The blockchain info of the challenge.
        /// This field won't be stored in the DB.
        #[skip_normalize]
        pub blockchain_info: NullableOption<SerializableChallenge>,

        /// The image url of the main NFT.
        #[db_name = "I"]
        pub nft_image_url: NullableOption<ArcStr>,

        /// The time this challenge expires.
        #[db_name = "X"]
        #[api_sensible_info]
        pub db_expires_at: NullableOption<DBExpiration>,
    }

    // For API ----------------------------------------------------------------

    fn is_valid_for_sorting(&self, _user_type: UserType) -> bool {
        true
    }

    fn is_valid_for_filtering(&self, _user_type: UserType) -> bool {
        true
    }
);
