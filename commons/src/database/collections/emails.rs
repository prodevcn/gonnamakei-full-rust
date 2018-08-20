use std::ops::Deref;
use std::sync::{Arc, Mutex};

use arangors::index::{Index, IndexSettings};

use crate::constants::EMAIL_INDEX;
use crate::database::collections::CollectionKind;
use crate::database::documents::{EmailDBDocument, EmailDBDocumentField};
use crate::database::types::DBUuid;
use crate::database::{DBCollection, DBDocument, DBInfo};
use crate::error::{AppError, AppResult, INPUT_VALIDATION_UNDEFINED_EMAIL_ERROR_CODE};

lazy_static! {
    static ref COLLECTION: Mutex<Option<Arc<EmailCollection>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct EmailCollection {
    db_info: Arc<DBInfo>,
}

impl EmailCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<EmailCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(EmailCollection {
                    db_info: db_info.clone(),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<EmailCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_by_key_or_reject(
        email_key: &DBUuid,
        fields: Option<&EmailDBDocument>,
    ) -> AppResult<EmailDBDocument> {
        let email = EmailCollection::instance()
            .get_one_by_key(email_key, fields)
            .await?;

        Self::resolve_email_or_reject(email)
    }

    pub async fn remove_and_get_by_key_or_reject(email_key: &DBUuid) -> AppResult<EmailDBDocument> {
        let document = EmailDBDocument {
            db_key: Some(email_key.clone()),
            ..Default::default()
        };

        document.remove(None).await
    }

    fn resolve_email_or_reject(email: Option<EmailDBDocument>) -> AppResult<EmailDBDocument> {
        match email {
            Some(v) => Ok(v),
            None => Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_EMAIL_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined email"))),
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;

        // Initialize collection.
        let collection_name = CollectionKind::Emails.name();
        let _ = database.create_collection(collection_name).await; // Ignore error because it means already created.

        // Add indexes.
        database
            .create_index(
                collection_name,
                &Index {
                    name: EMAIL_INDEX.into(),
                    fields: vec![EmailDBDocumentField::Email(None).path().to_string()],
                    settings: IndexSettings::Persistent {
                        unique: true,
                        sparse: false,
                        deduplicate: false,
                    },
                    ..Index::default()
                },
            )
            .await?;

        Ok(())
    }
}

impl DBCollection for EmailCollection {
    type Document = EmailDBDocument;

    fn name() -> &'static str {
        CollectionKind::Emails.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
