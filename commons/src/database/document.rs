use std::sync::Arc;

use arangors::document::options::{InsertOptions, OverwriteMode, RemoveOptions, UpdateOptions};
use arangors::document::response::DocumentResponse;
use arangors::ClientError;
use arcstr::ArcStr;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::constants::MAX_INSERT_RETRIES;
use crate::database::collection::DBCollection;
use crate::database::traits::{AQLMapping, DBNormalize, DBNormalizeResult};
use crate::database::types::{DBId, DBUuid, DBUuidType, DateTime};
use crate::database::DBInfo;
use crate::error::AppResult;

#[async_trait]
pub trait DBDocument:
    Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> + AQLMapping
{
    type Collection: DBCollection;

    // GETTERS ----------------------------------------------------------------

    fn db_id(&self) -> Option<DBId>;

    fn db_key(&self) -> &Option<DBUuid>;

    fn db_rev(&self) -> &Option<ArcStr>;

    fn created_at(&self) -> Option<DateTime> {
        self.db_key().as_ref().and_then(|v| v.date())
    }

    fn collection() -> Arc<Self::Collection>;

    /// Whether all the fields are missing or not.
    fn is_all_missing(&self) -> bool;

    // SETTERS ----------------------------------------------------------------

    fn set_db_key(&mut self, value: Option<DBUuid>);

    // METHODS ----------------------------------------------------------------

    /// Maps all fields that contain a value into a null.
    fn map_values_to_null(&mut self);

    /// Normalizes the fields of the document to clean it up.
    fn normalize_fields(&mut self) -> DBNormalizeResult;

    /// Filters the current document using the specified filter.
    fn filter(&mut self, filter: &Self);

    /// Inserts a new document.
    ///
    /// WARN: returns the whole document.
    async fn insert(mut self, overwrite: bool) -> AppResult<Self> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        if self.db_key().is_none() {
            self.set_db_key(Some(DBUuid::new(DBUuidType::DBKey)));
        }

        loop {
            let response = db_collection
                .create_document(
                    self.clone(),
                    InsertOptions::builder()
                        .return_new(true)
                        .return_old(false)
                        .keep_null(false)
                        .overwrite(overwrite)
                        .overwrite_mode(OverwriteMode::Replace)
                        .build(),
                )
                .await;

            match response {
                Ok(v) => match v {
                    DocumentResponse::Silent => unreachable!("Not silent insert!"),
                    DocumentResponse::Response { new, .. } => return Ok(new.unwrap()),
                },
                Err(e) => {
                    if !overwrite {
                        if let ClientError::Arango(e) = &e {
                            if e.error_num() == 1210 && e.message().contains("_key") {
                                self.set_db_key(Some(DBUuid::new(DBUuidType::DBKey)));
                                continue;
                            }
                        }
                    }

                    DBInfo::check_client_error_to_retry(e)?;
                }
            }
        }
    }

    /// Inserts a new document ignoring the result.
    async fn insert_and_ignore(mut self, overwrite: bool) -> AppResult<DBUuid> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        if self.db_key().is_none() {
            self.set_db_key(Some(DBUuid::new(DBUuidType::DBKey)));
        }

        let mut retries = 0;
        loop {
            let response = db_collection
                .create_document(
                    self.clone(),
                    InsertOptions::builder()
                        .return_new(false)
                        .return_old(false)
                        .keep_null(false)
                        .overwrite(overwrite)
                        .overwrite_mode(OverwriteMode::Replace)
                        .build(),
                )
                .await;

            match response {
                Ok(_) => return Ok(self.db_key().clone().unwrap()),
                Err(error) => {
                    if !overwrite {
                        if let ClientError::Arango(e) = &error {
                            if e.error_num() == 1210 && e.message().contains("_key") {
                                self.set_db_key(Some(DBUuid::new(DBUuidType::DBKey)));

                                retries += 1;

                                if retries >= MAX_INSERT_RETRIES {
                                    return Err(error.into());
                                }

                                continue;
                            }
                        }
                    }

                    DBInfo::check_client_error_to_retry(error)?;
                }
            }
        }
    }

    /// Updates the element and returns its updated value.
    ///
    /// WARN: returns the whole document.
    async fn update(&self, merge_objects: bool) -> AppResult<Self> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        let ignore_rev = self.db_rev().is_none();

        let key = self
            .db_key()
            .as_ref()
            .unwrap_or_else(|| {
                panic!(
                    "You forgot to include the key property in the {} document",
                    Self::Collection::name()
                )
            })
            .as_string()
            .as_str();

        loop {
            let response = db_collection
                .update_document(
                    key,
                    self.clone(),
                    UpdateOptions::builder()
                        .merge_objects(merge_objects)
                        .keep_null(false)
                        .return_new(true)
                        .ignore_revs(ignore_rev)
                        .build(),
                )
                .await;

            match response {
                Ok(v) => match v {
                    DocumentResponse::Silent => unreachable!("This update is not silent"),
                    DocumentResponse::Response { new, .. } => return Ok(new.unwrap()),
                },
                Err(e) => DBInfo::check_client_error_to_retry(e)?,
            }
        }
    }

    /// Updates the element ignoring the result.
    async fn update_and_ignore(&self, merge_objects: bool) -> AppResult<()> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        let ignore_rev = self.db_rev().is_none();

        let key = self
            .db_key()
            .as_ref()
            .unwrap_or_else(|| {
                panic!(
                    "You forgot to include the key property in the {} document",
                    Self::Collection::name()
                )
            })
            .as_string()
            .as_str();

        loop {
            let response = db_collection
                .update_document(
                    key,
                    self.clone(),
                    UpdateOptions::builder()
                        .merge_objects(merge_objects)
                        .keep_null(false)
                        .ignore_revs(ignore_rev)
                        .silent(true)
                        .build(),
                )
                .await;

            match response {
                Ok(_) => return Ok(()),
                Err(e) => DBInfo::check_client_error_to_retry(e)?,
            }
        }
    }

    /// Inserts a new document or updates it if it already exists.
    ///
    /// WARN: returns the whole document.
    async fn insert_or_update(mut self, merge_objects: bool) -> AppResult<Self> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        if self.db_key().is_none() {
            self.set_db_key(Some(DBUuid::new(DBUuidType::DBKey)));
        }

        loop {
            let response = db_collection
                .create_document(
                    self.clone(),
                    InsertOptions::builder()
                        .return_new(true)
                        .return_old(false)
                        .overwrite(true)
                        .overwrite_mode(OverwriteMode::Update)
                        .keep_null(false)
                        .merge_objects(merge_objects)
                        .build(),
                )
                .await;

            match response {
                Ok(v) => match v {
                    DocumentResponse::Silent => unreachable!("Not silent insert!"),
                    DocumentResponse::Response { new, .. } => return Ok(new.unwrap()),
                },
                Err(e) => {
                    DBInfo::check_client_error_to_retry(e)?;
                }
            }
        }
    }

    /// Inserts a new document or updates it if it already exists, ignoring the result.
    async fn insert_or_update_and_ignore(mut self, merge_objects: bool) -> AppResult<DBUuid> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        if self.db_key().is_none() {
            self.set_db_key(Some(DBUuid::new(DBUuidType::DBKey)));
        }

        loop {
            let response = db_collection
                .create_document(
                    self.clone(),
                    InsertOptions::builder()
                        .return_new(false)
                        .return_old(false)
                        .overwrite(true)
                        .overwrite_mode(OverwriteMode::Update)
                        .keep_null(false)
                        .merge_objects(merge_objects)
                        .silent(true)
                        .build(),
                )
                .await;

            match response {
                Ok(_) => return Ok(self.db_key().clone().unwrap()),
                Err(error) => {
                    DBInfo::check_client_error_to_retry(error)?;
                }
            }
        }
    }

    /// Removes the element returning the old value.
    async fn remove(&self, rev: Option<ArcStr>) -> AppResult<Self> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        let key = self
            .db_key()
            .as_ref()
            .unwrap_or_else(|| {
                panic!(
                    "You forgot to include the key property in the {} document",
                    Self::Collection::name()
                )
            })
            .as_string()
            .as_str();
        let rev = rev.map(|v| v.to_string());

        loop {
            let response = db_collection
                .remove_document(
                    key,
                    RemoveOptions::builder()
                        .return_old(true)
                        .silent(false)
                        .build(),
                    rev.clone(),
                )
                .await;

            match response {
                Ok(v) => match v {
                    DocumentResponse::Silent => unreachable!("This remove is not silent"),
                    DocumentResponse::Response { old, .. } => return Ok(old.unwrap()),
                },
                Err(e) => DBInfo::check_client_error_to_retry(e)?,
            }
        }
    }

    /// Removes the element ignoring the result.
    async fn remove_and_ignore(&self, rev: Option<ArcStr>) -> AppResult<()> {
        let collection = Self::collection();
        let db_collection = collection.db_collection().await?;

        let key = self
            .db_key()
            .as_ref()
            .unwrap_or_else(|| {
                panic!(
                    "You forgot to include the key property in the {} document",
                    Self::Collection::name()
                )
            })
            .as_string()
            .as_str();
        let rev = rev.map(|v| v.to_string());

        loop {
            let response = db_collection
                .remove_document::<()>(
                    key,
                    RemoveOptions::builder()
                        .return_old(false)
                        .silent(true)
                        .build(),
                    rev.clone(),
                )
                .await;

            match response {
                Ok(_) => return Ok(()),
                Err(e) => DBInfo::check_client_error_to_retry(e)?,
            }
        }
    }
}

impl<T> DBNormalize for T
where
    T: DBDocument,
{
    fn normalize(&mut self) -> DBNormalizeResult {
        self.normalize_fields()
    }
}
