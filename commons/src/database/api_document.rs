use crate::database::types::DBUuid;

pub trait APIDocument {
    // GETTERS ----------------------------------------------------------------

    fn id(&self) -> &Option<DBUuid>;

    /// Whether all the fields are missing or not.
    fn is_all_missing(&self) -> bool;
}
