use std::ops::Deref;
use std::sync::{Arc, Mutex};

use arangors::index::{Index, IndexSettings};

use crate::constants::{DATABASE_MUTEX_INDEX, DATABASE_TTL_INDEX};
use crate::database::collections::CollectionKind;
use crate::database::documents::{ChallengeDBDocument, ChallengeDBDocumentField, DBDocumentField};
use crate::database::types::{DBMutexField, DBUuid};
use crate::database::{DBCollection, DBInfo};
use crate::error::{AppError, AppResult, INPUT_VALIDATION_UNDEFINED_CHALLENGE_ERROR_CODE};

lazy_static! {
    static ref COLLECTION: Mutex<Option<Arc<ChallengeCollection>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct ChallengeCollection {
    db_info: Arc<DBInfo>,
}

impl ChallengeCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<ChallengeCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(ChallengeCollection {
                    db_info: db_info.clone(),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<ChallengeCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_by_key_or_reject(
        challenge_key: &DBUuid,
        fields: Option<&ChallengeDBDocument>,
    ) -> AppResult<ChallengeDBDocument> {
        let challenge = ChallengeCollection::instance()
            .get_one_by_key(challenge_key, fields)
            .await?;

        Self::resolve_challenge_or_reject(challenge)
    }

    fn resolve_challenge_or_reject(
        challenge: Option<ChallengeDBDocument>,
    ) -> AppResult<ChallengeDBDocument> {
        match challenge {
            Some(v) => Ok(v),
            None => Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_CHALLENGE_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined challenge"))),
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;

        // Initialize collection.
        let collection_name = CollectionKind::Challenges.name();
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
                    name: DATABASE_TTL_INDEX.into(),
                    fields: vec![ChallengeDBDocumentField::DbExpiresAt(None)
                        .path()
                        .to_string()],
                    settings: IndexSettings::Ttl { expire_after: 0 },
                    ..Index::default()
                },
            )
            .await?;

        Ok(())
    }
}

impl DBCollection for ChallengeCollection {
    type Document = ChallengeDBDocument;

    fn name() -> &'static str {
        CollectionKind::Challenges.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
