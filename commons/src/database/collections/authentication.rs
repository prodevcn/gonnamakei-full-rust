use std::ops::Deref;
use std::sync::{Arc, Mutex};

use arangors::index::{Index, IndexSettings};

use crate::constants::{AUTHORIZATION_ADDRESS_INDEX, DATABASE_TTL_INDEX};
use crate::database::collections::CollectionKind;
use crate::database::documents::{AuthenticationDBDocument, AuthenticationDBDocumentField};
use crate::database::types::{Address, DBUuid};
use crate::database::{AqlBuilder, AqlRemove, DBCollection, DBDocument, DBInfo, AQL_DOCUMENT_ID};
use crate::error::{AppError, AppResult, INPUT_VALIDATION_UNDEFINED_AUTHENTICATION_ERROR_CODE};

lazy_static! {
    static ref COLLECTION: Mutex<Option<Arc<AuthenticationCollection>>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct AuthenticationCollection {
    db_info: Arc<DBInfo>,
}

impl AuthenticationCollection {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn init(db_info: &Arc<DBInfo>) -> Arc<AuthenticationCollection> {
        let mut collection = COLLECTION.lock().unwrap();
        match collection.deref() {
            Some(v) => v.clone(),
            None => {
                let value = Arc::new(AuthenticationCollection {
                    db_info: db_info.clone(),
                });

                *collection = Some(value.clone());

                value
            }
        }
    }

    pub fn instance() -> Arc<AuthenticationCollection> {
        COLLECTION.lock().unwrap().as_ref().unwrap().clone()
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_by_key_or_reject(
        authentication_key: &DBUuid,
        fields: Option<&AuthenticationDBDocument>,
    ) -> AppResult<AuthenticationDBDocument> {
        let authentication = AuthenticationCollection::instance()
            .get_one_by_key(authentication_key, fields)
            .await?;

        Self::resolve_authentication_or_reject(authentication)
    }

    pub async fn remove_by_address_or_reject(address: &Address) -> AppResult<()> {
        let collection = Self::instance();

        // FOR i IN <collection>
        //     FILTER i.<address> == <address>
        //     REMOVE i
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, Self::name());

        aql.filter_step(
            format!(
                "{}.{} == {}",
                AQL_DOCUMENT_ID,
                AuthenticationDBDocumentField::Address(None).path(),
                serde_json::to_string(address).unwrap()
            )
            .into(),
        );

        aql.remove_step(AqlRemove::new_document(Self::name()));

        let _ = collection.send_aql(&aql).await?;
        Ok(())
    }

    fn resolve_authentication_or_reject(
        authentication: Option<AuthenticationDBDocument>,
    ) -> AppResult<AuthenticationDBDocument> {
        match authentication {
            Some(v) => Ok(v),
            None => Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_UNDEFINED_AUTHENTICATION_ERROR_CODE,
            )
            .message(arcstr::literal!("Undefined authentication"))),
        }
    }

    // STATIC METHODS ---------------------------------------------------------

    pub(crate) async fn create_collection(db_info: &Arc<DBInfo>) -> AppResult<()> {
        let database = &db_info.database;

        // Initialize collection.
        let collection_name = CollectionKind::Authentications.name();
        let _ = database.create_collection(collection_name).await; // Ignore error because it means already created.

        // Add indexes.
        database
            .create_index(
                collection_name,
                &Index {
                    name: DATABASE_TTL_INDEX.into(),
                    fields: vec![AuthenticationDBDocumentField::DbExpiresAt(None)
                        .path()
                        .to_string()],
                    settings: IndexSettings::Ttl { expire_after: 0 },
                    ..Index::default()
                },
            )
            .await?;
        database
            .create_index(
                collection_name,
                &Index {
                    name: AUTHORIZATION_ADDRESS_INDEX.into(),
                    fields: vec![AuthenticationDBDocumentField::Address(None)
                        .path()
                        .to_string()],
                    settings: IndexSettings::Persistent {
                        unique: true,
                        sparse: false,
                        deduplicate: false,
                    },
                    ..Index::default()
                },
            )
            .await?;

        // Add internal authentication tokens.
        let token = AuthenticationDBDocument {
            db_key: Some(
                db_info
                    .config
                    .backend
                    .as_ref()
                    .unwrap()
                    .handy_game_token
                    .clone(),
            ),
            ..Default::default()
        };
        token
            .insert_and_ignore(true)
            .await
            .expect("Cannot insert handy_game_token in DB");

        Ok(())
    }
}

impl DBCollection for AuthenticationCollection {
    type Document = AuthenticationDBDocument;

    fn name() -> &'static str {
        CollectionKind::Authentications.name()
    }

    fn db_info(&self) -> &Arc<DBInfo> {
        &self.db_info
    }
}
