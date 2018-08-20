use std::ops::Deref;
use std::sync::{Arc, Mutex};

use arangors::index::{Index, IndexSettings};

use crate::constants::DATABASE_MUTEX_INDEX;
use crate::database::collections::CollectionKind;
use crate::database::documents::{DBDocumentField, ParticipantDBDocument};
use crate::database::types::{DBMutexField, DBUuid};
use crate::database::{DBCollection, DBInfo};
use crate::error::{AppError, AppResult, INPUT_VALIDATION_UNDEFINED_PARTICIPANT_ERROR_CODE};

lazy_static! {
    static ref COLLECTION: Mutex<Option<Arc<ParticipantCollection>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct ParticipantCollection {
    db_info: Arc<DBInfo>,
}

impl ParticipantCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<ParticipantCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(ParticipantCollection {
                    db_info: db_info.clone(),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<ParticipantCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_by_key_or_reject(
        participant_key: &DBUuid,
        fields: Option<&ParticipantDBDocument>,
    ) -> AppResult<ParticipantDBDocument> {
        let participant = ParticipantCollection::instance()
            .get_one_by_key(participant_key, fields)
            .await?;

        Self::resolve_participant_or_reject(participant)
    }

    fn resolve_participant_or_reject(
        participant: Option<ParticipantDBDocument>,
    ) -> AppResult<ParticipantDBDocument> {
        match participant {
            Some(v) => Ok(v),
            None => Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_PARTICIPANT_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined participant"))),
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;

        // Initialize collection.
        let collection_name = CollectionKind::Participants.name();
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

        Ok(())
    }
}

impl DBCollection for ParticipantCollection {
    type Document = ParticipantDBDocument;

    fn name() -> &'static str {
        CollectionKind::Participants.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
