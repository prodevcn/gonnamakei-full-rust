use serde::{Deserialize, Deserializer, Serialize};

use crate::database::traits::{DBNormalize, DBNormalizeResult};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum NullableOption<T> {
    Missing,
    Null,
    Value(T),
}

impl<T> NullableOption<T> {
    // GETTERS ----------------------------------------------------------------

    pub fn is_missing(&self) -> bool {
        matches!(self, NullableOption::Missing)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, NullableOption::Null)
    }

    pub fn is_value(&self) -> bool {
        matches!(self, NullableOption::Value(_))
    }

    pub fn unwrap_as_ref(&self) -> &T {
        match self {
            NullableOption::Value(v) => v,
            _ => unreachable!("Cannot call 'unwrap_as_ref' without a value"),
        }
    }

    pub fn unwrap_as_mut_ref(&mut self) -> &mut T {
        match self {
            NullableOption::Value(v) => v,
            _ => unreachable!("Cannot call 'unwrap_as_mut_ref' without a value"),
        }
    }

    // METHODS ----------------------------------------------------------------

    pub fn unwrap(self) -> T {
        match self {
            NullableOption::Value(v) => v,
            _ => unreachable!("Cannot call 'unwrap' without a value"),
        }
    }

    pub fn unwrap_or(self, other: T) -> T {
        match self {
            NullableOption::Value(v) => v,
            _ => other,
        }
    }

    pub fn map<F, R>(self, mapper: F) -> NullableOption<R>
    where
        F: FnOnce(T) -> R,
    {
        match self {
            NullableOption::Missing => NullableOption::Missing,
            NullableOption::Null => NullableOption::Null,
            NullableOption::Value(v) => NullableOption::Value(mapper(v)),
        }
    }
}

impl<T: DBNormalize> DBNormalize for NullableOption<T> {
    // METHODS ----------------------------------------------------------------

    fn normalize(&mut self) -> DBNormalizeResult {
        let mut modified = false;

        if let NullableOption::Value(value) = self {
            match value.normalize() {
                DBNormalizeResult::NotModified => {}
                DBNormalizeResult::Modified => {
                    modified = true;
                }
                DBNormalizeResult::Removed => {
                    *self = NullableOption::Null;
                    modified = true;
                }
            }
        }

        if modified {
            DBNormalizeResult::Modified
        } else {
            DBNormalizeResult::NotModified
        }
    }
}

impl<T: Default> NullableOption<T> {
    // METHODS ----------------------------------------------------------------

    pub fn unwrap_or_default(self) -> T {
        match self {
            NullableOption::Value(v) => v,
            _ => Default::default(),
        }
    }
}

impl<T> Default for NullableOption<T> {
    fn default() -> Self {
        NullableOption::Missing
    }
}

impl<T> From<Option<T>> for NullableOption<T> {
    fn from(opt: Option<T>) -> NullableOption<T> {
        match opt {
            Some(v) => NullableOption::Value(v),
            None => NullableOption::Null,
        }
    }
}

impl<'de, T> Deserialize<'de> for NullableOption<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug)]
    struct TestStruct {
        #[serde(default)]
        field: NullableOption<i32>,
    }

    #[test]
    fn undefined() {
        let expected = NullableOption::Missing;
        let actual = serde_json::from_str::<TestStruct>("{}").unwrap().field;
        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let expected = NullableOption::Null;
        let actual = serde_json::from_str::<TestStruct>("{\"field\":null}")
            .unwrap()
            .field;
        assert_eq!(expected, actual);
    }

    #[test]
    fn value() {
        let n = 3;
        let expected = NullableOption::Value(n);
        let actual = serde_json::from_str::<TestStruct>("{\"field\":3}")
            .unwrap()
            .field;
        assert_eq!(expected, actual);
    }

    #[derive(Default, Deserialize, Debug)]
    #[serde(default)]
    struct TestStructDefaultOnTop {
        field: NullableOption<i32>,
    }

    #[test]
    fn undefined_default_on_top() {
        let expected = NullableOption::Missing;
        let actual = serde_json::from_str::<TestStructDefaultOnTop>("{}")
            .unwrap()
            .field;
        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_default_on_top() {
        let expected = NullableOption::Null;
        let actual = serde_json::from_str::<TestStructDefaultOnTop>("{\"field\":null}")
            .unwrap()
            .field;
        assert_eq!(expected, actual);
    }

    #[test]
    fn value_default_on_top() {
        let n = 3;
        let expected = NullableOption::Value(n);
        let actual = serde_json::from_str::<TestStructDefaultOnTop>("{\"field\":3}")
            .unwrap()
            .field;
        assert_eq!(expected, actual);
    }
}
