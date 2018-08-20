use std::convert::TryFrom;
use std::sync::Arc;

use tokio::task::JoinHandle;

pub use authentication::*;
pub use bet::*;
pub use challenge::*;
pub use config::*;
pub use emails::*;
use enum_derive::EnumList;
pub use mutex::*;
pub use participant::*;
pub use signature::*;

use crate::data::RemoteMutexGuard;
use crate::database::documents::ParticipantDBDocument;
use crate::database::documents::{BetDBDocument, ChallengeDBDocument, MutexDBDocument};
use crate::database::{DBCollection, DBInfo};
use crate::error::AppResult;

mod authentication;
mod bet;
pub(crate) mod cache;
mod challenge;
mod config;
mod emails;
mod mutex;
mod participant;
mod signature;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumList, Hash)]
pub enum CollectionKind {
    Configuration,
    Mutexes,
    Challenges,
    Participants,
    Bets,
    Signatures,
    Authentications,
    Emails,
}

impl CollectionKind {
    // GETTER -----------------------------------------------------------------

    pub fn name(&self) -> &'static str {
        match self {
            CollectionKind::Configuration => "Configuration",
            CollectionKind::Mutexes => "Mutexes",
            CollectionKind::Challenges => "Challenges",
            CollectionKind::Participants => "Participants",
            CollectionKind::Bets => "Bets",
            CollectionKind::Signatures => "Signatures",
            CollectionKind::Authentications => "Authentications",
            CollectionKind::Emails => "Emails",
        }
    }

    pub async fn drop_collection(&self) -> AppResult<()> {
        match self {
            CollectionKind::Configuration => {
                let instance = ConfigCollection::instance();
                instance.drop_collection().await?;
                instance.drop_analyzers().await
            }
            CollectionKind::Mutexes => MutexCollection::instance().drop_collection().await,
            CollectionKind::Challenges => ChallengeCollection::instance().drop_collection().await,
            CollectionKind::Participants => {
                ParticipantCollection::instance().drop_collection().await
            }
            CollectionKind::Bets => BetCollection::instance().drop_collection().await,
            CollectionKind::Signatures => SignatureCollection::instance().drop_collection().await,
            CollectionKind::Authentications => {
                AuthenticationCollection::instance().drop_collection().await
            }
            CollectionKind::Emails => EmailCollection::instance().drop_collection().await,
        }
    }

    // METHODS ----------------------------------------------------------------

    pub fn init(&self, db_info: &Arc<DBInfo>) {
        match self {
            CollectionKind::Configuration => {
                ConfigCollection::init(db_info);
            }
            CollectionKind::Mutexes => {
                MutexCollection::init(db_info);
            }
            CollectionKind::Challenges => {
                ChallengeCollection::init(db_info);
            }
            CollectionKind::Participants => {
                ParticipantCollection::init(db_info);
            }
            CollectionKind::Bets => {
                BetCollection::init(db_info);
            }
            CollectionKind::Signatures => {
                SignatureCollection::init(db_info);
            }
            CollectionKind::Authentications => {
                AuthenticationCollection::init(db_info);
            }
            CollectionKind::Emails => {
                EmailCollection::init(db_info);
            }
        }
    }

    pub async fn create_collection(&self, db_info: &Arc<DBInfo>) -> AppResult<()> {
        match self {
            CollectionKind::Configuration => ConfigCollection::create_collection(db_info).await,
            CollectionKind::Mutexes => MutexCollection::create_collection(db_info).await,
            CollectionKind::Challenges => ChallengeCollection::create_collection(db_info).await,
            CollectionKind::Participants => ParticipantCollection::create_collection(db_info).await,
            CollectionKind::Bets => BetCollection::create_collection(db_info).await,
            CollectionKind::Signatures => SignatureCollection::create_collection(db_info).await,
            CollectionKind::Authentications => {
                AuthenticationCollection::create_collection(db_info).await
            }
            CollectionKind::Emails => EmailCollection::create_collection(db_info).await,
        }
    }

    pub fn release_all_document_mutex(&self) -> Option<JoinHandle<()>> {
        match self {
            CollectionKind::Configuration => None,
            CollectionKind::Mutexes => Some(tokio::spawn(
                RemoteMutexGuard::<MutexDBDocument>::release_all_document_mutex(),
            )),
            CollectionKind::Challenges => Some(tokio::spawn(
                RemoteMutexGuard::<ChallengeDBDocument>::release_all_document_mutex(),
            )),
            CollectionKind::Participants => Some(tokio::spawn(RemoteMutexGuard::<
                ParticipantDBDocument,
            >::release_all_document_mutex(
            ))),
            CollectionKind::Bets => Some(tokio::spawn(
                RemoteMutexGuard::<BetDBDocument>::release_all_document_mutex(),
            )),
            CollectionKind::Signatures => None,
            CollectionKind::Authentications => None,
            CollectionKind::Emails => None,
        }
    }
}

impl TryFrom<&str> for CollectionKind {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Configuration" => Ok(CollectionKind::Configuration),
            "Mutexes" => Ok(CollectionKind::Mutexes),
            "Challenges" => Ok(CollectionKind::Challenges),
            "Participants" => Ok(CollectionKind::Participants),
            "Bets" => Ok(CollectionKind::Bets),
            "Signatures" => Ok(CollectionKind::Signatures),
            "Authentications" => Ok(CollectionKind::Authentications),
            "Emails" => Ok(CollectionKind::Emails),
            _ => Err(()),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn check_try_into() {
        for value in CollectionKind::enum_list() {
            let name = value.name();
            let value2: CollectionKind = name
                .try_into()
                .unwrap_or_else(|_| panic!("The value {:?} cannot be got from name", value));

            assert_eq!(*value, value2, "The value {:?} is incorrect", value);
        }
    }
}
