use crate::database::{AqlBuilder, NullableOption};
use crate::database::documents::ChallengeDBDocument;

#[test]
fn test_return_step_with_fields_empty() {
    let mut aql = AqlBuilder::new_simple();
    aql.return_step_with_fields("i", &ChallengeDBDocument::default());

    let query = aql.build_query();
    assert_eq!(" RETURN {_key:i._key,_rev:i._rev,}", query.as_str());
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[test]
fn test_return_step_with_fields_references_null() {
    let mut aql = AqlBuilder::new_simple();
    aql.return_step_with_fields(
        "i",
        &ChallengeDBDocument {
            name: NullableOption::Null,
            ..Default::default()
        },
    );

    let query = aql.build_query();
    println!("{}", query);
    assert_eq!(" RETURN {_key:i._key,_rev:i._rev,N:i.N,}", query.as_str());
}
