use std::borrow::Cow;
use std::sync::Arc;

use arcstr::ArcStr;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::data::SynchronizedDBDocument;
use crate::database::collections::BetCollection;
use crate::database::collections::{CollectionKind, MutexCollection};
use crate::database::documents::ChallengeAPIDocument;
use crate::database::documents::ChallengeDBDocument;
use crate::database::documents::ParticipantAPIDocument;
use crate::database::documents::ParticipantDBDocument;
use crate::database::traits::AQLMapping;
use crate::database::traits::DBNormalize;
use crate::database::traits::DBNormalizeResult;
use crate::database::types::Address;
use crate::database::types::{DBExpiration, DateTime};
use crate::database::types::{DBId, DBMutex, DBUuid};
use crate::database::APIDocument;
use crate::database::APIReference;
use crate::database::AqlBuilder;
use crate::database::DBReference;
use crate::database::{DBDocument, NullableOption};

model!(
    #![sync_level = "document"]
    #![build_api]

    pub struct Bet {
        /// The state of the bet.
        #[db_name = "S"]
        #[api_inner_type = "BetDBState"]
        pub state: NullableOption<BetDBState>,

        /// The participant that has created the bet.
        /// Note: includes `games_data` field.
        #[db_name = "P"]
        #[api_inner_type = "ParticipantAPIDocument"]
        pub participant: NullableOption<DBReference<ParticipantDBDocument>>,

        /// The challenge the bet has been applied to.
        #[db_name = "C"]
        #[api_inner_type = "ChallengeAPIDocument"]
        pub challenge: NullableOption<DBReference<ChallengeDBDocument>>,

        /// The transaction to create this bet.
        /// Note: only present when not yet created.
        #[db_name = "Tx"]
        #[api_sensible_info]
        pub transaction: NullableOption<ArcStr>,

        /// The keypair of the bet.
        #[db_name = "K"]
        #[api_sensible_info]
        pub keypair: NullableOption<ArcStr>,

        /// The keypair of the bet's fungible token account.
        #[db_name = "FK"]
        #[api_sensible_info]
        pub fungible_token_account_keypair: NullableOption<ArcStr>,

        /// The won NFT.
        #[db_name = "N"]
        pub won_nft: NullableOption<Address>,

        /// The time this bet expires.
        #[db_name = "T"]
        pub created_at: NullableOption<DateTime>,

        /// The time this bet expires.
        #[db_name = "X"]
        #[api_sensible_info]
        pub db_expires_at: NullableOption<DBExpiration>,
    }
);

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

sub_model!(
    #![build_api]

    pub enum BetDBState {
        /// The bet is waiting to be created in the blockchain.
        #[db_name = "N"]
        WaitingForCreating,

        /// The bet has been created in the blockchain.
        #[db_name = "C"]
        Created,

        /// The bet was won.
        #[db_name = "W"]
        Won,

        /// The bet was lost.
        #[db_name = "L"]
        Lost,

        /// The bet is expired but it is not in reflected in the blockchain.
        #[db_name = "X"]
        ExpiredNotInBlockchain,

        /// The bet is expired and it is reflected in the blockchain.
        #[db_name = "E"]
        Expired,
    }
);
