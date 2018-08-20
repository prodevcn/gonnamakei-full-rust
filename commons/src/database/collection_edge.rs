use async_trait::async_trait;

use crate::database::documents::DBDocumentField;
use crate::database::types::DBId;
use crate::database::{AqlBuilder, AqlReturn, DBCollection, AQL_DOCUMENT_ID};
use crate::error::AppResult;

#[async_trait]
pub trait DBEdgeCollection: DBCollection {
    // GETTERS ----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------

    /// Gets a document using the from field.
    async fn get_one_by_from(
        &self,
        key: &DBId,
        return_fields: Option<&Self::Document>,
    ) -> AppResult<Option<Self::Document>> {
        let result = self
            .get_one_by(&DBDocumentField::From.path(), &key, return_fields)
            .await?;
        Ok(result)
    }

    /// Gets a document using the to field.
    async fn get_one_by_to(
        &self,
        key: &DBId,
        return_fields: Option<&Self::Document>,
    ) -> AppResult<Option<Self::Document>> {
        let result = self
            .get_one_by(&DBDocumentField::To.path(), &key, return_fields)
            .await?;
        Ok(result)
    }

    /// Gets a document using the from and to fields.
    async fn get_one_by_from_and_to(
        &self,
        from: &DBId,
        to: &DBId,
        return_fields: Option<&Self::Document>,
    ) -> AppResult<Option<Self::Document>> {
        // Prepare AQL.
        // FOR i IN <collection>
        //      FILTER i._from == <from> && i._to == <to>
        //      RETURN i
        let mut aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, Self::name());

        aql.filter_step(
            format!(
                "{}.{} == {} && {}.{} == {}",
                AQL_DOCUMENT_ID,
                DBDocumentField::From.path(),
                serde_json::to_string(from).unwrap(),
                AQL_DOCUMENT_ID,
                DBDocumentField::To.path(),
                serde_json::to_string(to).unwrap(),
            )
            .into(),
        );

        if let Some(fields) = return_fields {
            aql.return_step_with_fields(AQL_DOCUMENT_ID, fields);
        } else {
            aql.return_step(AqlReturn::new_document());
        }

        let mut aql_result = self.send_aql(&aql).await?;

        Ok(aql_result.results.pop())
    }
}
