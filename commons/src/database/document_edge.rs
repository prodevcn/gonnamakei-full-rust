use async_trait::async_trait;

use crate::database::types::DBId;
use crate::database::DBDocument;

#[async_trait]
pub trait DBEdgeDocument: DBDocument {
    // GETTERS ----------------------------------------------------------------

    fn db_from(&self) -> &Option<DBId>;

    fn db_to(&self) -> &Option<DBId>;

    // GETTERS ----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------
}
