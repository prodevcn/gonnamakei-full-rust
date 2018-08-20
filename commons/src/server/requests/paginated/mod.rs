use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

pub use request::*;

use crate::database::types::UserType;
use crate::database::DBDocument;

mod request;

pub trait PaginatedDocument: Sized + Clone + Serialize + for<'de> Deserialize<'de> {
    type Field: PaginatedDocumentField;
    type DBDocument: DBDocument + From<Self>;

    // METHODS ----------------------------------------------------------------

    fn map_values_to_null(&mut self);

    fn into_db_document(self) -> Self::DBDocument {
        self.into()
    }
}

pub trait PaginatedDocumentField: Sized + Serialize + for<'de> Deserialize<'de> {
    type Document: PaginatedDocument;

    // METHODS ----------------------------------------------------------------

    fn is_valid_for_sorting(&self, user_type: UserType) -> bool;
    fn is_valid_for_filtering(&self, user_type: UserType) -> bool;
    fn path_to_value(&self) -> Cow<'static, str>;
    fn min_rows_per_page() -> u64;
    fn max_rows_per_page() -> u64;
}
