use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use arangors::document::options::OverwriteMode;
use serde::Serialize;

pub use result::*;

use crate::database::traits::AQLMapping;
use crate::error::{AppError, AppResult, INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE};

pub mod aql_functions;
mod result;

pub const AQL_COLLECTION_ID: &str = "@collection";
pub const AQL_DOCUMENT_ID: &str = "i";
pub const AQL_OLD_ID: &str = "OLD";
pub const AQL_NEW_ID: &str = "NEW";

/// This is used to make an optimization when creating aql queries, not recreating param names
/// every time. Virtually it can handle 100 variables.
pub const AQL_VARIABLE_IDS: &[&str] = &[
    "@v0", "@v1", "@v2", "@v3", "@v4", "@v5", "@v6", "@v7", "@v8", "@v9", "@v10", "@v11", "@v12",
    "@v13", "@v14", "@v15", "@v16", "@v17", "@v18", "@v19", "@v20", "@v21", "@v22", "@v23", "@v24",
    "@v25", "@v26", "@v27", "@v28", "@v29", "@v30", "@v31", "@v32", "@v33", "@v34", "@v35", "@v36",
    "@v37", "@v38", "@v39", "@v40", "@v41", "@v42", "@v43", "@v44", "@v45", "@v46", "@v47", "@v48",
    "@v49", "@v50", "@v51", "@v52", "@v53", "@v54", "@v55", "@v56", "@v57", "@v58", "@v59", "@v60",
    "@v61", "@v62", "@v63", "@v64", "@v65", "@v66", "@v67", "@v68", "@v69", "@v70", "@v71", "@v72",
    "@v73", "@v74", "@v75", "@v76", "@v77", "@v78", "@v79", "@v80", "@v81", "@v82", "@v83", "@v84",
    "@v85", "@v86", "@v87", "@v88", "@v89", "@v90", "@v91", "@v92", "@v93", "@v94", "@v95", "@v96",
    "@v97", "@v98", "@v99",
];

/// This is used to make an optimization when creating aql queries.
pub const AQL_INLINE_IDS: &[&str] = &[
    "_0", "_1", "_2", "_3", "_4", "_5", "_6", "_7", "_8", "_9", "_10", "_11", "_12", "_13", "_14",
    "_15", "_16", "_17", "_18", "_19", "_20", "_21", "_22", "_23", "_24", "_25", "_26", "_27",
    "_28", "_29", "_30", "_31", "_32", "_33", "_34", "_35", "_36", "_37", "_38", "_39", "_40",
    "_41", "_42", "_43", "_44", "_45", "_46", "_47", "_48", "_49", "_50", "_51", "_52", "_53",
    "_54", "_55", "_56", "_57", "_58", "_59", "_60", "_61", "_62", "_63", "_64", "_65", "_66",
    "_67", "_68", "_69", "_70", "_71", "_72", "_73", "_74", "_75", "_76", "_77", "_78", "_79",
    "_80", "_81", "_82", "_83", "_84", "_85", "_86", "_87", "_88", "_89", "_90", "_91", "_92",
    "_93", "_94", "_95", "_96", "_97", "_98", "_99",
];

#[derive(Debug)]
pub struct AqlBuilder<'a> {
    alias: &'a str,
    next_var: usize,
    kind: AqlBuilderKind<'a>,
    batch_size: Option<u32>,
    full_count: bool,
    handle_write_conflicts: bool,
    global_limit: u64,
    steps: Vec<AqlKind<'a>>,
    pub(crate) vars: HashMap<&'static str, serde_json::Value>,
}

#[derive(Debug)]
enum AqlBuilderKind<'a> {
    Plain,
    Collection(&'a str),
    List(Vec<serde_json::Value>),
}

