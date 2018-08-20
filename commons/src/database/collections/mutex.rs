use std::ops::Deref;
use std::sync::Arc;

use arangors::index::{Index, IndexSettings};

use crate::constants::DATABASE_MUTEX_INDEX;
use crate::database::collection::DBCollection;
use crate::database::collections::CollectionKind;
use crate::database::documents::{DBDocumentField, MutexDBDocument};
use crate::database::types::{DBMutexField, DBUuid};
use crate::database::DBInfo;
use crate::error::{AppError, AppResult, INPUT_VALIDATION_UNDEFINED_MUTEX_ERROR_CODE};

lazy_static! {
    static ref COLLECTION: std::sync::Mutex<Option<Arc<MutexCollection>>> =
        std::sync::Mutex::new(None);
}

#[derive(Debug)]
pub struct MutexCollection {
    db_info: Arc<DBInfo>,
}

impl MutexCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<MutexCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(MutexCollection {
                    db_info: db_info.clone(),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<MutexCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // GETTERS ----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------

    pub async fn get_by_key_or_reject(
        mutex_key: &DBUuid,
        fields: Option<&MutexDBDocument>,
    ) -> AppResult<MutexDBDocument> {
        let mutex = MutexCollection::instance()
            .get_one_by_key(mutex_key, fields)
            .await?;

        Self::resolve_mutex_or_reject(mutex)
    }

    fn resolve_mutex_or_reject(message: Option<MutexDBDocument>) -> AppResult<MutexDBDocument> {
        match message {
            Some(v) => Ok(v),
            None => Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_MUTEX_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined mutex"))),
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;

        // Initialize collection.
        let _collection = MutexCollection::init(db_info);
        let collection_name = CollectionKind::Mutexes.name();
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

impl DBCollection for MutexCollection {
    type Document = MutexDBDocument;

    fn name() -> &'static str {
        CollectionKind::Mutexes.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
