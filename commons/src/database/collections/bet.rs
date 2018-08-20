use std::ops::Deref;
use std::sync::{Arc, Mutex};

use arangors::index::{Index, IndexSettings};

use crate::constants::{
    BET_CHALLENGE_INDEX, BET_PARTICIPANT_INDEX, DATABASE_MUTEX_INDEX, DATABASE_TTL_INDEX,
};
use crate::database::collections::CollectionKind;
use crate::database::documents::{BetDBDocument, BetDBDocumentField, DBDocumentField};
use crate::database::types::{DBMutexField, DBUuid};
use crate::database::{DBCollection, DBInfo};
use crate::error::{AppError, AppResult, INPUT_VALIDATION_UNDEFINED_BET_ERROR_CODE};

lazy_static! {
    static ref COLLECTION: Mutex<Option<Arc<BetCollection>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct BetCollection {
    db_info: Arc<DBInfo>,
}

impl BetCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<BetCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(BetCollection {
                    db_info: db_info.clone(),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<BetCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_by_key_or_reject(
        bet_key: &DBUuid,
        fields: Option<&BetDBDocument>,
    ) -> AppResult<BetDBDocument> {
        let bet = BetCollection::instance()
            .get_one_by_key(bet_key, fields)
            .await?;

        Self::resolve_bet_or_reject(bet)
    }

    fn resolve_bet_or_reject(bet: Option<BetDBDocument>) -> AppResult<BetDBDocument> {
        match bet {
            Some(v) => Ok(v),
            None => Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_BET_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined bet"))),
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;

        // Initialize collection.
        let collection_name = CollectionKind::Bets.name();
        let _ = database.create_collection(collection_name).await; // Ignore error because it means already created.

        // Add indexes.
        database
            .create_index(
                collection_name,
                &Index {
                    name: DATABASE_MUTEX_INDEX.into(),
                    fields: vec![format!(
                        "{}.{}",
                        DBDocumentField::Mutex.path(),
                        DBMutexField::Node(None).path()
                    )],
                    settings: IndexSettings::Persistent {
                        unique: false,
                        sparse: true,
                        deduplicate: false,
                    },
                    ..Index::default()
                },
            )
            .await?;
        database
            .create_index(
                collection_name,
                &Index {
                    name: BET_PARTICIPANT_INDEX.into(),
                    fields: vec![BetDBDocumentField::Participant(None).path().to_string()],
                    settings: IndexSettings::Persistent {
                        unique: false,
                        sparse: true,
                        deduplicate: false,
                    },
                    ..Index::default()
                },
            )
            .await?;
        database
            .create_index(
                collection_name,
                &Index {
                    name: BET_CHALLENGE_INDEX.into(),
                    fields: vec![BetDBDocumentField::Challenge(None).path().to_string()],
                    settings: IndexSettings::Persistent {
                        unique: false,
                        sparse: true,
                        deduplicate: false,
                    },
                    ..Index::default()
                },
            )
            .await?;
        database
            .create_index(
                collection_name,
                &Index {
                    name: DATABASE_TTL_INDEX.into(),
                    fields: vec![BetDBDocumentField::DbExpiresAt(None).path().to_string()],
                    settings: IndexSettings::Ttl { expire_after: 0 },
                    ..Index::default()
                },
            )
            .await?;

        Ok(())
    }
}

impl DBCollection for BetCollection {
    type Document = BetDBDocument;

    fn name() -> &'static str {
        CollectionKind::Bets.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