impl<'a> AqlBuilder<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new_simple() -> AqlBuilder<'a> {
        AqlBuilder {
            alias: "",
            next_var: 0,
            kind: AqlBuilderKind::Plain,
            batch_size: None,
            full_count: false,
            handle_write_conflicts: false,
            global_limit: 0,
            steps: Default::default(),
            vars: Default::default(),
        }
    }

    pub fn new_for_in_collection(alias: &'a str, collection: &'a str) -> AqlBuilder<'a> {
        AqlBuilder {
            alias,
            next_var: 0,
            kind: AqlBuilderKind::Collection(collection),
            batch_size: None,
            full_count: false,
            handle_write_conflicts: false,
            global_limit: 0,
            steps: Default::default(),
            vars: Default::default(),
        }
    }

    pub fn new_for_in_list<T: Serialize>(alias: &'a str, list: &[T]) -> AqlBuilder<'a> {
        let iter = list.iter();
        Self::new_for_in_iterator(alias, iter)
    }

    pub fn new_for_in_set<T: Serialize>(alias: &'a str, set: &HashSet<T>) -> AqlBuilder<'a> {
        let iter = set.iter();
        Self::new_for_in_iterator(alias, iter)
    }

    pub fn new_for_in_iterator<T: Serialize, I: Iterator<Item = T>>(
        alias: &'a str,
        iterator: I,
    ) -> AqlBuilder<'a> {
        AqlBuilder {
            alias,
            next_var: 0,
            kind: AqlBuilderKind::List(
                iterator.map(|v| serde_json::to_value(v).unwrap()).collect(),
            ),
            batch_size: None,
            full_count: false,
            handle_write_conflicts: false,
            global_limit: 0,
            steps: Default::default(),
            vars: Default::default(),
        }
    }

    // GETTERS ----------------------------------------------------------------

    pub fn batch_size(&self) -> Option<u32> {
        self.batch_size
    }

    pub fn full_count(&self) -> bool {
        self.full_count
    }

    pub fn handle_write_conflicts(&self) -> bool {
        self.handle_write_conflicts
    }

    pub fn global_limit(&self) -> u64 {
        self.global_limit
    }

    // SETTERS ----------------------------------------------------------------

    pub fn set_batch_size(&mut self, batch_size: Option<u32>) {
        self.batch_size = batch_size;
    }

    pub fn set_full_count(&mut self, full_count: bool) {
        self.full_count = full_count;
    }

    pub fn set_handle_write_conflicts(&mut self, handle_write_conflicts: bool) {
        self.handle_write_conflicts = handle_write_conflicts;
    }

    pub fn set_global_limit(&mut self, global_limit: u64) {
        self.global_limit = global_limit;
    }

    pub fn set_list<T: Serialize>(&mut self, list: &[T]) {
        let iter = list.iter();
        self.set_list_from_iterator(iter);
    }

    pub fn set_list_from_set<T: Serialize>(&mut self, list: &HashSet<T>) {
        let iter = list.iter();
        self.set_list_from_iterator(iter);
    }

    pub fn set_list_from_iterator<T: Serialize, I: Iterator<Item = T>>(&mut self, iterator: I) {
        match &mut self.kind {
            AqlBuilderKind::List(v) => {
                *v = iterator.map(|v| serde_json::to_value(v).unwrap()).collect();
            }
            _ => unreachable!("Incorrect aql kind"),
        }
    }

    // METHODS ----------------------------------------------------------------

    pub fn step(&mut self, kind: AqlKind<'a>) {
        self.steps.push(kind);
    }

    pub fn return_step(&mut self, step: AqlReturn<'a>) {
        self.steps.push(AqlKind::Return(step));
    }

    pub fn return_step_with_fields<T: AQLMapping>(&mut self, variable: &str, return_fields: &T) {
        // Include lets.
        let mut next_id = 0;
        return_fields.include_let_steps(self, variable, &mut next_id);

        // Generate mapping.
        let mut buffer = Vec::with_capacity(50);
        next_id = 0;
        return_fields.map_to_json(&mut buffer, variable, &mut next_id);

        let expression = unsafe { String::from_utf8_unchecked(buffer) };

        self.steps.push(AqlKind::Return(AqlReturn {
            distinct: false,
            expression: expression.into(),
        }));
    }

    pub fn filter_step(&mut self, step: Cow<'a, str>) {
        self.steps.push(AqlKind::Filter(step));
    }

    pub fn sort_step(&mut self, step: Vec<AqlSort<'a>>) {
        self.steps.push(AqlKind::Sort(step));
    }

    pub fn limit_step(&mut self, step: AqlLimit) {
        self.steps.push(AqlKind::Limit(step));
    }

    pub fn let_step(&mut self, step: AqlLet<'a>) {
        self.steps.push(AqlKind::Let(step));
    }

    pub fn remove_step(&mut self, step: AqlRemove<'a>) {
        self.steps.push(AqlKind::Remove(step));
    }

    pub fn update_step(&mut self, step: AqlUpdate<'a>) {
        self.steps.push(AqlKind::Update(step));
    }

    pub fn replace_step(&mut self, step: AqlReplace<'a>) {
        self.steps.push(AqlKind::Replace(step));
    }

    pub fn insert_step(&mut self, step: AqlInsert<'a>) {
        self.steps.push(AqlKind::Insert(step));
    }

    pub fn upsert_step(&mut self, step: AqlUpsert<'a>) {
        self.steps.push(AqlKind::Upsert(step));
    }

    pub fn collect_step(&mut self, step: AqlCollect<'a>) {
        self.steps.push(AqlKind::Collect(step));
    }

    pub fn add_variable<T: Serialize>(&mut self, value: &T) -> AppResult<&'static str> {
        self.add_variable_as_json(serde_json::to_value(value)?)
    }

    pub fn add_variable_as_json(&mut self, value: serde_json::Value) -> AppResult<&'static str> {
        let id = if let Some(v) = AQL_VARIABLE_IDS.get(self.next_var) {
            *v
        } else {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                INPUT_VALIDATION_TOO_MANY_ELEMENTS_ERROR_CODE,
            )
            .message(arcstr::literal!("Too much variables to perform the query")));
        };

        self.vars.insert(&id[1..], value);
        self.next_var += 1;
        Ok(id)
    }

    pub(crate) fn build_query(&self) -> String {
        let mut query = match &self.kind {
            AqlBuilderKind::Plain => String::new(),
            AqlBuilderKind::Collection(collection) => {
                format!("FOR {} IN {}", self.alias, collection)
            }
            AqlBuilderKind::List(list) => {
                format!(
                    "FOR {} IN {}",
                    self.alias,
                    serde_json::to_string(list).unwrap()
                )
            }
        };

        for step in &self.steps {
            step.build_query(&mut query, self);
        }

        query
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub enum AqlKind<'a> {
    Return(AqlReturn<'a>),
    Filter(Cow<'a, str>),
    Sort(Vec<AqlSort<'a>>),
    Limit(AqlLimit),
    Let(AqlLet<'a>),
    Remove(AqlRemove<'a>),
    Update(AqlUpdate<'a>),
    Replace(AqlReplace<'a>),
    Insert(AqlInsert<'a>),
    Upsert(AqlUpsert<'a>),
    Collect(AqlCollect<'a>),
}

impl<'a> AqlKind<'a> {
    // METHODS ----------------------------------------------------------------

    pub(crate) fn build_query(&self, query: &mut String, builder: &AqlBuilder<'a>) {
        match self {
            AqlKind::Return(v) => v.build_query(query),
            AqlKind::Filter(v) => {
                query.push_str(" FILTER ");
                query.push_str(v.as_ref());
            }
            AqlKind::Sort(v) => {
                if v.is_empty() {
                    return;
                }

                query.push_str(" SORT ");
                v[0].build_query(query, builder);

                for sort in &v[1..] {
                    query.push_str(", ");
                    sort.build_query(query, builder);
                }
            }
            AqlKind::Limit(v) => v.build_query(query, builder),
            AqlKind::Let(v) => v.build_query(query, builder),
            AqlKind::Remove(v) => v.build_query(query),
            AqlKind::Update(v) => v.build_query(query),
            AqlKind::Replace(v) => v.build_query(query),
            AqlKind::Insert(v) => v.build_query(query),
            AqlKind::Upsert(v) => v.build_query(query),
            AqlKind::Collect(v) => v.build_query(query),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlReturn<'a> {
    pub distinct: bool,
    pub expression: Cow<'a, str>,
}

impl<'a> AqlReturn<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new_document() -> AqlReturn<'a> {
        AqlReturn {
            distinct: false,
            expression: AQL_DOCUMENT_ID.into(),
        }
    }

    pub fn new_updated() -> AqlReturn<'a> {
        AqlReturn {
            distinct: false,
            expression: AQL_NEW_ID.into(),
        }
    }

    pub fn new_old() -> AqlReturn<'a> {
        AqlReturn {
            distinct: false,
            expression: AQL_OLD_ID.into(),
        }
    }

    pub fn new_expression(expression: Cow<'a, str>) -> AqlReturn<'a> {
        AqlReturn {
            distinct: false,
            expression,
        }
    }

    // METHODS ----------------------------------------------------------------

    pub(crate) fn build_query(&self, query: &mut String) {
        query.push_str(" RETURN ");

        if self.distinct {
            query.push_str("DISTINCT ");
        }

        query.push_str(self.expression.as_ref());
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlSort<'a> {
    pub is_descending: bool,
    pub expression: Cow<'a, str>,
}

impl<'a> AqlSort<'a> {
    // METHODS ----------------------------------------------------------------

    pub(crate) fn build_query(&self, query: &mut String, _builder: &AqlBuilder<'a>) {
        query.push_str(self.expression.as_ref());

        if self.is_descending {
            query.push_str(" DESC");
        } else {
            query.push_str(" ASC");
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AqlLimit {
    pub offset: Option<u64>,
    pub count: u64,
}

impl<'a> AqlLimit {
    // METHODS ----------------------------------------------------------------

    pub(crate) fn build_query(&self, query: &mut String, _builder: &AqlBuilder<'a>) {
        query.push_str(" LIMIT ");

        if let Some(offset) = self.offset {
            query.push_str(format!("{}, ", offset).as_str());
        }

        query.push_str(format!("{}", self.count).as_str());
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlLet<'a> {
    pub variable: &'static str,
    pub expression: AqlLetKind<'a>,
}

impl<'a> AqlLet<'a> {
    // METHODS ----------------------------------------------------------------

    pub(crate) fn build_query(&self, query: &mut String, _builder: &AqlBuilder<'a>) {
        query.push_str(" LET ");
        query.push_str(self.variable);
        query.push_str(" = ");

        match &self.expression {
            AqlLetKind::Expression(v) => {
                query.push_str(v.as_ref());
            }
            AqlLetKind::Aql(v) => {
                let content = v.build_query();

                assert!(v.vars.is_empty(), "Sub builders cannot have variables");

                query.push('(');
                query.push_str(content.as_str());
                query.push(')');
            }
        }
    }
}

#[derive(Debug)]
pub enum AqlLetKind<'a> {
    Expression(Cow<'a, str>),
    Aql(AqlBuilder<'a>),
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlRemove<'a> {
    pub variable: Cow<'a, str>,
    pub collection: &'a str,
    pub ignore_revs: bool,
    pub ignore_errors: bool,
}

impl<'a> AqlRemove<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(variable: Cow<'a, str>, collection: &'a str) -> Self {
        AqlRemove {
            variable,
            collection,
            ignore_revs: false,
            ignore_errors: true,
        }
    }

    pub fn new_document(collection: &'a str) -> Self {
        Self::new(AQL_DOCUMENT_ID.into(), collection)
    }

    // METHODS ----------------------------------------------------------------

    pub fn apply_ignore_revs(mut self, ignore_revs: bool) -> Self {
        self.ignore_revs = ignore_revs;
        self
    }

    pub fn apply_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = ignore_errors;
        self
    }

    pub(crate) fn build_query(&self, query: &mut String) {
        query.push_str(" REMOVE ");
        query.push_str(self.variable.as_ref());
        query.push_str(" IN ");
        query.push_str(self.collection);
        query.push_str(
            format!(
                " OPTIONS {{ ignoreRevs: {}, ignoreErrors: {} }}",
                self.ignore_revs, self.ignore_errors
            )
            .as_str(),
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlUpdate<'a> {
    pub variable: Cow<'a, str>,
    pub collection: &'a str,
    pub expression: Cow<'a, str>,
    pub ignore_revs: bool,
    pub keep_null: bool,
    pub merge_objects: bool,
    pub ignore_errors: bool,
}

impl<'a> AqlUpdate<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(variable: Cow<'a, str>, collection: &'a str, expression: Cow<'a, str>) -> Self {
        AqlUpdate {
            variable,
            collection,
            expression,
            ignore_revs: false,
            keep_null: false,
            merge_objects: true,
            ignore_errors: false,
        }
    }

    pub fn new_document(collection: &'a str, expression: Cow<'a, str>) -> Self {
        Self::new(AQL_DOCUMENT_ID.into(), collection, expression)
    }

    // METHODS ----------------------------------------------------------------

    pub fn apply_ignore_revs(mut self, ignore_revs: bool) -> Self {
        self.ignore_revs = ignore_revs;
        self
    }

    pub fn apply_keep_null(mut self, keep_null: bool) -> Self {
        self.keep_null = keep_null;
        self
    }

    pub fn apply_merge_objects(mut self, merge_objects: bool) -> Self {
        self.merge_objects = merge_objects;
        self
    }

    pub fn apply_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = ignore_errors;
        self
    }

    pub(crate) fn build_query(&self, query: &mut String) {
        query.push_str(" UPDATE ");
        query.push_str(self.variable.as_ref());
        query.push_str(" WITH ");
        query.push_str(self.expression.as_ref());
        query.push_str(" IN ");
        query.push_str(self.collection);
        query.push_str(
            format!(
                " OPTIONS {{ ignoreRevs: {}, keepNull: {}, mergeObjects: {}, ignoreErrors: {} }}",
                self.ignore_revs, self.keep_null, self.merge_objects, self.ignore_errors
            )
            .as_str(),
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlReplace<'a> {
    pub variable: Cow<'a, str>,
    pub collection: &'a str,
    pub expression: Cow<'a, str>,
    pub ignore_revs: bool,
    pub ignore_errors: bool,
}

impl<'a> AqlReplace<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(variable: Cow<'a, str>, collection: &'a str, expression: Cow<'a, str>) -> Self {
        AqlReplace {
            variable,
            collection,
            expression,
            ignore_revs: false,
            ignore_errors: false,
        }
    }

    pub fn new_document(collection: &'a str, expression: Cow<'a, str>) -> Self {
        Self::new(AQL_DOCUMENT_ID.into(), collection, expression)
    }

    // METHODS ----------------------------------------------------------------

    pub fn apply_ignore_revs(mut self, ignore_revs: bool) -> Self {
        self.ignore_revs = ignore_revs;
        self
    }

    pub fn apply_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = ignore_errors;
        self
    }

    pub(crate) fn build_query(&self, query: &mut String) {
        query.push_str(" REPLACE ");
        query.push_str(self.variable.as_ref());
        query.push_str(" WITH ");
        query.push_str(self.expression.as_ref());
        query.push_str(" IN ");
        query.push_str(self.collection);
        query.push_str(
            format!(
                " OPTIONS {{ ignoreRevs: {}, ignoreErrors: {} }}",
                self.ignore_revs, self.ignore_errors
            )
            .as_str(),
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlInsert<'a> {
    pub collection: &'a str,
    pub expression: Cow<'a, str>,
    pub overwrite_mode: OverwriteMode,
    pub keep_null: bool,
    pub merge_objects: bool,
    pub ignore_errors: bool,
}

impl<'a> AqlInsert<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(collection: &'a str, expression: Cow<'a, str>) -> Self {
        AqlInsert {
            collection,
            expression,
            overwrite_mode: OverwriteMode::Replace,
            keep_null: false,
            merge_objects: true,
            ignore_errors: false,
        }
    }

    pub fn new_document(collection: &'a str) -> Self {
        Self::new(collection, AQL_DOCUMENT_ID.into())
    }

    // METHODS ----------------------------------------------------------------

    pub fn apply_keep_null(mut self, keep_null: bool) -> Self {
        self.keep_null = keep_null;
        self
    }

    pub fn apply_merge_objects(mut self, merge_objects: bool) -> Self {
        self.merge_objects = merge_objects;
        self
    }

    pub fn apply_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = ignore_errors;
        self
    }

    pub(crate) fn build_query(&self, query: &mut String) {
        query.push_str(" INSERT ");
        query.push_str(self.expression.as_ref());
        query.push_str(" INTO ");
        query.push_str(self.collection);
        query.push_str(
            format!(
                " OPTIONS {{ overwriteMode: {}, keepNull: {}, mergeObjects: {}, ignoreErrors: {} }}",
                serde_json::to_string(&self.overwrite_mode).unwrap(),
                self.keep_null,
                self.merge_objects,
                self.ignore_errors,
            )
                .as_str(),
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlUpsert<'a> {
    pub collection: &'a str,
    pub lookup_expression: Cow<'a, str>,
    pub insert_expression: Cow<'a, str>,
    pub update_expression: Cow<'a, str>,
    pub is_update: bool,
    pub keep_null: bool,
    pub merge_objects: bool,
    pub ignore_errors: bool,
}

impl<'a> AqlUpsert<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new_replace(
        collection: &'a str,
        lookup_expression: Cow<'a, str>,
        insert_expression: Cow<'a, str>,
        replace_expression: Cow<'a, str>,
    ) -> Self {
        AqlUpsert {
            collection,
            lookup_expression,
            insert_expression,
            update_expression: replace_expression,
            is_update: false,
            keep_null: false,
            merge_objects: true,
            ignore_errors: false,
        }
    }

    pub fn new_update(
        collection: &'a str,
        lookup_expression: Cow<'a, str>,
        insert_expression: Cow<'a, str>,
        update_expression: Cow<'a, str>,
    ) -> Self {
        AqlUpsert {
            collection,
            lookup_expression,
            insert_expression,
            update_expression,
            is_update: true,
            keep_null: false,
            merge_objects: true,
            ignore_errors: false,
        }
    }

    // METHODS ----------------------------------------------------------------

    pub fn apply_keep_null(mut self, keep_null: bool) -> Self {
        self.keep_null = keep_null;
        self
    }

    pub fn apply_merge_objects(mut self, merge_objects: bool) -> Self {
        self.merge_objects = merge_objects;
        self
    }

    pub fn apply_ignore_errors(mut self, ignore_errors: bool) -> Self {
        self.ignore_errors = ignore_errors;
        self
    }

    pub(crate) fn build_query(&self, query: &mut String) {
        query.push_str(" UPSERT ");
        query.push_str(self.lookup_expression.as_ref());
        query.push_str(" INSERT ");
        query.push_str(self.insert_expression.as_ref());

        if self.is_update {
            query.push_str(" UPDATE ");
        } else {
            query.push_str(" REPLACE ");
        }

        query.push_str(self.update_expression.as_ref());
        query.push_str(" IN ");
        query.push_str(self.collection);
        query.push_str(
            format!(
                " OPTIONS {{ keepNull: {}, mergeObjects: {}, ignoreErrors: {} }}",
                self.keep_null, self.merge_objects, self.ignore_errors,
            )
            .as_str(),
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct AqlCollect<'a> {
    pub group_variable: Cow<'a, str>,
    pub expression: Option<Cow<'a, str>>,
    pub keep: &'a [&'a str],
    pub hash_method: bool,
}

impl<'a> AqlCollect<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new_count(group_variable: Cow<'a, str>) -> Self {
        AqlCollect {
            group_variable,
            expression: None,
            keep: &[],
            hash_method: false,
        }
    }

    pub fn new_collect_and_keep(
        expression: Cow<'a, str>,
        group_variable: Cow<'a, str>,
        keep: &'a [&'a str],
    ) -> Self {
        AqlCollect {
            group_variable,
            expression: Some(expression),
            keep,
            hash_method: false,
        }
    }

    // METHODS ----------------------------------------------------------------

    pub fn apply_hash_method(mut self, hash_method: bool) -> Self {
        self.hash_method = hash_method;
        self
    }

    pub(crate) fn build_query(&self, query: &mut String) {
        if let Some(expression) = &self.expression {
            query.push_str(" COLLECT ");
            query.push_str(expression);
            query.push_str(" INTO ");
            query.push_str(&self.group_variable);

            if !self.keep.is_empty() {
                query.push_str(" KEEP ");

                let mut iter = self.keep.iter();
                query.push_str(iter.next().unwrap());

                for var in iter {
                    query.push(',');
                    query.push_str(var);
                }
            }
        } else {
            query.push_str(" COLLECT WITH COUNT INTO ");
            query.push_str(self.group_variable.as_ref());

            if self.hash_method {
                query.push_str(" OPTIONS { method: \"hash\" }");
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub fn get_aql_inline_variable(index: usize) -> &'static str {
    AQL_INLINE_IDS[index]
}
