use serde::Deserialize;
use serde::Serialize;

use crate::database::documents::APIConfig;
use crate::database::documents::APIFilteringStatsConfig;
use crate::database::types::{APIFilter, UserType};
use crate::database::{AqlBuilder, AqlLimit, AqlReturn, AqlSort, AQL_DOCUMENT_ID};
use crate::error::{
    AppError, AppResult, INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
    INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE,
};
use crate::server::requests::{PaginatedDocument, PaginatedDocumentField};
use crate::server::validators::length_validator;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "F: PaginatedDocumentField")]
pub struct PaginatedRequest<F: PaginatedDocumentField> {
    #[serde(default)]
    pub sort_by: Vec<PaginatedSortByRequest<F>>,
    pub page: u64,
    pub rows_per_page: u64,
    #[serde(default)]
    pub filter_by: Option<APIFilter<F>>,
    #[serde(default)]
    pub fields_filter: Option<F::Document>,
    #[serde(default)]
    pub count_pages: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound = "F: PaginatedDocumentField")]
pub struct PaginatedSortByRequest<F: PaginatedDocumentField> {
    pub field: F,
    #[serde(default)]
    pub descending: bool,
}

impl<F: PaginatedDocumentField> PaginatedRequest<F> {
    // METHODS ----------------------------------------------------------------

    pub fn validate(&self, user_type: UserType) -> AppResult<()> {
        // Validate the sort_by.
        length_validator(self.sort_by.len(), 0, 3, "sortBy")?;

        for sort_by in &self.sort_by {
            if !sort_by.field.is_valid_for_sorting(user_type) {
                return Err(AppError::new_with_status(
                    warp::http::StatusCode::BAD_REQUEST,
                    INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
                )
                .message(
                    format!(
                        "The field '{}' is not valid for filtering",
                        serde_json::to_string(&sort_by.field).unwrap()
                    )
                    .into(),
                )
                .param(arcstr::literal!("sortBy")));
            }
        }

        // Validate the filter_by.
        if let Some(filter_by) = &self.filter_by {
            if let Err(e) = filter_by.validate(user_type) {
                return Err(AppError::new_with_status(
                    warp::http::StatusCode::BAD_REQUEST,
                    INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
                )
                .message(e.into())
                .param(arcstr::literal!("filterBy")));
            }
        }

        // Validate rows.
        let minimum_rows = F::min_rows_per_page();
        if self.rows_per_page < minimum_rows {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
            )
            .message(
                format!(
                    "The minimum rows per page is {}. Current: {}",
                    minimum_rows, self.rows_per_page
                )
                .into(),
            )
            .param(arcstr::literal!("rowsPerPage")));
        }

        let maximum_rows = F::max_rows_per_page();
        if self.rows_per_page > maximum_rows {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_INCORRECT_VALUE_ERROR_CODE,
            )
            .message(
                format!(
                    "The maximum rows per page is {}. Current: {}",
                    maximum_rows, self.rows_per_page
                )
                .into(),
            )
            .param(arcstr::literal!("rowsPerPage")));
        }

        Ok(())
    }

    pub fn normalize(&mut self, api_config: &APIConfig) -> AppResult<()> {
        if let Some(fields_filter) = &mut self.fields_filter {
            fields_filter.map_values_to_null();
        }

        if let Some(filter_by) = self.filter_by.take() {
            let filter_by = filter_by.normalize();
            let mut stats = APIFilteringStatsConfig::default();

            filter_by.calculate_stats(&mut stats);

            let config_stats = &api_config.filtering.admin;
            if stats.field_count > config_stats.field_count
                || stats.const_count > config_stats.const_count
                || stats.expression_count > config_stats.expression_count
                || stats.function_count > config_stats.function_count
            {
                return Err(AppError::new_with_status(
                    warp::http::StatusCode::BAD_REQUEST,
                    INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE,
                )
                .message(arcstr::literal!(
                    "The filterBy property contains too many elements to be executed"
                ))
                .param(arcstr::literal!("filterBy")));
            }

            self.filter_by = Some(filter_by.normalize());
        }

        Ok(())
    }

    pub fn build_aql(self, collection: &str) -> AppResult<AqlBuilder> {
        // FOR i IN <collection>
        //      FILTER ..
        //      SORT ..
        //      LIMIT ..
        //      RETURN i
        let aql = AqlBuilder::new_for_in_collection(AQL_DOCUMENT_ID, collection);
        self.build_aql_using(aql)
    }

    pub fn build_aql_using(self, mut aql: AqlBuilder) -> AppResult<AqlBuilder> {
        // FOR i IN <collection>
        //      FILTER ..
        //      SORT ..
        //      LIMIT ..
        //      RETURN i

        // Filter part
        if let Some(filter_by) = &self.filter_by {
            let mut query = String::new();
            filter_by.build_aql(&mut query, &mut aql)?;

            aql.filter_step(query.into());
        }

        // Sort part
        if !self.sort_by.is_empty() {
            aql.sort_step(
                self.sort_by
                    .iter()
                    .map(|sorting| AqlSort {
                        expression: format!(
                            "{}.{}",
                            AQL_DOCUMENT_ID,
                            sorting.field.path_to_value()
                        )
                        .into(),
                        is_descending: sorting.descending,
                    })
                    .collect(),
            );
        }

        // Pagination
        aql.limit_step(AqlLimit {
            offset: Some(self.rows_per_page * self.page),
            count: self.rows_per_page,
        });
        aql.set_batch_size(Some(self.rows_per_page.min(100) as u32));
        aql.set_full_count(self.count_pages);
        aql.set_global_limit(self.rows_per_page);

        if let Some(fields) = self.fields_filter {
            let fields = fields.into_db_document();
            aql.return_step_with_fields(AQL_DOCUMENT_ID, &fields);
        } else {
            aql.return_step(AqlReturn::new_document());
        }

        Ok(aql)
    }
}
