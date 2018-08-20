use std::ops::Deref;
use std::sync::{Arc, Mutex};

use arangors::index::{Index, IndexSettings};

use crate::constants::DATABASE_TTL_INDEX;
use crate::database::collections::CollectionKind;
use crate::database::documents::{SignatureDBDocument, SignatureDBDocumentField};
use crate::database::types::DBUuid;
use crate::database::{DBCollection, DBDocument, DBInfo};
use crate::error::{AppError, AppResult, INPUT_VALIDATION_UNDEFINED_SIGNATURE_ERROR_CODE};

lazy_static! {
    static ref COLLECTION: Mutex<Option<Arc<SignatureCollection>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct SignatureCollection {
    db_info: Arc<DBInfo>,
}

impl SignatureCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<SignatureCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(SignatureCollection {
                    db_info: db_info.clone(),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<SignatureCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_by_key_or_reject(
        signature_key: &DBUuid,
        fields: Option<&SignatureDBDocument>,
    ) -> AppResult<SignatureDBDocument> {
        let signature = SignatureCollection::instance()
            .get_one_by_key(signature_key, fields)
            .await?;

        Self::resolve_signature_or_reject(signature)
    }

    pub async fn remove_and_get_by_key_or_reject(
        signature_key: &DBUuid,
    ) -> AppResult<SignatureDBDocument> {
        let document = SignatureDBDocument {
            db_key: Some(signature_key.clone()),
            ..Default::default()
        };

        document.remove(None).await
    }

    fn resolve_signature_or_reject(
        signature: Option<SignatureDBDocument>,
    ) -> AppResult<SignatureDBDocument> {
        match signature {
            Some(v) => Ok(v),
            None => Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_SIGNATURE_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined signature"))),
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;

        // Initialize collection.
        let collection_name = CollectionKind::Signatures.name();
        let _ = database.create_collection(collection_name).await; // Ignore error because it means already created.

        // Add indexes.
        database
            .create_index(
                collection_name,
                &Index {
                    name: DATABASE_TTL_INDEX.into(),
                    fields: vec![SignatureDBDocumentField::DbExpiresAt(None)
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

impl DBCollection for SignatureCollection {
    type Document = SignatureDBDocument;

    fn name() -> &'static str {
        CollectionKind::Signatures.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
